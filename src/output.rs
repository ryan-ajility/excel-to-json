//! Output formatting module for the Cascade Fields import tool.
//!
//! This module handles the formatting and output of processed data in various
//! formats suitable for consumption by different systems, particularly PHP/Laravel
//! applications.
//!
//! # Supported Formats
//!
//! - **JSON** - Standard JSON format for API responses
//! - **CSV** - Comma-separated values for spreadsheet applications
//! - **PHP Array** - JSON structure optimized for PHP consumption
//!
//! # Example
//!
//! ```rust
//! use import_cascade_fields::output::{OutputFormatter, OutputFormat};
//! use import_cascade_fields::models::{ProcessingResult, ProcessingMetadata};
//!
//! # fn main() -> anyhow::Result<()> {
//! let result = ProcessingResult::success(
//!     vec![],  // CascadeField records
//!     ProcessingMetadata {
//!         total_rows_processed: 100,
//!         valid_records: 95,
//!         invalid_records: 5,
//!         processing_time_ms: 150,
//!         warnings: None,
//!     },
//! );
//!
//! // Format as JSON
//! let json_output = OutputFormatter::format_output(&result, OutputFormat::Json)?;
//! println!("JSON: {}", json_output);
//!
//! // Format as CSV
//! let csv_output = OutputFormatter::format_output(&result, OutputFormat::Csv)?;
//! println!("CSV: {}", csv_output);
//! # Ok(())
//! # }
//! ```

use crate::models::{CascadeField, ProcessingResult};
use anyhow::Result;
use serde_json::{self, json, Value};
use std::io::Write;
use tracing::info;

/// Output format options for processed data.
///
/// Determines how the processing results will be formatted
/// for output to different consumers.
///
/// # Example
///
/// ```rust
/// use import_cascade_fields::output::OutputFormat;
/// use std::str::FromStr;
///
/// // Parse from string
/// let format = OutputFormat::from_str("json").unwrap();
/// matches!(format, OutputFormat::Json);
///
/// // Parse case-insensitive
/// let format = OutputFormat::from_str("CSV").unwrap();
/// matches!(format, OutputFormat::Csv);
///
/// // Parse PHP format variations
/// let format = OutputFormat::from_str("php").unwrap();
/// matches!(format, OutputFormat::PhpArray);
/// ```
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Csv,
    PhpArray,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;
    
    /// Parses an OutputFormat from a string.
    ///
    /// Accepts various format names (case-insensitive):
    /// - "json" → Json
    /// - "csv" → Csv  
    /// - "php", "phparray", "php-array" → PhpArray
    ///
    /// # Example
    ///
    /// ```rust
    /// use import_cascade_fields::output::OutputFormat;
    /// use std::str::FromStr;
    ///
    /// assert!(matches!(OutputFormat::from_str("json"), Ok(OutputFormat::Json)));
    /// assert!(matches!(OutputFormat::from_str("CSV"), Ok(OutputFormat::Csv)));
    /// assert!(matches!(OutputFormat::from_str("php-array"), Ok(OutputFormat::PhpArray)));
    /// assert!(OutputFormat::from_str("invalid").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "csv" => Ok(OutputFormat::Csv),
            "php" | "phparray" | "php-array" => Ok(OutputFormat::PhpArray),
            _ => Err(format!("Unknown output format: {}", s)),
        }
    }
}

/// Handles output formatting for different targets.
///
/// The `OutputFormatter` provides static methods to format processing results
/// into various output formats and write them to different destinations.
///
/// # Example
///
/// ```rust
/// use import_cascade_fields::output::{OutputFormatter, OutputFormat};
/// use import_cascade_fields::models::{ProcessingResult, ProcessingMetadata, CascadeField};
/// use std::io::Write;
///
/// # fn main() -> anyhow::Result<()> {
/// // Create sample result
/// let result = ProcessingResult::success(
///     vec![
///         // CascadeField records
///     ],
///     ProcessingMetadata {
///         total_rows_processed: 10,
///         valid_records: 10,
///         invalid_records: 0,
///         processing_time_ms: 50,
///         warnings: None,
///     },
/// );
///
/// // Format and output
/// let output = OutputFormatter::format_output(&result, OutputFormat::Json)?;
/// OutputFormatter::write_to_stdout(&output)?;
///
/// // Or write to file
/// OutputFormatter::write_to_file(&output, "output.json")?;
///
/// // Create a summary report
/// let summary = OutputFormatter::create_summary(&result);
/// println!("{}", summary);
/// # Ok(())
/// # }
/// ```
pub struct OutputFormatter;

impl OutputFormatter {
    /// Formats the processing result according to the specified format.
    ///
    /// Routes the result to the appropriate formatter based on the
    /// requested output format.
    ///
    /// # Arguments
    ///
    /// * `result` - The processing result to format
    /// * `format` - The desired output format
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - Formatted output as a string
    /// * `Err` - If formatting fails
    ///
    /// # Example
    ///
    /// ```rust
    /// use import_cascade_fields::output::{OutputFormatter, OutputFormat};
    /// use import_cascade_fields::models::{ProcessingResult, ProcessingMetadata};
    ///
    /// # fn main() -> anyhow::Result<()> {
    /// let result = ProcessingResult::success(
    ///     vec![],
    ///     ProcessingMetadata {
    ///         total_rows_processed: 5,
    ///         valid_records: 5,
    ///         invalid_records: 0,
    ///         processing_time_ms: 25,
    ///         warnings: None,
    ///     },
    /// );
    ///
    /// // Format as JSON
    /// let json = OutputFormatter::format_output(&result, OutputFormat::Json)?;
    /// assert!(json.contains("success"));
    ///
    /// // Format as CSV
    /// let csv = OutputFormatter::format_output(&result, OutputFormat::Csv)?;
    /// assert!(csv.contains("main_label,main_value"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn format_output(result: &ProcessingResult, format: OutputFormat) -> Result<String> {
        match format {
            OutputFormat::Json => Self::format_json(result),
            OutputFormat::Csv => Self::format_csv(result),
            OutputFormat::PhpArray => Self::format_php_array(result),
        }
    }
    
    /// Formats the result as JSON for PHP consumption.
    ///
    /// Creates a standard JSON representation of the processing result,
    /// including all records, metadata, and error information.
    ///
    /// # Arguments
    ///
    /// * `result` - The processing result to format
    ///
    /// # Returns
    ///
    /// Pretty-printed JSON string
    ///
    /// # JSON Structure
    ///
    /// ```json
    /// {
    ///   "success": true,
    ///   "records": [...],
    ///   "metadata": {
    ///     "total_rows_processed": 100,
    ///     "valid_records": 95,
    ///     "invalid_records": 5,
    ///     "processing_time_ms": 150
    ///   }
    /// }
    /// ```
    fn format_json(result: &ProcessingResult) -> Result<String> {
        let json = serde_json::to_string_pretty(result)?;
        info!("Formatted output as JSON ({} bytes)", json.len());
        Ok(json)
    }
    
    /// Formats the result as CSV.
    ///
    /// Creates a CSV representation of the CascadeField records.
    /// Error results produce a simple status CSV.
    ///
    /// # Arguments
    ///
    /// * `result` - The processing result to format
    ///
    /// # Returns
    ///
    /// CSV-formatted string with headers and data rows
    ///
    /// # CSV Format
    ///
    /// ```text
    /// main_label,main_value,main_description,sub_label,sub_value,...
    /// "Category A","CAT001","Description",...
    /// ```
    ///
    /// # Example
    ///
    /// ```rust
    /// use import_cascade_fields::models::{CascadeField, ProcessingResult, ProcessingMetadata};
    /// use import_cascade_fields::output::{OutputFormatter, OutputFormat};
    ///
    /// # fn main() -> anyhow::Result<()> {
    /// let records = vec![
    ///     CascadeField::from_row(vec![
    ///         Some("Label".to_string()),
    ///         Some("VAL001".to_string()),
    ///         None, None, None, None, None, None, None, None, None, None,
    ///     ]).unwrap(),
    /// ];
    ///
    /// let result = ProcessingResult::success(
    ///     records,
    ///     ProcessingMetadata {
    ///         total_rows_processed: 1,
    ///         valid_records: 1,
    ///         invalid_records: 0,
    ///         processing_time_ms: 10,
    ///         warnings: None,
    ///     },
    /// );
    ///
    /// let csv = OutputFormatter::format_output(&result, OutputFormat::Csv)?;
    /// assert!(csv.contains("main_label,main_value"));
    /// assert!(csv.contains("Label,VAL001"));
    /// # Ok(())
    /// # }
    /// ```
    fn format_csv(result: &ProcessingResult) -> Result<String> {
        if !result.success {
            // For errors, return a simple CSV with error information
            return Ok(format!("status,error\nfailed,\"{}\"", 
                result.error.as_ref().unwrap_or(&"Unknown error".to_string())));
        }
        
        let mut csv_output = String::new();
        
        // Write CSV header
        csv_output.push_str("main_label,main_value,main_description,");
        csv_output.push_str("sub_label,sub_value,sub_description,");
        csv_output.push_str("major_label,major_value,major_description,");
        csv_output.push_str("minor_label,minor_value,minor_description\n");
        
        // Write records
        if let Some(records) = &result.records {
            for record in records {
                csv_output.push_str(&Self::format_csv_row(record));
                csv_output.push('\n');
            }
        }
        
        info!("Formatted output as CSV ({} bytes)", csv_output.len());
        Ok(csv_output)
    }
    
    /// Formats a single CascadeField as a CSV row.
    ///
    /// Converts all fields to CSV format with proper escaping.
    ///
    /// # Arguments
    ///
    /// * `field` - The CascadeField to format
    ///
    /// # Returns
    ///
    /// CSV-formatted row as a string
    fn format_csv_row(field: &CascadeField) -> String {
        format!(
            "{},{},{},{},{},{},{},{},{},{},{},{}",
            Self::escape_csv(&field.main_label),
            Self::escape_csv(&field.main_value),
            Self::escape_csv(&field.main_description),
            Self::escape_csv(&field.sub_label),
            Self::escape_csv(&field.sub_value),
            Self::escape_csv(&field.sub_description),
            Self::escape_csv(&field.major_label),
            Self::escape_csv(&field.major_value),
            Self::escape_csv(&field.major_description),
            Self::escape_csv(&field.minor_label),
            Self::escape_csv(&field.minor_value),
            Self::escape_csv(&field.minor_description)
        )
    }
    
    /// Escapes a CSV field value.
    ///
    /// Properly escapes strings for CSV format:
    /// - Quotes strings containing commas, quotes, or newlines
    /// - Escapes internal quotes by doubling them
    /// - Returns empty string for None values
    ///
    /// # Arguments
    ///
    /// * `value` - Optional string to escape
    ///
    /// # Returns
    ///
    /// Properly escaped CSV field value
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn escape_csv(value: &Option<String>) -> String {
    /// #     match value {
    /// #         Some(s) => {
    /// #             if s.contains(',') || s.contains('"') || s.contains('\n') {
    /// #                 format!("\"{}\"", s.replace('"', "\"\""))
    /// #             } else {
    /// #                 s.clone()
    /// #             }
    /// #         },
    /// #         None => String::new(),
    /// #     }
    /// # }
    /// // Simple value
    /// assert_eq!(escape_csv(&Some("test".to_string())), "test");
    ///
    /// // Value with comma
    /// assert_eq!(escape_csv(&Some("test,value".to_string())), "\"test,value\"");
    ///
    /// // Value with quotes
    /// assert_eq!(escape_csv(&Some("test\"value".to_string())), "\"test\"\"value\"");
    ///
    /// // None value
    /// assert_eq!(escape_csv(&None), "");
    /// ```
    fn escape_csv(value: &Option<String>) -> String {
        match value {
            Some(s) => {
                if s.contains(',') || s.contains('"') || s.contains('\n') {
                    format!("\"{}\"", s.replace('"', "\"\""))
                } else {
                    s.clone()
                }
            },
            None => String::new(),
        }
    }
    
    /// Formats the result as a PHP-compatible array of arrays.
    ///
    /// Creates a JSON structure optimized for PHP/Laravel consumption,
    /// with all None values converted to empty strings.
    ///
    /// # Arguments
    ///
    /// * `result` - The processing result to format
    ///
    /// # Returns
    ///
    /// JSON string formatted for PHP consumption
    ///
    /// # PHP Array Structure
    ///
    /// ```json
    /// {
    ///   "success": true,
    ///   "data": [
    ///     {
    ///       "main_label": "Category",
    ///       "main_value": "CAT001",
    ///       "main_description": "",
    ///       // ... all fields with empty strings for null
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
    ///
    /// # Example
    ///
    /// ```rust
    /// use import_cascade_fields::models::{CascadeField, ProcessingResult, ProcessingMetadata};
    /// use import_cascade_fields::output::{OutputFormatter, OutputFormat};
    ///
    /// # fn main() -> anyhow::Result<()> {
    /// let records = vec![
    ///     CascadeField::from_row(vec![
    ///         Some("Label".to_string()),
    ///         Some("VAL001".to_string()),
    ///         None,  // Will become empty string in PHP
    ///         None, None, None, None, None, None, None, None, None,
    ///     ]).unwrap(),
    /// ];
    ///
    /// let result = ProcessingResult::success(
    ///     records,
    ///     ProcessingMetadata {
    ///         total_rows_processed: 1,
    ///         valid_records: 1,
    ///         invalid_records: 0,
    ///         processing_time_ms: 10,
    ///         warnings: None,
    ///     },
    /// );
    ///
    /// let php_output = OutputFormatter::format_output(&result, OutputFormat::PhpArray)?;
    /// assert!(php_output.contains("success"));
    /// assert!(php_output.contains("data"));
    /// assert!(php_output.contains("main_description"));  // None becomes ""
    /// # Ok(())
    /// # }
    /// ```
    fn format_php_array(result: &ProcessingResult) -> Result<String> {
        if !result.success {
            // For errors, return an error structure that PHP can handle
            let error_response = json!({
                "success": false,
                "error": result.error.as_ref().unwrap_or(&"Unknown error".to_string()),
                "data": []
            });
            return Ok(serde_json::to_string_pretty(&error_response)?);
        }
        
        // Convert records to array of PHP-compatible associative arrays
        let php_array: Vec<Value> = result.records
            .as_ref()
            .map(|records| {
                records.iter()
                    .map(|record| record.to_php_array())
                    .collect()
            })
            .unwrap_or_else(Vec::new);
        
        // Create the response structure
        let response = json!({
            "success": true,
            "data": php_array,
            "metadata": {
                "total_rows_processed": result.metadata.total_rows_processed,
                "valid_records": result.metadata.valid_records,
                "invalid_records": result.metadata.invalid_records,
                "processing_time_ms": result.metadata.processing_time_ms,
                "warnings": result.metadata.warnings
            }
        });
        
        let json = serde_json::to_string_pretty(&response)?;
        info!("Formatted output as PHP array ({} bytes)", json.len());
        Ok(json)
    }
    
    /// Writes the output to stdout.
    ///
    /// Writes the formatted output directly to standard output and flushes
    /// the buffer to ensure immediate delivery.
    ///
    /// # Arguments
    ///
    /// * `output` - The formatted string to write
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Successfully written to stdout
    /// * `Err` - If write or flush fails
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use import_cascade_fields::output::OutputFormatter;
    ///
    /// # fn main() -> anyhow::Result<()> {
    /// let output = r#"{
    ///   "success": true,
    ///   "records": []
    /// }"#;
    ///
    /// OutputFormatter::write_to_stdout(output)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn write_to_stdout(output: &str) -> Result<()> {
        let mut stdout = std::io::stdout();
        stdout.write_all(output.as_bytes())?;
        stdout.flush()?;
        Ok(())
    }
    
    /// Writes the output to a file.
    ///
    /// Creates or overwrites a file with the formatted output.
    ///
    /// # Arguments
    ///
    /// * `output` - The formatted string to write
    /// * `path` - Path to the output file
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Successfully written to file
    /// * `Err` - If file creation or write fails
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use import_cascade_fields::output::OutputFormatter;
    ///
    /// # fn main() -> anyhow::Result<()> {
    /// let output = "main_label,main_value\nCategory,CAT001";
    ///
    /// OutputFormatter::write_to_file(output, "output.csv")?;
    /// println!("Output written to output.csv");
    /// # Ok(())
    /// # }
    /// ```
    pub fn write_to_file(output: &str, path: &str) -> Result<()> {
        std::fs::write(path, output)?;
        info!("Output written to file: {}", path);
        Ok(())
    }
    
    /// Creates a summary report of the processing.
    ///
    /// Generates a human-readable summary of the processing results,
    /// including success/failure status, record counts, warnings, and timing.
    ///
    /// # Arguments
    ///
    /// * `result` - The processing result to summarize
    ///
    /// # Returns
    ///
    /// Formatted summary string with emoji indicators
    ///
    /// # Example
    ///
    /// ```rust
    /// use import_cascade_fields::models::{ProcessingResult, ProcessingMetadata};
    /// use import_cascade_fields::output::OutputFormatter;
    ///
    /// // Success case
    /// let result = ProcessingResult::success(
    ///     vec![],
    ///     ProcessingMetadata {
    ///         total_rows_processed: 100,
    ///         valid_records: 95,
    ///         invalid_records: 5,
    ///         processing_time_ms: 150,
    ///         warnings: Some(vec![
    ///             "Row 10: Missing description".to_string(),
    ///             "Row 20: Duplicate key".to_string(),
    ///         ]),
    ///     },
    /// );
    ///
    /// let summary = OutputFormatter::create_summary(&result);
    /// assert!(summary.contains("✓ Successfully processed"));
    /// assert!(summary.contains("95 records"));
    /// assert!(summary.contains("⚠ 5 invalid records"));
    /// assert!(summary.contains("Warnings:"));
    ///
    /// // Error case
    /// let error_result = ProcessingResult::error(
    ///     "File not found".to_string(),
    ///     None,
    ///     ProcessingMetadata {
    ///         total_rows_processed: 0,
    ///         valid_records: 0,
    ///         invalid_records: 0,
    ///         processing_time_ms: 5,
    ///         warnings: None,
    ///     },
    /// );
    ///
    /// let error_summary = OutputFormatter::create_summary(&error_result);
    /// assert!(error_summary.contains("✗ Processing failed"));
    /// assert!(error_summary.contains("File not found"));
    /// ```
    pub fn create_summary(result: &ProcessingResult) -> String {
        let mut summary = String::new();
        
        if result.success {
            summary.push_str(&format!(
                "✓ Successfully processed {} records\n",
                result.metadata.valid_records
            ));
            
            if result.metadata.invalid_records > 0 {
                summary.push_str(&format!(
                    "⚠ {} invalid records were skipped\n",
                    result.metadata.invalid_records
                ));
            }
            
            summary.push_str(&format!(
                "⏱ Processing time: {}ms\n",
                result.metadata.processing_time_ms
            ));
            
            if let Some(warnings) = &result.metadata.warnings {
                if !warnings.is_empty() {
                    summary.push_str("\nWarnings:\n");
                    for warning in warnings.iter().take(5) {
                        summary.push_str(&format!("  - {}\n", warning));
                    }
                    if warnings.len() > 5 {
                        summary.push_str(&format!("  ... and {} more warnings\n", warnings.len() - 5));
                    }
                }
            }
        } else {
            summary.push_str(&format!(
                "✗ Processing failed: {}\n",
                result.error.as_ref().unwrap_or(&"Unknown error".to_string())
            ));
            
            if let Some(details) = &result.details {
                summary.push_str(&format!("  File: {}\n", details.file));
                
                if let Some(sheets) = &details.available_sheets {
                    summary.push_str("  Available sheets: ");
                    summary.push_str(&sheets.join(", "));
                    summary.push('\n');
                }
            }
        }
        
        summary
    }
}
