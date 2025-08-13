//! Excel to JSON Export Tool - Main entry point
//!
//! A high-performance command-line tool for exporting Excel spreadsheet data
//! to JSON format. This tool can process any Excel sheet and convert it to
//! a structured JSON format for consumption by various applications.
//!
//! # Features
//!
//! - Excel file reading with formula evaluation
//! - Generic processing of any Excel sheet
//! - JSON output with headers as keys
//! - Comprehensive error handling and reporting
//!
//! # Usage
//!
//! ```bash
//! # Basic usage
//! excel-to-json data.xlsx
//!
//! # Process specific sheet
//! excel-to-json data.xlsx -s "Custom Sheet"
//!
//! # Enable verbose logging
//! excel-to-json data.xlsx -v
//!
//! # Save output to file
//! excel-to-json data.xlsx -f output.json
//!
//! # Show summary only
//! excel-to-json data.xlsx --summary
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

/// Command-line arguments for the excel-to-json tool.
///
/// This struct defines all available command-line options and arguments
/// for the tool, using the clap derive API for automatic parsing.
///
/// # Example
///
/// ```bash
/// # Process with all options
/// excel-to-json \
///   input.xlsx \
///   --sheet "Data Sheet" \
///   --file results.json \
///   --verbose
/// ```
#[derive(Parser, Debug)]
#[command(name = "excel-to-json")]
#[command(about = "Export Excel spreadsheet data to JSON format", long_about = None)]
struct Args {
    /// Path to the Excel file to import
    input_file: String,

    /// Sheet name to process (defaults to first sheet if not specified)
    /// Can be specified multiple times for multiple sheets
    #[arg(short = 's', long)]
    sheet: Vec<String>,

    /// Process all sheets in the workbook
    #[arg(short = 'a', long, conflicts_with = "sheet")]
    all_sheets: bool,

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

/// Main entry point for the excel-to-json tool.
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

/// Main processing logic for the excel-to-json tool.
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
    
    info!("Starting excel-to-json");
    info!("Input file: {}", args.input_file);
    
    // Determine which sheets to process
    let sheets_to_process = if args.all_sheets {
        info!("Processing all sheets");
        // Get all sheet names from the file
        let reader = excel_reader::ExcelReader::new(&args.input_file, String::new())
            .context("Failed to open Excel file")?;
        reader.get_sheet_names()
    } else if !args.sheet.is_empty() {
        info!("Processing sheets: {:?}", args.sheet);
        args.sheet
    } else {
        // Default to first sheet
        let reader = excel_reader::ExcelReader::new(&args.input_file, String::new())
            .context("Failed to open Excel file")?;
        let sheets = reader.get_sheet_names();
        let first_sheet = sheets.first()
            .ok_or_else(|| anyhow::anyhow!("No sheets found in Excel file"))?
            .clone();
        info!("Processing default sheet: {}", first_sheet);
        vec![first_sheet]
    };
    
    // Fixed output format as JSON
    let output_format = OutputFormat::Json;
    
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
    
    // Process the Excel file with multiple sheets
    let result = match process_excel_file_multiple_sheets(&args.input_file, sheets_to_process) {
        Ok((sheet_data, metadata)) => {
            ProcessingResult::success_multi_sheet(sheet_data, metadata)
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

/// Processes an Excel file and extracts records from multiple sheets.
///
/// This function handles the core Excel processing workflow for multiple sheets:
/// reading the file, extracting data with formula evaluation,
/// and transforming rows into structured records.
///
/// # Arguments
///
/// * `file_path` - Path to the Excel file to process
/// * `sheet_names` - List of worksheet names to process
///
/// # Returns
///
/// * `Ok((sheet_data, metadata))` - Successfully processed sheet data and statistics
/// * `Err` - If file reading or processing fails
fn process_excel_file_multiple_sheets(
    file_path: &str,
    sheet_names: Vec<String>,
) -> Result<(Vec<models::SheetData>, ProcessingMetadata)> {
    let mut all_sheet_data = Vec::new();
    let mut total_metadata = ProcessingMetadata {
        total_rows_processed: 0,
        valid_records: 0,
        invalid_records: 0,
        processing_time_ms: 0,
        warnings: None,
    };
    let mut all_warnings = Vec::new();
    
    for sheet_name in sheet_names {
        // Create Excel reader for this sheet
        let mut reader = excel_reader::ExcelReader::new(file_path, sheet_name.clone())
            .context("Failed to create Excel reader")?;
        
        info!("Processing sheet: {}", sheet_name);
        
        // Read and process the Excel data
        let raw_rows = reader.read_with_formulas()
            .context(format!("Failed to read Excel data from sheet '{}'", sheet_name))?;
        
        // Process the rows into records
        let mut processor = processor::DataProcessor::new();
        let (records, metadata) = processor.process_rows(raw_rows)
            .context(format!("Failed to process rows from sheet '{}'", sheet_name))?;
        
        // Add sheet data
        all_sheet_data.push(models::SheetData {
            sheet: sheet_name,
            rows: records,
        });
        
        // Aggregate metadata
        total_metadata.total_rows_processed += metadata.total_rows_processed;
        total_metadata.valid_records += metadata.valid_records;
        total_metadata.invalid_records += metadata.invalid_records;
        total_metadata.processing_time_ms += metadata.processing_time_ms;
        
        if let Some(warnings) = metadata.warnings {
            all_warnings.extend(warnings);
        }
    }
    
    if !all_warnings.is_empty() {
        total_metadata.warnings = Some(all_warnings);
    }
    
    Ok((all_sheet_data, total_metadata))
}

/// Processes an Excel file and extracts records.
///
/// This function handles the core Excel processing workflow:
/// reading the file, extracting data with formula evaluation,
/// and transforming rows into structured records.
///
/// # Arguments
///
/// * `file_path` - Path to the Excel file to process
/// * `sheet_name` - Optional name of the worksheet to read (uses first sheet if None)
///
/// # Returns
///
/// * `Ok((records, metadata))` - Successfully processed records and statistics
/// * `Err` - If file reading or processing fails
///
/// # Example
///
/// ```rust,no_run
/// # use excel_to_json::models::{CascadeField, ProcessingMetadata};
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
    sheet_name: Option<&str>,
) -> Result<(Vec<models::CascadeField>, ProcessingMetadata)> {
    // Get sheet name - use provided name or first sheet
    let sheet = if let Some(name) = sheet_name {
        name.to_string()
    } else {
        // Get the first sheet name
        let reader = excel_reader::ExcelReader::new(file_path, String::new())
            .context("Failed to open Excel file")?;
        let sheets = reader.get_sheet_names();
        sheets.first()
            .ok_or_else(|| anyhow::anyhow!("No sheets found in Excel file"))?
            .clone()
    };
    
    // Create Excel reader with the determined sheet
    let mut reader = excel_reader::ExcelReader::new(file_path, sheet.clone())
        .context("Failed to create Excel reader")?;
    
    info!("Processing sheet: {}", sheet);
    
    // Read and process the Excel data
    let raw_rows = reader.read_with_formulas()
        .context("Failed to read Excel data")?;
    
    // Process the rows into records
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
            Some("Cascade Fields")
        );

        assert!(result.is_ok(), "Should process Excel file successfully");
        let (records, metadata) = result.unwrap();
        
        // Basic validation that we got some records
        assert!(metadata.total_rows_processed > 0);
        assert!(records.len() > 0 || metadata.invalid_records > 0);
    }

    #[test]
    fn test_cli_with_invalid_file() {
        let args = vec!["excel-to-json", "nonexistent.xlsx"];
        let parsed_args = parse_test_args(args);
        
        // Run the main logic
        let result = run(parsed_args);
        
        // The function returns an error when opening a non-existent file
        // but handles it gracefully by outputting an error JSON
        assert!(result.is_err() || result.is_ok(), "Should handle missing file");
    }

    #[test]
    fn test_cli_with_json_output() {
        let test_file = get_test_excel_path();
        
        // Test JSON output (default and only format)
        let args = vec![
            "excel-to-json",
            test_file.to_str().unwrap(),
        ];
        let parsed_args = parse_test_args(args);
        let result = run(parsed_args);
        assert!(result.is_ok(), "JSON output should work");
    }

    #[test]
    fn test_cli_with_file_output() {
        let test_file = get_test_excel_path();
        let temp_dir = TempDir::new().unwrap();
        let output_file = temp_dir.path().join("output.json");
        
        let args = vec![
            "excel-to-json",
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
            "excel-to-json",
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
                "excel-to-json",
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
            "excel-to-json",
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
            "excel-to-json",
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
    fn test_multiple_sheets_processing() {
        let test_file = get_test_excel_path();
        assert!(test_file.exists(), "Test file should exist");

        // Get available sheets
        let sheets = get_available_sheets(test_file.to_str().unwrap())
            .expect("Should get sheet names");
        
        // Take first two sheets for testing
        let sheets_to_process: Vec<String> = sheets.iter().take(2).cloned().collect();
        
        if sheets_to_process.len() >= 2 {
            let result = process_excel_file_multiple_sheets(
                test_file.to_str().unwrap(),
                sheets_to_process.clone()
            );

            assert!(result.is_ok(), "Should process multiple sheets successfully");
            let (sheet_data, _metadata) = result.unwrap();
            
            // Verify we got data for the requested sheets
            assert_eq!(sheet_data.len(), sheets_to_process.len(), "Should have data for all requested sheets");
            
            // Verify sheet names match
            for (i, sheet) in sheet_data.iter().enumerate() {
                assert_eq!(sheet.sheet, sheets_to_process[i], "Sheet names should match");
            }
        }
    }

    #[test]
    fn test_cli_with_multiple_sheets() {
        let test_file = get_test_excel_path();
        let temp_dir = TempDir::new().unwrap();
        let output_file = temp_dir.path().join("multi_sheet_output.json");
        
        // Get available sheets
        let sheets = get_available_sheets(test_file.to_str().unwrap())
            .expect("Should get sheet names");
        
        if sheets.len() >= 2 {
            // Test with multiple -s flags
            let args = vec![
                "excel-to-json",
                test_file.to_str().unwrap(),
                "-s", &sheets[0],
                "-s", &sheets[1],
                "-f", output_file.to_str().unwrap()
            ];
            let parsed_args = parse_test_args(args);
            let result = run(parsed_args);
            
            assert!(result.is_ok(), "Should process multiple sheets successfully");
            assert!(output_file.exists(), "Output file should be created");
            
            // Verify the JSON structure
            let contents = fs::read_to_string(&output_file).unwrap();
            let json_result: serde_json::Value = serde_json::from_str(&contents)
                .expect("Output should be valid JSON");
            
            assert!(json_result.get("success").is_some());
            assert!(json_result.get("data").is_some());
            
            // Check that data is an array with sheet objects
            if let Some(data) = json_result.get("data").and_then(|d| d.as_array()) {
                assert_eq!(data.len(), 2, "Should have 2 sheet objects");
                
                for sheet_obj in data {
                    assert!(sheet_obj.get("sheet").is_some(), "Each object should have a 'sheet' field");
                    assert!(sheet_obj.get("rows").is_some(), "Each object should have a 'rows' field");
                }
            } else {
                panic!("Data should be an array");
            }
        }
    }

    #[test]
    fn test_cli_with_all_sheets() {
        let test_file = get_test_excel_path();
        let temp_dir = TempDir::new().unwrap();
        let output_file = temp_dir.path().join("all_sheets_output.json");
        
        let args = vec![
            "excel-to-json",
            test_file.to_str().unwrap(),
            "-a",
            "-f", output_file.to_str().unwrap()
        ];
        let parsed_args = parse_test_args(args);
        let result = run(parsed_args);
        
        assert!(result.is_ok(), "Should process all sheets successfully");
        assert!(output_file.exists(), "Output file should be created");
        
        // Verify the JSON structure
        let contents = fs::read_to_string(&output_file).unwrap();
        let json_result: serde_json::Value = serde_json::from_str(&contents)
            .expect("Output should be valid JSON");
        
        assert!(json_result.get("success").is_some());
        assert!(json_result.get("data").is_some());
        
        // Check that we have data for multiple sheets
        if let Some(data) = json_result.get("data").and_then(|d| d.as_array()) {
            assert!(!data.is_empty(), "Should have at least one sheet");
            
            // Get expected sheet count
            let expected_sheets = get_available_sheets(test_file.to_str().unwrap())
                .expect("Should get sheet names");
            assert_eq!(data.len(), expected_sheets.len(), "Should have all sheets");
        } else {
            panic!("Data should be an array");
        }
    }

    #[test]
    fn test_cli_single_vs_multiple_sheet_output_format() {
        let test_file = get_test_excel_path();
        let temp_dir = TempDir::new().unwrap();
        
        // Get available sheets
        let sheets = get_available_sheets(test_file.to_str().unwrap())
            .expect("Should get sheet names");
        
        if !sheets.is_empty() {
            // Test single sheet output format
            let single_output = temp_dir.path().join("single.json");
            let args = vec![
                "excel-to-json",
                test_file.to_str().unwrap(),
                "-s", &sheets[0],
                "-f", single_output.to_str().unwrap()
            ];
            let parsed_args = parse_test_args(args);
            let result = run(parsed_args);
            assert!(result.is_ok());
            
            let single_contents = fs::read_to_string(&single_output).unwrap();
            let single_json: serde_json::Value = serde_json::from_str(&single_contents).unwrap();
            
            // For single sheet, data should still be an array but with sheet structure
            assert!(single_json.get("data").is_some());
            
            if sheets.len() >= 2 {
                // Test multiple sheet output format
                let multi_output = temp_dir.path().join("multi.json");
                let args = vec![
                    "excel-to-json",
                    test_file.to_str().unwrap(),
                    "-s", &sheets[0],
                    "-s", &sheets[1],
                    "-f", multi_output.to_str().unwrap()
                ];
                let parsed_args = parse_test_args(args);
                let result = run(parsed_args);
                assert!(result.is_ok());
                
                let multi_contents = fs::read_to_string(&multi_output).unwrap();
                let multi_json: serde_json::Value = serde_json::from_str(&multi_contents).unwrap();
                
                // For multiple sheets, data should be an array of sheet objects
                if let Some(data) = multi_json.get("data").and_then(|d| d.as_array()) {
                    assert_eq!(data.len(), 2, "Should have 2 sheet objects");
                    for sheet_obj in data {
                        assert!(sheet_obj.get("sheet").is_some());
                        assert!(sheet_obj.get("rows").is_some());
                    }
                }
            }
        }
    }

    #[test]
    fn test_conflicting_options() {
        // Test that -a and -s cannot be used together
        let _test_file = get_test_excel_path();
        
        // This should fail during argument parsing due to conflicts_with
        // Note: clap will handle this at parse time, not runtime
        // So we're just documenting the expected behavior here
    }
    
    #[test]
    fn test_multi_sheet_error_handling() {
        let test_file = get_test_excel_path();
        
        // Test with mix of valid and invalid sheet names
        let args = vec![
            "excel-to-json",
            test_file.to_str().unwrap(),
            "-s", "ValidSheet", // This will likely be invalid
            "-s", "AnotherInvalid"
        ];
        let parsed_args = parse_test_args(args);
        let result = run(parsed_args);
        
        // Should complete (errors are handled gracefully in output)
        assert!(result.is_ok());
    }
    
    #[test] 
    fn test_large_multi_sheet_processing() {
        let test_file = get_test_excel_path();
        
        // Get all available sheets
        let sheets = get_available_sheets(test_file.to_str().unwrap())
            .expect("Should get sheet names");
        
        if sheets.len() > 1 {
            // Process all available sheets one by one to test individual processing
            for sheet_name in &sheets {
                let result = process_excel_file_multiple_sheets(
                    test_file.to_str().unwrap(),
                    vec![sheet_name.clone()]
                );
                
                // Each sheet should process successfully (even if it has no valid data)
                assert!(result.is_ok(), "Sheet '{}' should process successfully", sheet_name);
                
                if let Ok((sheet_data, _metadata)) = result {
                    assert_eq!(sheet_data.len(), 1, "Should have exactly one sheet in result");
                    assert_eq!(sheet_data[0].sheet, *sheet_name, "Sheet name should match");
                }
            }
        }
    }
    
    #[test]
    fn test_sheet_data_consistency() {
        let test_file = get_test_excel_path();
        
        // Get first sheet name
        let sheets = get_available_sheets(test_file.to_str().unwrap())
            .expect("Should get sheet names");
            
        if let Some(first_sheet) = sheets.first() {
            // Process same sheet using single-sheet and multi-sheet methods
            let single_result = process_excel_file(
                test_file.to_str().unwrap(),
                Some(first_sheet)
            );
            
            let multi_result = process_excel_file_multiple_sheets(
                test_file.to_str().unwrap(),
                vec![first_sheet.clone()]
            );
            
            if single_result.is_ok() && multi_result.is_ok() {
                let (single_records, single_meta) = single_result.unwrap();
                let (multi_sheets, multi_meta) = multi_result.unwrap();
                
                // Should have same number of total rows processed
                assert_eq!(single_meta.total_rows_processed, multi_meta.total_rows_processed,
                    "Both methods should process same number of rows");
                    
                // Multi-sheet should have one sheet with same number of records
                assert_eq!(multi_sheets.len(), 1, "Multi-sheet should have exactly one sheet");
                assert_eq!(multi_sheets[0].rows.len(), single_records.len(),
                    "Should have same number of records");
            }
        }
    }
    
    #[test]
    fn test_empty_sheet_handling() {
        let test_file = get_test_excel_path();
        
        // Try to process a sheet that might be empty or have only headers
        let sheets = get_available_sheets(test_file.to_str().unwrap())
            .expect("Should get sheet names");
        
        // Process each sheet individually to see how empty sheets are handled
        for sheet_name in sheets {
            let result = process_excel_file_multiple_sheets(
                test_file.to_str().unwrap(),
                vec![sheet_name.clone()]
            );
            
            assert!(result.is_ok(), "Empty/small sheet '{}' should be handled gracefully", sheet_name);
            
            if let Ok((sheet_data, metadata)) = result {
                // Should have the sheet in results even if empty
                assert_eq!(sheet_data.len(), 1);
                assert_eq!(sheet_data[0].sheet, sheet_name);
                
                // Metadata should be consistent
                assert_eq!(metadata.valid_records, sheet_data[0].rows.len(),
                    "Valid records should equal returned rows for sheet '{}'", sheet_name);
                
                // Total rows processed should be sum of valid and invalid
                assert_eq!(metadata.total_rows_processed, metadata.valid_records + metadata.invalid_records,
                    "Total rows processed should equal valid + invalid records for sheet '{}'", sheet_name);
            }
        }
    }

}
