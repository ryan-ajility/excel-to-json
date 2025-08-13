//! Excel file reading and processing module.
//!
//! This module provides functionality to read Excel files, navigate worksheets,
//! and process cell data including formula evaluation. It uses the `calamine`
//! crate for Excel file parsing and provides specialized handling for
//! VLOOKUP formulas commonly found in cascade field data.
//!
//! # Example
//!
//! ```rust,no_run
//! use excel_to_json::excel_reader::ExcelReader;
//!
//! # fn main() -> anyhow::Result<()> {
//! let mut reader = ExcelReader::new("data.xlsx", "Cascade Fields".to_string())?;
//! let rows = reader.read_with_formulas()?;
//! 
//! for row in rows {
//!     println!("Row data: {:?}", row);
//! }
//! # Ok(())
//! # }
//! ```

use anyhow::{Context, Result};
use calamine::{open_workbook, Data, Reader, Xlsx};
use std::collections::HashMap;
use std::path::Path;
use tracing::{debug, info, warn};

/// Reads and processes Excel files with support for formula evaluation.
///
/// The `ExcelReader` struct provides methods to read Excel worksheets,
/// process cell data, and handle formula evaluation (particularly VLOOKUP).
/// It maintains the workbook state and target sheet name for processing.
///
/// # Example
///
/// ```rust,no_run
/// use excel_to_json::excel_reader::ExcelReader;
///
/// # fn main() -> anyhow::Result<()> {
/// // Create a reader for a specific sheet
/// let mut reader = ExcelReader::new("cascade_data.xlsx", "Cascade Fields".to_string())?;
///
/// // Get available sheet names
/// let sheets = reader.get_sheet_names();
/// println!("Available sheets: {:?}", sheets);
///
/// // Read and process the data
/// let data = reader.read_with_formulas()?;
/// println!("Processed {} rows", data.len());
/// # Ok(())
/// # }
/// ```
pub struct ExcelReader {
    workbook: Xlsx<std::io::BufReader<std::fs::File>>,
    sheet_name: String,
}

impl ExcelReader {
    /// Creates a new ExcelReader for the specified file.
    ///
    /// Opens an Excel file and prepares it for reading. The reader maintains
    /// a reference to the workbook and the target sheet name.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the Excel file to open
    /// * `sheet_name` - Name of the worksheet to process
    ///
    /// # Returns
    ///
    /// * `Ok(ExcelReader)` - Successfully opened Excel file
    /// * `Err` - If the file cannot be opened or is not a valid Excel file
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use excel_to_json::excel_reader::ExcelReader;
    /// use std::path::Path;
    ///
    /// # fn main() -> anyhow::Result<()> {
    /// // Open an Excel file
    /// let reader = ExcelReader::new("data.xlsx", "Sheet1".to_string())?;
    ///
    /// // Using Path reference
    /// let path = Path::new("/path/to/file.xlsx");
    /// let reader = ExcelReader::new(path, "Cascade Fields".to_string())?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file does not exist
    /// - The file is not a valid Excel file
    /// - The file cannot be read due to permissions
    pub fn new<P: AsRef<Path>>(path: P, sheet_name: String) -> Result<Self> {
        let workbook: Xlsx<_> = open_workbook(path.as_ref())
            .with_context(|| format!("Failed to open Excel file: {:?}", path.as_ref()))?;
        
        info!("Successfully opened Excel file: {:?}", path.as_ref());
        
        Ok(ExcelReader {
            workbook,
            sheet_name,
        })
    }

    /// Returns a list of all sheet names in the workbook.
    ///
    /// This method is useful for discovering available sheets in an Excel file,
    /// particularly when handling errors related to missing sheets.
    ///
    /// # Returns
    ///
    /// A vector of sheet names as strings
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use excel_to_json::excel_reader::ExcelReader;
    ///
    /// # fn main() -> anyhow::Result<()> {
    /// let reader = ExcelReader::new("data.xlsx", String::new())?;
    /// let sheets = reader.get_sheet_names();
    ///
    /// // Check if a specific sheet exists
    /// if sheets.contains(&"Cascade Fields".to_string()) {
    ///     println!("Found Cascade Fields sheet");
    /// }
    ///
    /// // List all sheets
    /// for sheet in sheets {
    ///     println!("Sheet: {}", sheet);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_sheet_names(&self) -> Vec<String> {
        self.workbook.sheet_names().to_vec()
    }

    /// Reads the specified sheet and returns processed rows with resolved VLOOKUP values.
    ///
    /// This method processes Excel data with special handling for VLOOKUP formulas.
    /// It builds lookup tables from all sheets and attempts to resolve VLOOKUP
    /// references. Currently unused but kept for potential future VLOOKUP resolution needs.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<Vec<Option<String>>>)` - Processed rows with resolved values
    /// * `Err` - If the sheet doesn't exist or cannot be read
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use excel_to_json::excel_reader::ExcelReader;
    ///
    /// # fn main() -> anyhow::Result<()> {
    /// let mut reader = ExcelReader::new("data.xlsx", "Cascade Fields".to_string())?;
    /// let rows = reader.read_cascade_fields()?;
    ///
    /// // Process each row
    /// for (idx, row) in rows.iter().enumerate() {
    ///     println!("Row {}: {} columns", idx + 1, row.len());
    ///     
    ///     // Check for non-empty values
    ///     let non_empty = row.iter().filter(|v| v.is_some()).count();
    ///     println!("  Non-empty cells: {}", non_empty);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if the specified sheet is not found in the workbook
    #[allow(dead_code)]
    pub fn read_cascade_fields(&mut self) -> Result<Vec<Vec<Option<String>>>> {
        // Check if the sheet exists
        let sheet_names = self.get_sheet_names();
        if !sheet_names.contains(&self.sheet_name) {
            anyhow::bail!(
                "Sheet '{}' not found. Available sheets: {:?}",
                self.sheet_name,
                sheet_names
            );
        }

        info!("Reading sheet: {}", self.sheet_name);

        // Build lookup tables from all sheets for VLOOKUP resolution
        let lookup_tables = self.build_lookup_tables()?;
        
        // Read the target sheet
        let range = self.workbook
            .worksheet_range(&self.sheet_name)
            .map_err(|e| anyhow::anyhow!("Error reading sheet '{}': {}", self.sheet_name, e))?;

        let mut processed_rows = Vec::new();
        let mut is_header = true;
        
        for (row_idx, row) in range.rows().enumerate() {
            // Skip header row
            if is_header {
                is_header = false;
                debug!("Skipping header row");
                continue;
            }

            let mut processed_row = Vec::new();
            
            for (col_idx, cell) in row.iter().enumerate() {
                let value = match cell {
                    Data::String(s) => {
                        // Check if this looks like a VLOOKUP formula result
                        if s.starts_with("=VLOOKUP") || s.starts_with("=vlookup") {
                            // Try to resolve the VLOOKUP
                            match self.resolve_vlookup(s.as_str(), &lookup_tables) {
                                Some(resolved) => Some(resolved),
                                None => {
                                    warn!("Failed to resolve VLOOKUP at row {}, col {}: {}", 
                                          row_idx + 1, col_idx + 1, s);
                                    Some(s.clone())
                                }
                            }
                        } else {
                            Some(s.clone())
                        }
                    },
                    Data::Float(f) => Some(format!("{}", f)),
                    Data::Int(i) => Some(format!("{}", i)),
                    Data::Bool(b) => Some(format!("{}", b)),
                    Data::DateTime(dt) => Some(format!("{}", dt)),
                    Data::DateTimeIso(dt) => Some(dt.clone()),
                    Data::DurationIso(d) => Some(d.clone()),
                    Data::Error(e) => {
                        warn!("Error cell at row {}, col {}: {:?}", row_idx + 1, col_idx + 1, e);
                        None
                    },
                    Data::Empty => None,
                };
                
                processed_row.push(value);
            }
            
            // Only add non-empty rows
            if processed_row.iter().any(|v| v.is_some()) {
                processed_rows.push(processed_row);
            }
        }

        info!("Processed {} data rows from sheet '{}'", processed_rows.len(), self.sheet_name);
        
        Ok(processed_rows)
    }

    /// Builds lookup tables from all sheets for VLOOKUP resolution.
    ///
    /// Creates a nested HashMap structure where:
    /// - Outer key: Sheet name
    /// - Inner key: First column value (lookup key)
    /// - Value: Vector of all column values in that row
    ///
    /// This structure supports efficient VLOOKUP resolution by providing
    /// O(1) lookup time for finding values.
    ///
    /// # Returns
    ///
    /// A HashMap containing lookup tables for all sheets
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use excel_to_json::excel_reader::ExcelReader;
    /// # fn main() -> anyhow::Result<()> {
    /// # let mut reader = ExcelReader::new("data.xlsx", "Sheet1".to_string())?;
    /// // The lookup tables structure:
    /// // {
    /// //   "Sheet1": {
    /// //     "KEY001": ["KEY001", "Value1", "Description1"],
    /// //     "KEY002": ["KEY002", "Value2", "Description2"],
    /// //   },
    /// //   "Sheet2": { ... }
    /// // }
    /// # Ok(())
    /// # }
    /// ```
    #[allow(dead_code)]
    fn build_lookup_tables(&mut self) -> Result<HashMap<String, HashMap<String, Vec<String>>>> {
        let mut tables = HashMap::new();
        
        for sheet_name in self.get_sheet_names() {
            debug!("Building lookup table for sheet: {}", sheet_name);
            
            if let Ok(range) = self.workbook.worksheet_range(&sheet_name) {
                    let mut sheet_table: HashMap<String, Vec<String>> = HashMap::new();
                    
                    for row in range.rows() {
                        if row.is_empty() {
                            continue;
                        }
                        
                        // Use first column as key
                        let key = match &row[0] {
                            Data::String(s) => s.clone(),
                            Data::Float(f) => format!("{}", f),
                            Data::Int(i) => format!("{}", i),
                            _ => continue,
                        };
                        
                        // Store entire row as values
                        let values: Vec<String> = row.iter().map(|cell| {
                            match cell {
                                Data::String(s) => s.clone(),
                                Data::Float(f) => format!("{}", f),
                                Data::Int(i) => format!("{}", i),
                                Data::Bool(b) => format!("{}", b),
                                Data::DateTime(dt) => format!("{}", dt),
                                Data::DateTimeIso(dt) => dt.clone(),
                                Data::DurationIso(d) => d.clone(),
                                _ => String::new(),
                            }
                        }).collect();
                        
                        sheet_table.insert(key, values);
                    }
                    
                    tables.insert(sheet_name.clone(), sheet_table);
            }
        }
        
        debug!("Built lookup tables for {} sheets", tables.len());
        
        Ok(tables)
    }

    /// Attempts to resolve a VLOOKUP formula.
    ///
    /// This is a placeholder for VLOOKUP formula resolution. In practice,
    /// calamine should handle formula evaluation automatically. This method
    /// is kept as a fallback for cases where formulas aren't evaluated.
    ///
    /// # Arguments
    ///
    /// * `_formula` - The VLOOKUP formula string to resolve
    /// * `_lookup_tables` - Pre-built lookup tables from all sheets
    ///
    /// # Returns
    ///
    /// * `Some(String)` - Resolved value if successful
    /// * `None` - If the formula cannot be resolved
    ///
    /// # Example Formula Format
    ///
    /// ```text
    /// =VLOOKUP(A2,Sheet2!A:C,2,FALSE)
    /// ```
    ///
    /// Where:
    /// - `A2` is the lookup value
    /// - `Sheet2!A:C` is the table array
    /// - `2` is the column index
    /// - `FALSE` indicates exact match
    ///
    /// # Note
    ///
    /// This implementation currently returns `None` as calamine
    /// handles formula evaluation. Future implementations could
    /// parse and resolve VLOOKUP formulas manually if needed.
    #[allow(dead_code)]
    fn resolve_vlookup(&self, _formula: &str, _lookup_tables: &HashMap<String, HashMap<String, Vec<String>>>) -> Option<String> {
        // This is a simplified VLOOKUP resolver
        // In practice, calamine should handle formula evaluation automatically
        // This is a fallback for cases where formulas aren't evaluated
        
        // Try to extract the lookup value and return column from the formula
        // Example: =VLOOKUP(A2,Sheet2!A:C,2,FALSE)
        
        // For now, return None to indicate unresolved
        // In a real implementation, you would parse the formula and look up the value
        None
    }

    /// Process formulas and return evaluated values when available.
    ///
    /// This is the primary method for reading Excel data. It processes the
    /// specified sheet and returns cell values with formula evaluation.
    /// The method handles various data types and converts them to strings.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<Vec<Option<String>>>)` - Processed rows with evaluated formulas
    /// * `Err` - If the sheet doesn't exist or cannot be read
    ///
    /// # Data Type Handling
    ///
    /// - **String**: Returned as-is
    /// - **Float**: Formatted as string (integers without decimals)
    /// - **Int**: Converted to string
    /// - **Bool**: Converted to "true" or "false"
    /// - **DateTime**: Formatted as string
    /// - **Error**: Returns None with a warning log
    /// - **Empty**: Returns None
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use excel_to_json::excel_reader::ExcelReader;
    ///
    /// # fn main() -> anyhow::Result<()> {
    /// let mut reader = ExcelReader::new("data.xlsx", "Cascade Fields".to_string())?;
    /// let rows = reader.read_with_formulas()?;
    ///
    /// // Process rows, skipping empty ones
    /// let non_empty_rows: Vec<_> = rows.into_iter()
    ///     .filter(|row| row.iter().any(|cell| cell.is_some()))
    ///     .collect();
    ///
    /// println!("Found {} non-empty rows", non_empty_rows.len());
    ///
    /// // Extract specific columns
    /// for row in non_empty_rows {
    ///     if let Some(main_value) = &row.get(1).and_then(|v| v.as_ref()) {
    ///         println!("Main value: {}", main_value);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Performance Note
    ///
    /// This method loads the entire sheet into memory. For very large files,
    /// consider implementing streaming or chunked processing.
    pub fn read_with_formulas(&mut self) -> Result<Vec<Vec<Option<String>>>> {
        // Check if the sheet exists
        let sheet_names = self.get_sheet_names();
        if !sheet_names.contains(&self.sheet_name) {
            anyhow::bail!(
                "Sheet '{}' not found. Available sheets: {:?}",
                self.sheet_name,
                sheet_names
            );
        }

        info!("Reading sheet with formula evaluation: {}", self.sheet_name);

        // Get both the range and formula evaluations
        let range = self.workbook
            .worksheet_range(&self.sheet_name)
            .map_err(|e| anyhow::anyhow!("Error reading sheet '{}': {}", self.sheet_name, e))?;

        // Try to get formula evaluations
        let formulas = self.workbook.worksheet_formula(&self.sheet_name);

        let mut processed_rows = Vec::new();
        let mut is_header = true;
        
        for (row_idx, row) in range.rows().enumerate() {
            // Skip header row
            if is_header {
                is_header = false;
                debug!("Skipping header row");
                continue;
            }

            let mut processed_row = Vec::new();
            
            for (col_idx, cell) in row.iter().enumerate() {
                let value = match cell {
                    Data::String(s) => Some(s.clone()),
                    Data::Float(f) => {
                        // Check if this is an integer that should be displayed without decimals
                        if f.fract() == 0.0 {
                            Some(format!("{:.0}", f))
                        } else {
                            Some(format!("{}", f))
                        }
                    },
                    Data::Int(i) => Some(format!("{}", i)),
                    Data::Bool(b) => Some(format!("{}", b)),
                    Data::DateTime(dt) => Some(format!("{}", dt)),
                    Data::DateTimeIso(dt) => Some(dt.clone()),
                    Data::DurationIso(d) => Some(d.clone()),
                    Data::Error(_) => {
                        // Check if there's a formula for this cell
                        match &formulas {
                            Ok(formula_range) => {
                                // Try to get the formula result
                                if let Some(formula_cell) = formula_range.get((row_idx, col_idx)) {
                                    Some(formula_cell.clone())
                                } else {
                                    None
                                }
                            },
                            _ => None,
                        }
                    },
                    Data::Empty => None,
                };
                
                processed_row.push(value);
            }
            
            // Only add non-empty rows
            if processed_row.iter().any(|v| v.is_some()) {
                processed_rows.push(processed_row);
            }
        }

        info!("Processed {} data rows from sheet '{}'", processed_rows.len(), self.sheet_name);
        
        Ok(processed_rows)
    }
}
