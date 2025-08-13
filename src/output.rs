//! Output formatting module for the Excel to JSON export tool.
//!
//! This module handles the formatting and output of processed Excel data
//! as JSON for consumption by various systems.
//!
//! # Supported Format
//!
//! - **JSON** - Standard JSON format for API responses and data interchange
//!
//! # Example
//!
//! ```rust
//! use excel_to_json::output::{OutputFormatter, OutputFormat};
//! use excel_to_json::models::{ProcessingResult, ProcessingMetadata};
//!
//! # fn main() -> anyhow::Result<()> {
//! let result = ProcessingResult::success(
//!     vec![],  // Processed records
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
//! # Ok(())
//! # }
//! ```

use crate::models::ProcessingResult;
use anyhow::Result;
use serde_json::{self, json, Value};
use std::io::Write;
use tracing::info;

/// Output format options for processed data.
///
/// Currently only supports JSON output format.
///
/// # Example
///
/// ```rust
/// use excel_to_json::output::OutputFormat;
/// use std::str::FromStr;
///
/// // Parse from string
/// let format = OutputFormat::from_str("json").unwrap();
/// matches!(format, OutputFormat::Json);
/// ```
#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;
    
    /// Parses an OutputFormat from a string.
    ///
    /// Accepts "json" (case-insensitive)
    ///
    /// # Example
    ///
    /// ```rust
    /// use excel_to_json::output::OutputFormat;
    /// use std::str::FromStr;
    ///
    /// assert!(matches!(OutputFormat::from_str("json"), Ok(OutputFormat::Json)));
    /// assert!(matches!(OutputFormat::from_str("JSON"), Ok(OutputFormat::Json)));
    /// assert!(OutputFormat::from_str("invalid").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            _ => Err(format!("Unknown output format: {}. Only 'json' is supported.", s)),
        }
    }
}

/// Handles output formatting for JSON export.
///
/// The `OutputFormatter` provides static methods to format processing results
/// as JSON and write them to different destinations.
///
/// # Example
///
/// ```rust
/// use excel_to_json::output::{OutputFormatter, OutputFormat};
/// use excel_to_json::models::{ProcessingResult, ProcessingMetadata, CascadeField};
/// use std::io::Write;
///
/// # fn main() -> anyhow::Result<()> {
/// // Create sample result
/// let result = ProcessingResult::success(
///     vec![
///         // Processed records
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
    /// Formats the processing result as JSON.
    ///
    /// # Arguments
    ///
    /// * `result` - The processing result to format
    /// * `format` - The desired output format (currently only JSON)
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - Formatted output as a JSON string
    /// * `Err` - If formatting fails
    ///
    /// # Example
    ///
    /// ```rust
    /// use excel_to_json::output::{OutputFormatter, OutputFormat};
    /// use excel_to_json::models::{ProcessingResult, ProcessingMetadata};
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
    /// # Ok(())
    /// # }
    /// ```
    pub fn format_output(result: &ProcessingResult, format: OutputFormat) -> Result<String> {
        match format {
            OutputFormat::Json => Self::format_json(result),
        }
    }
    
    /// Formats the result as JSON.
    ///
    /// Creates a JSON representation with all records converted to a generic
    /// format where None values become empty strings for compatibility.
    ///
    /// # Arguments
    ///
    /// * `result` - The processing result to format
    ///
    /// # Returns
    ///
    /// Pretty-printed JSON string
    ///
    /// # JSON Structure for Multi-Sheet
    ///
    /// ```json
    /// {
    ///   "success": true,
    ///   "data": [
    ///     {
    ///       "sheet": "Sheet1",
    ///       "rows": [...]
    ///     },
    ///     {
    ///       "sheet": "Sheet2",
    ///       "rows": [...]
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
    fn format_json(result: &ProcessingResult) -> Result<String> {
        if !result.success {
            // For errors, return an error structure
            let error_response = json!({
                "success": false,
                "error": result.error.as_ref().unwrap_or(&"Unknown error".to_string()),
                "data": []
            });
            return Ok(serde_json::to_string_pretty(&error_response)?);
        }
        
        // Check if this is a multi-sheet result
        let data = if let Some(sheet_data) = &result.sheet_data {
            // Format multi-sheet data
            sheet_data.iter()
                .map(|sheet| {
                    json!({
                        "sheet": sheet.sheet,
                        "rows": sheet.rows.iter()
                            .map(|record| record.to_php_array())
                            .collect::<Vec<Value>>()
                    })
                })
                .collect::<Vec<Value>>()
        } else if let Some(records) = &result.records {
            // Format single-sheet data (backwards compatibility)
            records.iter()
                .map(|record| record.to_php_array())
                .collect()
        } else {
            Vec::new()
        };
        
        // Create the response structure
        let response = json!({
            "success": true,
            "data": data,
            "metadata": {
                "total_rows_processed": result.metadata.total_rows_processed,
                "valid_records": result.metadata.valid_records,
                "invalid_records": result.metadata.invalid_records,
                "processing_time_ms": result.metadata.processing_time_ms,
                "warnings": result.metadata.warnings
            }
        });
        
        let json = serde_json::to_string_pretty(&response)?;
        info!("Formatted output as JSON ({} bytes)", json.len());
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
    /// use excel_to_json::output::OutputFormatter;
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
    /// use excel_to_json::output::OutputFormatter;
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
    /// use excel_to_json::models::{ProcessingResult, ProcessingMetadata};
    /// use excel_to_json::output::OutputFormatter;
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
