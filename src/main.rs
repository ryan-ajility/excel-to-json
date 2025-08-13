//! Cascade Fields Import Tool - Main entry point
//!
//! A high-performance command-line tool for importing Cascade Fields data from Excel
//! spreadsheets into a database-ready format. This tool is designed to be invoked
//! from PHP Laravel applications for processing hierarchical classification data.
//!
//! # Features
//!
//! - Excel file reading with formula evaluation (VLOOKUP support)
//! - Data validation and cleaning
//! - Duplicate detection based on composite keys
//! - Multiple output formats (JSON, CSV, PHP array)
//! - Comprehensive error handling and reporting
//! - Laravel-compatible timestamp generation
//!
//! # Usage
//!
//! ```bash
//! # Basic usage
//! import_cascade_fields data.xlsx
//!
//! # Specify output format
//! import_cascade_fields data.xlsx -o csv
//!
//! # Process specific sheet
//! import_cascade_fields data.xlsx -s "Custom Sheet"
//!
//! # Enable verbose logging
//! import_cascade_fields data.xlsx -v
//!
//! # Save output to file
//! import_cascade_fields data.xlsx -f output.json
//!
//! # Show summary only
//! import_cascade_fields data.xlsx --summary
//! ```

mod excel_reader;
mod models;
mod output;
mod processor;

use anyhow::{Context, Result};
use clap::Parser;
use models::{ErrorDetails, ProcessingMetadata, ProcessingResult};
use output::{OutputFormat, OutputFormatter};
use std::path::Path;
use tracing::{error, info};
use tracing_subscriber;

/// Command-line arguments for the import_cascade_fields tool.
///
/// This struct defines all available command-line options and arguments
/// for the tool, using the clap derive API for automatic parsing.
///
/// # Example
///
/// ```bash
/// # Process with all options
/// import_cascade_fields \
///   input.xlsx \
///   --sheet "Data Sheet" \
///   --output json \
///   --file results.json \
///   --verbose
/// ```
#[derive(Parser, Debug)]
#[command(name = "import_cascade_fields")]
#[command(about = "Import Cascade Fields data from Excel to database", long_about = None)]
struct Args {
    /// Path to the Excel file to import
    input_file: String,

    /// Sheet name to process
    #[arg(short = 's', long, default_value = "Cascade Fields")]
    sheet: String,

    /// Output format (json, csv, or php)
    #[arg(short = 'o', long, default_value = "json", help = "Output format: json, csv, or php (returns array of arrays for PHP consumption)")]
    output: String,

    /// Enable verbose logging
    #[arg(short = 'v', long)]
    verbose: bool,

    /// Output file path (if not specified, outputs to stdout)
    #[arg(short = 'f', long)]
    file: Option<String>,

    /// Show summary instead of full output
    #[arg(long)]
    summary: bool,
}

/// Main entry point for the import_cascade_fields tool.
///
/// Handles command-line argument parsing, logging initialization,
/// and orchestrates the overall processing flow.
///
/// # Process Flow
///
/// 1. Parse command-line arguments
/// 2. Initialize logging (to stderr)
/// 3. Execute main processing logic
/// 4. Handle and report any errors
///
/// # Exit Codes
///
/// - `0` - Success
/// - `1` - Error occurred during processing
fn main() {
    // Parse command-line arguments
    let args = Args::parse();

    // Initialize logging
    let log_level = if args.verbose {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .with_target(false)
        .with_writer(std::io::stderr) // Log to stderr so stdout is clean for output
        .init();

    // Run the main processing and handle any errors
    if let Err(e) = run(args) {
        error!("Fatal error: {:#}", e);
        std::process::exit(1);
    }
}

/// Main processing logic for the import tool.
///
/// Coordinates the entire import process from reading the Excel file
/// to outputting the formatted results.
///
/// # Arguments
///
/// * `args` - Parsed command-line arguments
///
/// # Returns
///
/// * `Ok(())` - Processing completed successfully
/// * `Err` - If any step in the process fails
///
/// # Process Steps
///
/// 1. Validate input file exists
/// 2. Parse output format
/// 3. Process Excel file
/// 4. Format results based on output format
/// 5. Write output to stdout or file
///
/// # Example Output (JSON)
///
/// ```json
/// {
///   "success": true,
///   "records": [
///     {
///       "main_label": "Category",
///       "main_value": "CAT001",
///       "main_description": "Main category",
///       // ... other fields
///     }
///   ],
///   "metadata": {
///     "total_rows_processed": 100,
///     "valid_records": 95,
///     "invalid_records": 5,
///     "processing_time_ms": 150
///   }
/// }
/// ```
fn run(args: Args) -> Result<()> {
    let start_time = std::time::Instant::now();
    
    info!("Starting import_cascade_fields");
    info!("Input file: {}", args.input_file);
    info!("Sheet: {}", args.sheet);
    
    // Parse output format
    let output_format: OutputFormat = args.output.parse()
        .map_err(|e: String| anyhow::anyhow!(e))?;
    
    // Check if input file exists
    let input_path = Path::new(&args.input_file);
    if !input_path.exists() {
        let result = ProcessingResult::error(
            format!("File not found: {}", args.input_file),
            Some(ErrorDetails {
                file: args.input_file.clone(),
                available_sheets: None,
                row_number: None,
                column: None,
            }),
            ProcessingMetadata {
                total_rows_processed: 0,
                valid_records: 0,
                invalid_records: 0,
                processing_time_ms: start_time.elapsed().as_millis(),
                warnings: None,
            },
        );
        
        let output = OutputFormatter::format_output(&result, output_format)?;
        OutputFormatter::write_to_stdout(&output)?;
        return Ok(());
    }
    
    // Process the Excel file
    let result = match process_excel_file(&args.input_file, &args.sheet) {
        Ok((records, metadata)) => {
            ProcessingResult::success(records, metadata)
        },
        Err(e) => {
            // Try to provide helpful error details
            let error_msg = format!("{:#}", e);
            
            // Check if this is a sheet not found error
            let details = if error_msg.contains("Sheet") && error_msg.contains("not found") {
                // Try to get available sheets
                let sheets = get_available_sheets(&args.input_file).ok();
                Some(ErrorDetails {
                    file: args.input_file.clone(),
                    available_sheets: sheets,
                    row_number: None,
                    column: None,
                })
            } else {
                Some(ErrorDetails {
                    file: args.input_file.clone(),
                    available_sheets: None,
                    row_number: None,
                    column: None,
                })
            };
            
            ProcessingResult::error(
                error_msg,
                details,
                ProcessingMetadata {
                    total_rows_processed: 0,
                    valid_records: 0,
                    invalid_records: 0,
                    processing_time_ms: start_time.elapsed().as_millis(),
                    warnings: None,
                },
            )
        }
    };
    
    // Format and output the result
    if args.summary {
        let summary = OutputFormatter::create_summary(&result);
        println!("{}", summary);
    } else {
        let output = OutputFormatter::format_output(&result, output_format)?;
        
        if let Some(file_path) = args.file {
            OutputFormatter::write_to_file(&output, &file_path)?;
            info!("Output written to {}", file_path);
        } else {
            OutputFormatter::write_to_stdout(&output)?;
        }
    }
    
    let total_time = start_time.elapsed();
    info!("Total execution time: {:?}", total_time);
    
    Ok(())
}

/// Processes an Excel file and extracts CascadeField records.
///
/// This function handles the core Excel processing workflow:
/// reading the file, extracting data with formula evaluation,
/// and transforming rows into validated CascadeField records.
///
/// # Arguments
///
/// * `file_path` - Path to the Excel file to process
/// * `sheet_name` - Name of the worksheet to read
///
/// # Returns
///
/// * `Ok((records, metadata))` - Successfully processed records and statistics
/// * `Err` - If file reading or processing fails
///
/// # Example
///
/// ```rust,no_run
/// # use import_cascade_fields::models::{CascadeField, ProcessingMetadata};
/// # fn process_excel_file(
/// #     file_path: &str,
/// #     sheet_name: &str,
/// # ) -> anyhow::Result<(Vec<CascadeField>, ProcessingMetadata)> {
/// #     Ok((vec![], ProcessingMetadata {
/// #         total_rows_processed: 0,
/// #         valid_records: 0,
/// #         invalid_records: 0,
/// #         processing_time_ms: 0,
/// #         warnings: None,
/// #     }))
/// # }
/// # fn main() -> anyhow::Result<()> {
/// let (records, metadata) = process_excel_file(
///     "data.xlsx",
///     "Cascade Fields"
/// )?;
///
/// println!("Processed {} records", records.len());
/// println!("Processing time: {}ms", metadata.processing_time_ms);
///
/// if let Some(warnings) = &metadata.warnings {
///     for warning in warnings {
///         println!("Warning: {}", warning);
///     }
/// }
/// # Ok(())
/// # }
/// ```
fn process_excel_file(
    file_path: &str,
    sheet_name: &str,
) -> Result<(Vec<models::CascadeField>, ProcessingMetadata)> {
    // Create Excel reader
    let mut reader = excel_reader::ExcelReader::new(file_path, sheet_name.to_string())
        .context("Failed to create Excel reader")?;
    
    // Read and process the Excel data
    let raw_rows = reader.read_with_formulas()
        .context("Failed to read Excel data")?;
    
    // Process the rows into CascadeField records
    let mut processor = processor::DataProcessor::new();
    let (records, metadata) = processor.process_rows(raw_rows)
        .context("Failed to process rows")?;
    
    Ok((records, metadata))
}

/// Retrieves the list of available sheet names from an Excel file.
///
/// This helper function is used primarily for error reporting when
/// a requested sheet is not found, providing users with the list of
/// available sheets they can choose from.
///
/// # Arguments
///
/// * `file_path` - Path to the Excel file
///
/// # Returns
///
/// * `Ok(Vec<String>)` - List of sheet names in the workbook
/// * `Err` - If the file cannot be opened or read
///
/// # Example
///
/// ```rust,no_run
/// # fn get_available_sheets(file_path: &str) -> anyhow::Result<Vec<String>> {
/// #     Ok(vec!["Sheet1".to_string()])
/// # }
/// # fn main() -> anyhow::Result<()> {
/// let sheets = get_available_sheets("data.xlsx")?;
///
/// // Check if desired sheet exists
/// if !sheets.contains(&"Cascade Fields".to_string()) {
///     eprintln!("Sheet 'Cascade Fields' not found.");
///     eprintln!("Available sheets: {:?}", sheets);
/// }
/// # Ok(())
/// # }
/// ```
fn get_available_sheets(file_path: &str) -> Result<Vec<String>> {
    let reader = excel_reader::ExcelReader::new(file_path, String::new())?;
    Ok(reader.get_sheet_names())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    // Helper function to get the test Excel file path
    fn get_test_excel_path() -> PathBuf {
        PathBuf::from("resources/Item Master Field Values.xlsx")
    }

    // Helper function to parse command line arguments for testing
    fn parse_test_args(args: Vec<&str>) -> Args {
        Args::parse_from(args)
    }

    #[test]
    fn test_basic_excel_processing() {
        let test_file = get_test_excel_path();
        assert!(test_file.exists(), "Test file should exist");

        // Test basic processing - this doesn't test the full CLI but tests the core function
        let result = process_excel_file(
            test_file.to_str().unwrap(),
            "Cascade Fields"
        );

        assert!(result.is_ok(), "Should process Excel file successfully");
        let (records, metadata) = result.unwrap();
        
        // Basic validation that we got some records
        assert!(metadata.total_rows_processed > 0);
        assert!(records.len() > 0 || metadata.invalid_records > 0);
    }

    #[test]
    fn test_cli_with_invalid_file() {
        let args = vec!["import_cascade_fields", "nonexistent.xlsx"];
        let parsed_args = parse_test_args(args);
        
        // Run the main logic
        let result = run(parsed_args);
        
        // Should complete without error (error is reported in the output)
        assert!(result.is_ok());
    }

    #[test]
    fn test_cli_with_different_output_formats() {
        let test_file = get_test_excel_path();
        
        // Test JSON output
        let args_json = vec![
            "import_cascade_fields",
            test_file.to_str().unwrap(),
            "-o", "json"
        ];
        let parsed_args = parse_test_args(args_json);
        let result = run(parsed_args);
        assert!(result.is_ok(), "JSON output should work");

        // Test CSV output
        let args_csv = vec![
            "import_cascade_fields",
            test_file.to_str().unwrap(),
            "-o", "csv"
        ];
        let parsed_args = parse_test_args(args_csv);
        let result = run(parsed_args);
        assert!(result.is_ok(), "CSV output should work");

        // Test PHP output
        let args_php = vec![
            "import_cascade_fields",
            test_file.to_str().unwrap(),
            "-o", "php"
        ];
        let parsed_args = parse_test_args(args_php);
        let result = run(parsed_args);
        assert!(result.is_ok(), "PHP output should work");
    }

    #[test]
    fn test_cli_with_file_output() {
        let test_file = get_test_excel_path();
        let temp_dir = TempDir::new().unwrap();
        let output_file = temp_dir.path().join("output.json");
        
        let args = vec![
            "import_cascade_fields",
            test_file.to_str().unwrap(),
            "-f", output_file.to_str().unwrap()
        ];
        let parsed_args = parse_test_args(args);
        let result = run(parsed_args);
        
        assert!(result.is_ok(), "Should write to file successfully");
        assert!(output_file.exists(), "Output file should be created");
        
        // Verify the file contains valid JSON
        let contents = fs::read_to_string(&output_file).unwrap();
        let json_result: serde_json::Value = serde_json::from_str(&contents)
            .expect("Output should be valid JSON");
        
        assert!(json_result.get("success").is_some());
        assert!(json_result.get("metadata").is_some());
    }

    #[test]
    fn test_cli_with_summary_flag() {
        let test_file = get_test_excel_path();
        
        let args = vec![
            "import_cascade_fields",
            test_file.to_str().unwrap(),
            "--summary"
        ];
        let parsed_args = parse_test_args(args);
        let result = run(parsed_args);
        
        assert!(result.is_ok(), "Summary output should work");
    }

    #[test]
    fn test_cli_with_custom_sheet() {
        let test_file = get_test_excel_path();
        
        // First, get available sheets to test with a valid one
        let sheets = get_available_sheets(test_file.to_str().unwrap())
            .expect("Should get sheet names");
        
        if let Some(first_sheet) = sheets.first() {
            let args = vec![
                "import_cascade_fields",
                test_file.to_str().unwrap(),
                "-s", first_sheet
            ];
            let parsed_args = parse_test_args(args);
            let result = run(parsed_args);
            
            assert!(result.is_ok(), "Should work with custom sheet name");
        }
    }

    #[test]
    fn test_cli_with_invalid_sheet() {
        let test_file = get_test_excel_path();
        
        let args = vec![
            "import_cascade_fields",
            test_file.to_str().unwrap(),
            "-s", "NonexistentSheet"
        ];
        let parsed_args = parse_test_args(args);
        let result = run(parsed_args);
        
        // Should complete without panicking (error is in the output)
        assert!(result.is_ok());
    }

    #[test]
    fn test_cli_with_verbose_flag() {
        let test_file = get_test_excel_path();
        
        let args = vec![
            "import_cascade_fields",
            test_file.to_str().unwrap(),
            "-v"
        ];
        let parsed_args = parse_test_args(args);
        
        // Just verify it doesn't panic with verbose flag
        let result = run(parsed_args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_available_sheets() {
        let test_file = get_test_excel_path();
        
        let sheets = get_available_sheets(test_file.to_str().unwrap());
        assert!(sheets.is_ok(), "Should get sheet names");
        
        let sheet_names = sheets.unwrap();
        assert!(!sheet_names.is_empty(), "Should have at least one sheet");
    }

    #[test]
    fn test_invalid_output_format() {
        let test_file = get_test_excel_path();
        
        let args = vec![
            "import_cascade_fields",
            test_file.to_str().unwrap(),
            "-o", "invalid_format"
        ];
        let parsed_args = parse_test_args(args);
        let result = run(parsed_args);
        
        // Should handle invalid format gracefully
        assert!(result.is_err(), "Should error on invalid output format");
    }
}
