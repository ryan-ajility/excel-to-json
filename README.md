# Cascade Fields Import Tool

## Project Overview

This Rust project creates a high-performance binary executable for importing Cascade Fields data from an Excel spreadsheet into a database. The tool is designed to be invoked from a PHP Laravel application, specifically from `/Users/ajility/Projects/jde-tables/app/ItemMasters/Importers/CascadeFieldImport.php`.

## Purpose

The primary goal is to process Excel spreadsheets containing Cascade Fields data with VLOOKUP formulas, resolve those formulas to their actual display values, and prepare the data for database insertion. This Rust implementation provides superior performance for handling large Excel files with complex formula resolution compared to PHP-based solutions.

## Data Structure

### Input: Excel Spreadsheet
The tool processes the "Cascade Fields" sheet from Excel files (e.g., `Item Master Field Values.xlsx`) containing the following columns:
- `main_label`
- `main_value`
- `main_description`
- `sub_label`
- `sub_value`
- `sub_description`
- `major_label`
- `major_value`
- `major_description`
- `minor_label`
- `minor_value`
- `minor_description`

**Note:** The value and description fields contain VLOOKUP formulas that must be resolved to their actual display values.

### Output: Database Table (`cascade_fields`)
The processed data maps directly to the `cascade_fields` database table with the same field structure.

## Implementation Plan

### 1. Core Dependencies
```toml
[dependencies]
calamine = "0.25"          # Excel file reading with formula evaluation
serde = "1.0"              # Serialization/deserialization
serde_json = "1.0"         # JSON output for PHP integration
clap = "4.5"               # Command-line argument parsing
anyhow = "1.0"             # Error handling
tracing = "0.2"            # Logging
tracing-subscriber = "0.3" # Log formatting
```

### 2. Architecture

#### Module Structure
```
src/
├── main.rs           # Entry point and CLI handling
├── excel_reader.rs   # Excel file reading and VLOOKUP resolution
├── models.rs         # Data structures for Cascade Fields
├── processor.rs      # Core business logic for data transformation
└── output.rs         # JSON formatting for PHP consumption
```

#### Key Components

**A. Excel Reader Module (`excel_reader.rs`)**
- Open and parse Excel files using calamine
- Navigate to the "Cascade Fields" sheet
- Resolve VLOOKUP formulas to their display values
- Handle formula evaluation errors gracefully

**B. Data Models (`models.rs`)**
- Define `CascadeField` struct matching database schema
- Implement validation logic for required fields
- Support serialization to JSON

**C. Processor Module (`processor.rs`)**
- Transform raw Excel data to `CascadeField` records
- Handle missing or empty values according to business rules
- Validate composite keys (main_value, sub_value, major_value, minor_value)
- Filter out incomplete records

**D. Output Module (`output.rs`)**
- Format processed data as JSON for PHP consumption
- Include metadata (row count, processing time, errors)
- Support different output formats (JSON, CSV) for flexibility

### 3. Command-Line Interface

The binary will accept the following arguments:
```bash
import_cascade_fields [OPTIONS] <INPUT_FILE>

Arguments:
  <INPUT_FILE>  Path to the Excel file to import

Options:
  -s, --sheet <SHEET>      Sheet name (default: "Cascade Fields")
  -o, --output <FORMAT>    Output format: json|csv (default: json)
  -v, --verbose            Enable verbose logging
  -h, --help               Print help information
```

### 4. Integration with PHP

The PHP code will invoke the binary and parse its output:

```php
// Example PHP integration
$command = sprintf(
    '%s %s',
    '/path/to/import_cascade_fields',
    escapeshellarg($excelFilePath)
);

$output = shell_exec($command);
$data = json_decode($output, true);

if ($data['success']) {
    // Process $data['records'] for database insertion
    foreach ($data['records'] as $record) {
        // Upsert logic
    }
}
```

### 5. Error Handling

The tool will handle various error scenarios:
- File not found or inaccessible
- Invalid Excel format
- Missing "Cascade Fields" sheet
- VLOOKUP resolution failures
- Invalid or incomplete data rows

Errors will be reported in a structured format:
```json
{
  "success": false,
  "error": "Sheet 'Cascade Fields' not found",
  "details": {
    "file": "path/to/file.xlsx",
    "available_sheets": ["Sheet1", "Sheet2"]
  }
}
```

### 6. Performance Considerations

- **Streaming Processing**: Process Excel rows in chunks to minimize memory usage
- **Parallel VLOOKUP Resolution**: Utilize multiple threads for formula evaluation
- **Efficient Data Structures**: Use appropriate data structures for lookup tables
- **Memory Management**: Clear intermediate data structures as soon as they're no longer needed

### 7. Testing Strategy

- **Unit Tests**: Test each module independently
- **Integration Tests**: Test end-to-end Excel processing
- **Sample Data**: Create test Excel files with various VLOOKUP scenarios
- **Benchmarks**: Compare performance against PHP implementation

### 8. Build and Distribution

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Install locally
cargo install --path .
```

The release binary will be placed at `target/release/import_cascade_fields` and should be copied to a location accessible by the PHP application.

## Usage Example

```bash
# Process an Excel file and output JSON
./import_cascade_fields "/path/to/Item Master Field Values.xlsx"

# Process with verbose logging
./import_cascade_fields -v "/path/to/Item Master Field Values.xlsx"

# Process a specific sheet
./import_cascade_fields -s "Custom Sheet" "/path/to/file.xlsx"
```

## Success Criteria

1. **Accurate VLOOKUP Resolution**: All VLOOKUP formulas are correctly resolved to their display values
2. **Data Integrity**: Composite keys (main_value, sub_value, major_value, minor_value) are preserved
3. **Performance**: Process large Excel files (10,000+ rows) in under 10 seconds
4. **Reliability**: Graceful error handling with informative error messages
5. **Integration**: Seamless integration with the existing PHP Laravel application

## Future Enhancements

- Support for additional Excel formula types beyond VLOOKUP
- Database direct insertion option (bypass PHP)
- Web service mode for REST API integration
- Progress reporting for long-running imports
- Caching of resolved VLOOKUP values for repeated imports

## License

[Specify your license here]

## Contact

[Your contact information]
