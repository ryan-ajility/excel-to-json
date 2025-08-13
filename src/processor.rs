//! Data processing module for transforming Excel data into CascadeField records.
//!
//! This module contains the core business logic for validating, cleaning,
//! and transforming raw Excel data into structured CascadeField records
//! ready for database insertion.
//!
//! # Example
//!
//! ```rust
//! use excel_to_json::processor::DataProcessor;
//! use excel_to_json::models::CascadeField;
//!
//! # fn main() -> anyhow::Result<()> {
//! let mut processor = DataProcessor::new();
//!
//! // Sample raw data from Excel
//! let raw_rows = vec![
//!     vec![
//!         Some("Main".to_string()),
//!         Some("M001".to_string()),
//!         None,
//!         Some("Sub".to_string()),
//!         Some("S001".to_string()),
//!         None,
//!         None, None, None, None, None, None,
//!     ],
//! ];
//!
//! let (records, metadata) = processor.process_rows(raw_rows)?;
//! println!("Processed {} valid records", records.len());
//! # Ok(())
//! # }
//! ```

use crate::models::{CascadeField, ProcessingMetadata};
use anyhow::Result;
use tracing::{debug, info, warn};

/// Processes raw Excel data into validated CascadeField records.
///
/// The `DataProcessor` handles the transformation of raw Excel rows into
/// structured `CascadeField` records, including validation and cleaning.
///
/// # Example
///
/// ```rust
/// use excel_to_json::processor::DataProcessor;
///
/// # fn main() -> anyhow::Result<()> {
/// // Create a new processor
/// let mut processor = DataProcessor::new();
///
/// // Process some rows
/// let raw_data = vec![
///     vec![Some("Label".to_string()); 12],
/// ];
///
/// let (records, metadata) = processor.process_rows(raw_data)?;
///
/// // Check processing results
/// assert_eq!(metadata.total_rows_processed, 1);
/// println!("Valid records: {}", metadata.valid_records);
/// println!("Invalid records: {}", metadata.invalid_records);
///
/// if let Some(warnings) = &metadata.warnings {
///     for warning in warnings {
///         println!("Warning: {}", warning);
///     }
/// }
/// # Ok(())
/// # }
/// ```
pub struct DataProcessor {
    warnings: Vec<String>,
}

impl DataProcessor {
    /// Creates a new DataProcessor instance.
    ///
    /// Initializes a processor with an empty warnings vector that will
    /// collect any issues encountered during processing.
    ///
    /// # Example
    ///
    /// ```rust
    /// use excel_to_json::processor::DataProcessor;
    ///
    /// let processor = DataProcessor::new();
    /// // Processor is ready to process Excel rows
    /// ```
    pub fn new() -> Self {
        DataProcessor {
            warnings: Vec::new(),
        }
    }

    /// Processes raw Excel rows into validated CascadeField records.
    ///
    /// This is the main processing method that transforms raw Excel data into
    /// structured records. It performs the following operations:
    /// - Converts each row to a CascadeField
    /// - Cleans and normalizes field data
    /// - Validates records for required fields
    /// - Collects processing warnings
    ///
    /// # Arguments
    ///
    /// * `raw_rows` - Vector of raw Excel rows, each containing optional string values
    ///
    /// # Returns
    ///
    /// * `Ok((records, metadata))` - Successfully processed records and statistics
    /// * `Err` - If a critical processing error occurs
    ///
    /// # Example
    ///
    /// ```rust
    /// use excel_to_json::processor::DataProcessor;
    ///
    /// # fn main() -> anyhow::Result<()> {
    /// let mut processor = DataProcessor::new();
    ///
    /// // Sample data with valid and invalid rows
    /// let raw_rows = vec![
    ///     // Valid row with all required fields
    ///     vec![
    ///         Some("Main Label".to_string()),
    ///         Some("MAIN001".to_string()),  // Required main_value
    ///         Some("Main Description".to_string()),
    ///         Some("Sub Label".to_string()),
    ///         Some("SUB001".to_string()),
    ///         Some("Sub Description".to_string()),
    ///         Some("Major Label".to_string()),
    ///         Some("MAJ001".to_string()),
    ///         Some("Major Description".to_string()),
    ///         Some("Minor Label".to_string()),
    ///         Some("MIN001".to_string()),
    ///         Some("Minor Description".to_string()),
    ///     ],
    ///     // Invalid row (missing main_value)
    ///     vec![
    ///         Some("Label".to_string()),
    ///         None,  // Missing required main_value
    ///         None, None, None, None, None, None, None, None, None, None,
    ///     ],
    /// ];
    ///
    /// let (records, metadata) = processor.process_rows(raw_rows)?;
    ///
    /// assert_eq!(records.len(), 1);  // Only the valid record
    /// assert_eq!(metadata.total_rows_processed, 2);
    /// assert_eq!(metadata.valid_records, 1);
    /// assert_eq!(metadata.invalid_records, 1);
    /// # Ok(())
    /// # }
    /// ```
    pub fn process_rows(&mut self, raw_rows: Vec<Vec<Option<String>>>) -> Result<(Vec<CascadeField>, ProcessingMetadata)> {
        let start_time = std::time::Instant::now();
        let total_rows = raw_rows.len();
        
        info!("Processing {} rows", total_rows);
        
        let mut valid_records = Vec::new();
        let mut invalid_count = 0;
        
        for (row_idx, row) in raw_rows.into_iter().enumerate() {
            // Convert row to CascadeField
            match CascadeField::from_row(row.clone()) {
                Some(mut field) => {
                    // Trim whitespace from all string fields
                    self.clean_field(&mut field);
                    
                    // Validate the field
                    if field.is_valid() {
                        debug!("Valid record at row {}", row_idx + 2);
                        valid_records.push(field);
                    } else {
                        debug!("Invalid record at row {} - missing required fields", row_idx + 2);
                        invalid_count += 1;
                        
                        // Add warning for incomplete keys if applicable
                        if !field.has_complete_keys() {
                            self.warnings.push(format!(
                                "Row {}: Incomplete composite keys",
                                row_idx + 2
                            ));
                        }
                    }
                },
                None => {
                    debug!("Failed to parse row {}", row_idx + 2);
                    invalid_count += 1;
                    self.warnings.push(format!("Row {}: Insufficient columns", row_idx + 2));
                }
            }
        }
        
        let processing_time = start_time.elapsed().as_millis();
        
        info!(
            "Processing complete: {} valid records, {} invalid records in {}ms",
            valid_records.len(),
            invalid_count,
            processing_time
        );
        
        // Log warnings if any
        if !self.warnings.is_empty() {
            warn!("Processing warnings: {:?}", self.warnings);
        }
        
        let metadata = ProcessingMetadata {
            total_rows_processed: total_rows,
            valid_records: valid_records.len(),
            invalid_records: invalid_count,
            processing_time_ms: processing_time,
            warnings: if self.warnings.is_empty() {
                None
            } else {
                Some(self.warnings.clone())
            },
        };
        
        Ok((valid_records, metadata))
    }
    
    /// Cleans a CascadeField by trimming whitespace and normalizing empty strings.
    ///
    /// This method performs data cleaning operations on all string fields:
    /// - Trims leading and trailing whitespace
    /// - Converts empty strings to None
    /// - Preserves None values
    ///
    /// # Arguments
    ///
    /// * `field` - Mutable reference to the CascadeField to clean
    ///
    /// # Example
    ///
    /// ```rust
    /// use excel_to_json::models::CascadeField;
    /// use excel_to_json::processor::DataProcessor;
    ///
    /// let processor = DataProcessor::new();
    ///
    /// // Create a field with whitespace
    /// let row = vec![
    ///     Some("  Main Label  ".to_string()),  // Has whitespace
    ///     Some("MAIN001".to_string()),
    ///     Some("".to_string()),  // Empty string
    ///     Some("   ".to_string()),  // Only whitespace
    ///     None, None, None, None, None, None, None, None,
    /// ];
    ///
    /// let mut field = CascadeField::from_row(row).unwrap();
    /// // After cleaning (done internally in process_rows):
    /// // - "  Main Label  " becomes "Main Label"
    /// // - "" becomes None
    /// // - "   " becomes None
    /// ```
    fn clean_field(&self, field: &mut CascadeField) {
        field.main_label = field.main_label.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
        field.main_value = field.main_value.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
        field.main_description = field.main_description.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
        
        field.sub_label = field.sub_label.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
        field.sub_value = field.sub_value.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
        field.sub_description = field.sub_description.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
        
        field.major_label = field.major_label.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
        field.major_value = field.major_value.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
        field.major_description = field.major_description.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
        
        field.minor_label = field.minor_label.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
        field.minor_value = field.minor_value.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
        field.minor_description = field.minor_description.as_ref().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
    }
    
    
    /// Filters records by completeness of composite keys.
    ///
    /// Returns only records that have all four value fields populated
    /// (main_value, sub_value, major_value, minor_value).
    ///
    /// # Arguments
    ///
    /// * `records` - Vector of CascadeField records to filter
    ///
    /// # Returns
    ///
    /// Vector containing only records with complete composite keys
    ///
    /// # Example
    ///
    /// ```rust
    /// use excel_to_json::models::CascadeField;
    /// use excel_to_json::processor::DataProcessor;
    ///
    /// // Create a mix of complete and incomplete records
    /// let records = vec![
    ///     // Complete record
    ///     CascadeField::from_row(vec![
    ///         None,
    ///         Some("M001".to_string()),
    ///         None, None,
    ///         Some("S001".to_string()),
    ///         None, None,
    ///         Some("MAJ001".to_string()),
    ///         None, None,
    ///         Some("MIN001".to_string()),
    ///         None,
    ///     ]).unwrap(),
    ///     // Incomplete record (missing minor_value)
    ///     CascadeField::from_row(vec![
    ///         None,
    ///         Some("M002".to_string()),
    ///         None, None,
    ///         Some("S002".to_string()),
    ///         None, None,
    ///         Some("MAJ002".to_string()),
    ///         None, None,
    ///         None,  // Missing minor_value
    ///         None,
    ///     ]).unwrap(),
    /// ];
    ///
    /// let complete = DataProcessor::filter_complete_records(records);
    /// assert_eq!(complete.len(), 1);
    /// assert_eq!(complete[0].main_value, Some("M001".to_string()));
    /// ```
    #[allow(dead_code)]
    pub fn filter_complete_records(records: Vec<CascadeField>) -> Vec<CascadeField> {
        records.into_iter()
            .filter(|record| record.has_complete_keys())
            .collect()
    }
    
    /// Groups records by main category for analysis.
    ///
    /// Creates a HashMap where records are grouped by their main_value field.
    /// This is useful for analyzing the distribution of records across main categories.
    ///
    /// # Arguments
    ///
    /// * `records` - Slice of CascadeField records to group
    ///
    /// # Returns
    ///
    /// HashMap where:
    /// - Key: main_value as String
    /// - Value: Vector of references to CascadeField records with that main_value
    ///
    /// # Example
    ///
    /// ```rust
    /// use excel_to_json::models::CascadeField;
    /// use excel_to_json::processor::DataProcessor;
    ///
    /// // Create records with different main values
    /// let records = vec![
    ///     CascadeField::from_row(vec![
    ///         None,
    ///         Some("CATEGORY_A".to_string()),
    ///         None, None, None, None, None, None, None, None, None, None,
    ///     ]).unwrap(),
    ///     CascadeField::from_row(vec![
    ///         None,
    ///         Some("CATEGORY_B".to_string()),
    ///         None, None, None, None, None, None, None, None, None, None,
    ///     ]).unwrap(),
    ///     CascadeField::from_row(vec![
    ///         None,
    ///         Some("CATEGORY_A".to_string()),  // Another CATEGORY_A
    ///         None, None, None, None, None, None, None, None, None, None,
    ///     ]).unwrap(),
    /// ];
    ///
    /// let grouped = DataProcessor::group_by_main_value(&records);
    ///
    /// assert_eq!(grouped.len(), 2);  // Two unique categories
    /// assert_eq!(grouped.get("CATEGORY_A").unwrap().len(), 2);
    /// assert_eq!(grouped.get("CATEGORY_B").unwrap().len(), 1);
    ///
    /// // Analyze distribution
    /// for (category, items) in &grouped {
    ///     println!("{}: {} records", category, items.len());
    /// }
    /// ```
    #[allow(dead_code)]
    pub fn group_by_main_value(records: &[CascadeField]) -> std::collections::HashMap<String, Vec<&CascadeField>> {
        use std::collections::HashMap;
        
        let mut grouped = HashMap::new();
        
        for record in records {
            if let Some(main_value) = &record.main_value {
                grouped.entry(main_value.clone())
                    .or_insert_with(Vec::new)
                    .push(record);
            }
        }
        
        grouped
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_data_processor() {
        let mut processor = DataProcessor::new();
        
        let rows = vec![
            vec![
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
            ],
            vec![
                Some("Main Label 2".to_string()),
                None, // Invalid row - missing main_value
                Some("Main Description 2".to_string()),
                Some("Sub Label 2".to_string()),
                Some("SUB2".to_string()),
                Some("Sub Description 2".to_string()),
                Some("Major Label 2".to_string()),
                Some("MAJ2".to_string()),
                Some("Major Description 2".to_string()),
                Some("Minor Label 2".to_string()),
                Some("MIN2".to_string()),
                Some("Minor Description 2".to_string()),
            ],
        ];
        
        let (records, metadata) = processor.process_rows(rows).expect("Should process rows");
        
        assert_eq!(records.len(), 1);
        assert_eq!(metadata.valid_records, 1);
        assert_eq!(metadata.invalid_records, 1);
        assert_eq!(metadata.total_rows_processed, 2);
    }
    
    #[test]
    fn test_multiple_valid_records() {
        let mut processor = DataProcessor::new();
        
        let rows = vec![
            vec![
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
            ],
            vec![
                Some("Main Label 2".to_string()),
                Some("MAIN1".to_string()), // Same value but different record
                Some("Different Description".to_string()),
                Some("Sub Label".to_string()),
                Some("SUB1".to_string()),
                Some("Different Sub Description".to_string()),
                Some("Major Label".to_string()),
                Some("MAJ1".to_string()),
                Some("Different Major Description".to_string()),
                Some("Minor Label".to_string()),
                Some("MIN1".to_string()),
                Some("Different Minor Description".to_string()),
            ],
        ];
        
        let (records, metadata) = processor.process_rows(rows).expect("Should process rows");
        
        // Both records should be included since we're not checking for duplicates
        assert_eq!(records.len(), 2);
        assert_eq!(metadata.valid_records, 2);
        assert_eq!(metadata.invalid_records, 0);
        assert_eq!(metadata.total_rows_processed, 2);
    }
}
