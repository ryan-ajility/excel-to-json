//! Data models for the Cascade Fields import tool.
//!
//! This module contains the core data structures used throughout the application,
//! including the main `CascadeField` struct that represents database records
//! and supporting types for processing results and error handling.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Represents a single Cascade Field record matching the database schema.
///
/// This struct maps directly to the `cascade_fields` database table and contains
/// hierarchical classification data with four levels: main, sub, major, and minor.
/// Each level has three associated fields: label, value, and description.
///
/// # Example
///
/// ```rust
/// use import_cascade_fields::models::CascadeField;
///
/// // Create a CascadeField from raw data
/// let row_data = vec![
///     Some("Category".to_string()),      // main_label
///     Some("CAT001".to_string()),        // main_value
///     Some("Main category".to_string()), // main_description
///     Some("Subcategory".to_string()),   // sub_label
///     Some("SUB001".to_string()),        // sub_value
///     Some("Subcategory desc".to_string()), // sub_description
///     Some("Major".to_string()),          // major_label
///     Some("MAJ001".to_string()),        // major_value
///     Some("Major desc".to_string()),    // major_description
///     Some("Minor".to_string()),          // minor_label
///     Some("MIN001".to_string()),        // minor_value
///     Some("Minor desc".to_string()),    // minor_description
/// ];
///
/// let field = CascadeField::from_row(row_data).unwrap();
/// assert!(field.is_valid());
/// assert!(field.has_complete_keys());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CascadeField {
    pub main_label: Option<String>,
    pub main_value: Option<String>,
    pub main_description: Option<String>,
    pub sub_label: Option<String>,
    pub sub_value: Option<String>,
    pub sub_description: Option<String>,
    pub major_label: Option<String>,
    pub major_value: Option<String>,
    pub major_description: Option<String>,
    pub minor_label: Option<String>,
    pub minor_value: Option<String>,
    pub minor_description: Option<String>,
}

impl CascadeField {
    /// Creates a new CascadeField from raw row data.
    ///
    /// This function takes a vector of optional strings representing a row from
    /// an Excel spreadsheet and converts it into a `CascadeField` struct.
    /// The function expects at least 12 columns in the specific order matching
    /// the database schema.
    ///
    /// # Arguments
    ///
    /// * `row` - A vector of optional strings representing the Excel row data
    ///
    /// # Returns
    ///
    /// * `Some(CascadeField)` if the row has at least 12 columns
    /// * `None` if the row has insufficient columns
    ///
    /// # Example
    ///
    /// ```rust
    /// use import_cascade_fields::models::CascadeField;
    ///
    /// // Valid row with all fields
    /// let complete_row = vec![
    ///     Some("Main".to_string()),
    ///     Some("M001".to_string()),
    ///     Some("Main Description".to_string()),
    ///     Some("Sub".to_string()),
    ///     Some("S001".to_string()),
    ///     Some("Sub Description".to_string()),
    ///     Some("Major".to_string()),
    ///     Some("MAJ001".to_string()),
    ///     Some("Major Description".to_string()),
    ///     Some("Minor".to_string()),
    ///     Some("MIN001".to_string()),
    ///     Some("Minor Description".to_string()),
    /// ];
    ///
    /// let field = CascadeField::from_row(complete_row).unwrap();
    /// assert_eq!(field.main_value, Some("M001".to_string()));
    ///
    /// // Row with missing optional fields (still valid)
    /// let partial_row = vec![
    ///     None,  // main_label can be None
    ///     Some("M002".to_string()),
    ///     None,  // main_description can be None
    ///     None,
    ///     Some("S002".to_string()),
    ///     None,
    ///     None,
    ///     Some("MAJ002".to_string()),
    ///     None,
    ///     None,
    ///     Some("MIN002".to_string()),
    ///     None,
    /// ];
    ///
    /// let field = CascadeField::from_row(partial_row).unwrap();
    /// assert!(field.is_valid());
    ///
    /// // Invalid row (too few columns)
    /// let invalid_row = vec![Some("test".to_string())];
    /// assert!(CascadeField::from_row(invalid_row).is_none());
    /// ```
    pub fn from_row(row: Vec<Option<String>>) -> Option<Self> {
        if row.len() < 12 {
            return None;
        }

        Some(CascadeField {
            main_label: row.get(0).cloned().flatten(),
            main_value: row.get(1).cloned().flatten(),
            main_description: row.get(2).cloned().flatten(),
            sub_label: row.get(3).cloned().flatten(),
            sub_value: row.get(4).cloned().flatten(),
            sub_description: row.get(5).cloned().flatten(),
            major_label: row.get(6).cloned().flatten(),
            major_value: row.get(7).cloned().flatten(),
            major_description: row.get(8).cloned().flatten(),
            minor_label: row.get(9).cloned().flatten(),
            minor_value: row.get(10).cloned().flatten(),
            minor_description: row.get(11).cloned().flatten(),
        })
    }


    /// Validates that the record has the required composite keys.
    ///
    /// A record is considered valid if it has at least a `main_value`.
    /// This is the minimum requirement for a database record.
    ///
    /// # Returns
    ///
    /// * `true` if the record has at least a main_value
    /// * `false` otherwise
    ///
    /// # Example
    ///
    /// ```rust
    /// use import_cascade_fields::models::CascadeField;
    ///
    /// // Valid record with main_value
    /// let valid_row = vec![
    ///     None,
    ///     Some("MAIN001".to_string()),  // main_value is present
    ///     None, None, None, None, None, None, None, None, None, None,
    /// ];
    /// let field = CascadeField::from_row(valid_row).unwrap();
    /// assert!(field.is_valid());
    ///
    /// // Invalid record without main_value
    /// let invalid_row = vec![
    ///     Some("Label".to_string()),
    ///     None,  // main_value is missing
    ///     Some("Description".to_string()),
    ///     None, None, None, None, None, None, None, None, None,
    /// ];
    /// let field = CascadeField::from_row(invalid_row).unwrap();
    /// assert!(!field.is_valid());
    /// ```
    pub fn is_valid(&self) -> bool {
        // At least main_value should be present for a valid record
        // Additional validation logic can be added based on business rules
        self.main_value.is_some()
    }

    /// Checks if the record has complete composite keys.
    ///
    /// A record has complete keys when all four value fields are present:
    /// main_value, sub_value, major_value, and minor_value.
    /// This is typically required for unique identification in the database.
    ///
    /// # Returns
    ///
    /// * `true` if all four value fields are present
    /// * `false` if any value field is missing
    ///
    /// # Example
    ///
    /// ```rust
    /// use import_cascade_fields::models::CascadeField;
    ///
    /// // Complete keys
    /// let complete = vec![
    ///     None,
    ///     Some("M001".to_string()),    // main_value
    ///     None,
    ///     None,
    ///     Some("S001".to_string()),    // sub_value
    ///     None,
    ///     None,
    ///     Some("MAJ001".to_string()),  // major_value
    ///     None,
    ///     None,
    ///     Some("MIN001".to_string()),  // minor_value
    ///     None,
    /// ];
    /// let field = CascadeField::from_row(complete).unwrap();
    /// assert!(field.has_complete_keys());
    ///
    /// // Incomplete keys (missing minor_value)
    /// let incomplete = vec![
    ///     None,
    ///     Some("M001".to_string()),
    ///     None,
    ///     None,
    ///     Some("S001".to_string()),
    ///     None,
    ///     None,
    ///     Some("MAJ001".to_string()),
    ///     None,
    ///     None,
    ///     None,  // minor_value is missing
    ///     None,
    /// ];
    /// let field = CascadeField::from_row(incomplete).unwrap();
    /// assert!(!field.has_complete_keys());
    /// ```
    pub fn has_complete_keys(&self) -> bool {
        self.main_value.is_some()
            && self.sub_value.is_some()
            && self.major_value.is_some()
            && self.minor_value.is_some()
    }


    /// Converts the CascadeField to a PHP-compatible associative array representation.
    ///
    /// This method creates a JSON object that can be easily consumed by PHP applications.
    /// None values are converted to empty strings for compatibility with PHP's
    /// handling of database NULL values.
    ///
    /// # Returns
    ///
    /// A `serde_json::Value` object representing the field as an associative array
    ///
    /// # Example
    ///
    /// ```rust
    /// use import_cascade_fields::models::CascadeField;
    /// use serde_json::json;
    ///
    /// let row = vec![
    ///     Some("Category".to_string()),
    ///     Some("CAT001".to_string()),
    ///     None,  // This will become an empty string in PHP
    ///     None, None, None, None, None, None, None, None, None,
    /// ];
    ///
    /// let field = CascadeField::from_row(row).unwrap();
    /// let php_array = field.to_php_array();
    ///
    /// // The result can be serialized to JSON for PHP consumption
    /// let json_str = php_array.to_string();
    /// assert!(json_str.contains("\"main_label\":\"Category\""));
    /// assert!(json_str.contains("\"main_value\":\"CAT001\""));
    /// assert!(json_str.contains("\"main_description\":\"\""));  // Empty string for None
    /// ```
    pub fn to_php_array(&self) -> Value {
        json!({
            "main_label": self.main_label.as_ref().map(|s| s.as_str()).unwrap_or(""),
            "main_value": self.main_value.as_ref().map(|s| s.as_str()).unwrap_or(""),
            "main_description": self.main_description.as_ref().map(|s| s.as_str()).unwrap_or(""),
            "sub_label": self.sub_label.as_ref().map(|s| s.as_str()).unwrap_or(""),
            "sub_value": self.sub_value.as_ref().map(|s| s.as_str()).unwrap_or(""),
            "sub_description": self.sub_description.as_ref().map(|s| s.as_str()).unwrap_or(""),
            "major_label": self.major_label.as_ref().map(|s| s.as_str()).unwrap_or(""),
            "major_value": self.major_value.as_ref().map(|s| s.as_str()).unwrap_or(""),
            "major_description": self.major_description.as_ref().map(|s| s.as_str()).unwrap_or(""),
            "minor_label": self.minor_label.as_ref().map(|s| s.as_str()).unwrap_or(""),
            "minor_value": self.minor_value.as_ref().map(|s| s.as_str()).unwrap_or(""),
            "minor_description": self.minor_description.as_ref().map(|s| s.as_str()).unwrap_or(""),
        })
    }
}

/// Represents the output structure for PHP integration.
///
/// This struct encapsulates the complete result of a processing operation,
/// including success/failure status, processed records, error information,
/// and processing metadata.
///
/// # Example
///
/// ```rust
/// use import_cascade_fields::models::{ProcessingResult, ProcessingMetadata, CascadeField};
///
/// // Create a successful result
/// let records = vec![
///     // ... CascadeField instances
/// ];
/// let metadata = ProcessingMetadata {
///     total_rows_processed: 100,
///     valid_records: 95,
///     invalid_records: 5,
///     processing_time_ms: 250,
///     warnings: Some(vec!["Row 10: Missing minor_value".to_string()]),
/// };
///
/// let success_result = ProcessingResult::success(records, metadata);
/// assert!(success_result.success);
///
/// // Create an error result
/// let error_result = ProcessingResult::error(
///     "File not found".to_string(),
///     None,
///     ProcessingMetadata {
///         total_rows_processed: 0,
///         valid_records: 0,
///         invalid_records: 0,
///         processing_time_ms: 10,
///         warnings: None,
///     },
/// );
/// assert!(!error_result.success);
/// ```
#[derive(Debug, Serialize)]
pub struct ProcessingResult {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<CascadeField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ErrorDetails>,
    pub metadata: ProcessingMetadata,
}

/// Additional error details for debugging and user feedback.
///
/// This struct provides context about errors that occur during processing,
/// helping users understand and resolve issues.
///
/// # Example
///
/// ```rust
/// use import_cascade_fields::models::ErrorDetails;
///
/// let details = ErrorDetails {
///     file: "/path/to/file.xlsx".to_string(),
///     available_sheets: Some(vec![
///         "Sheet1".to_string(),
///         "Data".to_string(),
///         "Summary".to_string(),
///     ]),
///     row_number: Some(42),
///     column: Some("minor_value".to_string()),
/// };
/// ```
#[derive(Debug, Serialize)]
pub struct ErrorDetails {
    pub file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_sheets: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_number: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<String>,
}

/// Metadata about the processing operation.
///
/// Contains statistics and performance metrics about the import process,
/// useful for monitoring and optimization.
///
/// # Example
///
/// ```rust
/// use import_cascade_fields::models::ProcessingMetadata;
///
/// let metadata = ProcessingMetadata {
///     total_rows_processed: 1000,
///     valid_records: 950,
///     invalid_records: 50,
///     processing_time_ms: 1500,
///     warnings: Some(vec![
///         "Row 100: Duplicate key detected".to_string(),
///         "Row 250: Missing description fields".to_string(),
///     ]),
/// };
///
/// // Calculate success rate
/// let success_rate = (metadata.valid_records as f64 / metadata.total_rows_processed as f64) * 100.0;
/// println!("Success rate: {:.2}%", success_rate);
/// ```
#[derive(Debug, Serialize)]
pub struct ProcessingMetadata {
    pub total_rows_processed: usize,
    pub valid_records: usize,
    pub invalid_records: usize,
    pub processing_time_ms: u128,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

impl ProcessingResult {
    /// Creates a successful processing result.
    ///
    /// Use this method when the processing completes successfully,
    /// even if some records were invalid or skipped.
    ///
    /// # Arguments
    ///
    /// * `records` - Vector of successfully processed CascadeField records
    /// * `metadata` - Processing statistics and metrics
    ///
    /// # Example
    ///
    /// ```rust
    /// use import_cascade_fields::models::{ProcessingResult, ProcessingMetadata, CascadeField};
    ///
    /// let records = vec![
    ///     // ... processed CascadeField instances
    /// ];
    ///
    /// let metadata = ProcessingMetadata {
    ///     total_rows_processed: 100,
    ///     valid_records: 100,
    ///     invalid_records: 0,
    ///     processing_time_ms: 150,
    ///     warnings: None,
    /// };
    ///
    /// let result = ProcessingResult::success(records, metadata);
    /// assert!(result.success);
    /// assert!(result.error.is_none());
    /// ```
    pub fn success(records: Vec<CascadeField>, metadata: ProcessingMetadata) -> Self {
        ProcessingResult {
            success: true,
            records: Some(records),
            error: None,
            details: None,
            metadata,
        }
    }

    /// Creates an error processing result.
    ///
    /// Use this method when the processing fails completely and cannot continue.
    ///
    /// # Arguments
    ///
    /// * `error` - Error message describing what went wrong
    /// * `details` - Optional additional context about the error
    /// * `metadata` - Processing statistics up to the point of failure
    ///
    /// # Example
    ///
    /// ```rust
    /// use import_cascade_fields::models::{ProcessingResult, ProcessingMetadata, ErrorDetails};
    ///
    /// let details = ErrorDetails {
    ///     file: "data.xlsx".to_string(),
    ///     available_sheets: Some(vec!["Sheet1".to_string()]),
    ///     row_number: None,
    ///     column: None,
    /// };
    ///
    /// let metadata = ProcessingMetadata {
    ///     total_rows_processed: 0,
    ///     valid_records: 0,
    ///     invalid_records: 0,
    ///     processing_time_ms: 5,
    ///     warnings: None,
    /// };
    ///
    /// let result = ProcessingResult::error(
    ///     "Sheet 'Cascade Fields' not found".to_string(),
    ///     Some(details),
    ///     metadata,
    /// );
    ///
    /// assert!(!result.success);
    /// assert!(result.records.is_none());
    /// assert_eq!(result.error, Some("Sheet 'Cascade Fields' not found".to_string()));
    /// ```
    pub fn error(error: String, details: Option<ErrorDetails>, metadata: ProcessingMetadata) -> Self {
        ProcessingResult {
            success: false,
            records: None,
            error: Some(error),
            details,
            metadata,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cascade_field_creation() {
        let row = vec![
            Some("Main Label".to_string()),
            Some("MAIN1".to_string()),
            Some("Main Description".to_string()),
            Some("Sub Label".to_string()),
            Some("SUB1".to_string()),
            Some("Sub Description".to_string()),
            Some("Major Label".to_string()),
            Some("MAJ1".to_string()),
            Some("Major Description".to_string()),
            Some("Minor Label".to_string()),
            Some("MIN1".to_string()),
            Some("Minor Description".to_string()),
        ];
        
        let field = CascadeField::from_row(row).expect("Should create field");
        
        assert_eq!(field.main_value, Some("MAIN1".to_string()));
        assert_eq!(field.sub_value, Some("SUB1".to_string()));
        assert_eq!(field.major_value, Some("MAJ1".to_string()));
        assert_eq!(field.minor_value, Some("MIN1".to_string()));
        assert!(field.is_valid());
        assert!(field.has_complete_keys());
    }
    
    #[test]
    fn test_incomplete_cascade_field() {
        let row = vec![
            Some("Main Label".to_string()),
            None, // Missing main_value
            Some("Main Description".to_string()),
            Some("Sub Label".to_string()),
            Some("SUB1".to_string()),
            Some("Sub Description".to_string()),
            Some("Major Label".to_string()),
            Some("MAJ1".to_string()),
            Some("Major Description".to_string()),
            Some("Minor Label".to_string()),
            Some("MIN1".to_string()),
            Some("Minor Description".to_string()),
        ];
        
        let field = CascadeField::from_row(row).expect("Should create field");
        
        assert!(!field.is_valid());
        assert!(!field.has_complete_keys());
    }
    
    #[test]
    fn test_processing_result_success() {
        let records = vec![
            CascadeField::from_row(vec![
                Some("Main".to_string()),
                Some("M1".to_string()),
                Some("Desc".to_string()),
                Some("Sub".to_string()),
                Some("S1".to_string()),
                Some("SubDesc".to_string()),
                Some("Major".to_string()),
                Some("MAJ1".to_string()),
                Some("MajDesc".to_string()),
                Some("Minor".to_string()),
                Some("MIN1".to_string()),
                Some("MinDesc".to_string()),
            ]).unwrap(),
        ];
        
        let metadata = ProcessingMetadata {
            total_rows_processed: 1,
            valid_records: 1,
            invalid_records: 0,
            processing_time_ms: 100,
            warnings: None,
        };
        
        let result = ProcessingResult::success(records.clone(), metadata);
        
        assert!(result.success);
        assert!(result.records.is_some());
        assert_eq!(result.records.unwrap().len(), 1);
        assert!(result.error.is_none());
    }
    
    #[test]
    fn test_processing_result_error() {
        let metadata = ProcessingMetadata {
            total_rows_processed: 0,
            valid_records: 0,
            invalid_records: 0,
            processing_time_ms: 10,
            warnings: None,
        };
        
        let result = ProcessingResult::error(
            "Test error".to_string(),
            None,
            metadata,
        );
        
        assert!(!result.success);
        assert!(result.records.is_none());
        assert_eq!(result.error, Some("Test error".to_string()));
    }
}
