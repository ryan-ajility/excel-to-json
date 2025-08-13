# Excel to JSON Export Tool

## Project Overview

This Rust project provides a high-performance command-line tool for exporting Excel spreadsheet data to JSON format. The tool can process any Excel sheet and convert it to a structured JSON format suitable for consumption by various applications.

## Purpose

The primary goal is to provide a generic Excel-to-JSON converter that:
- Reads Excel files with formula evaluation support
- Exports data as JSON with sheet headers as object keys
- Handles any Excel sheet structure dynamically
- Provides clean, structured output for easy integration

## Features

- **Excel File Reading**: Full support for Excel files with formula evaluation
- **Dynamic Sheet Processing**: Automatically detects headers and structures data accordingly
- **JSON Output**: Exports data as an array of objects, with each row represented as an object keyed by column headers
- **Flexible Sheet Selection**: Process specific sheets or default to the first sheet
- **Error Handling**: Comprehensive error reporting with helpful messages
- **Performance**: Optimized for processing large Excel files efficiently

## Installation

### From Source

```bash
# Clone the repository
git clone <repository-url>
cd excel-to-json

# Build the project
cargo build --release

# The binary will be at target/release/excel-to-json
```

### Using Cargo

```bash
cargo install --path .
```

## Usage

### Basic Usage

```bash
# Export the first sheet of an Excel file to JSON
excel-to-json data.xlsx

# Export a specific sheet
excel-to-json data.xlsx -s "Sheet Name"

# Export multiple specific sheets
excel-to-json data.xlsx -s "Sheet1" -s "Sheet2" -s "Sheet3"

# Export all sheets from the workbook
excel-to-json data.xlsx --all-sheets
# Or use the short option
excel-to-json data.xlsx -a

# Save output to file
excel-to-json data.xlsx -f output.json

# Enable verbose logging
excel-to-json data.xlsx -v

# Show summary instead of full output
excel-to-json data.xlsx --summary
```

### Multiple Sheet Processing Examples

```bash
# Process all sheets and save to file
excel-to-json inventory.xlsx -a -f all_sheets.json

# Process only sales-related sheets
excel-to-json quarterly_report.xlsx -s "Q1 Sales" -s "Q2 Sales" -s "Q3 Sales" -s "Q4 Sales"

# Process all sheets with verbose output for debugging
excel-to-json complex_workbook.xlsx -a -v

# Get summary of all sheets without full data
excel-to-json large_file.xlsx -a --summary
```

### Command-Line Options

```
excel-to-json [OPTIONS] <INPUT_FILE>

Arguments:
  <INPUT_FILE>  Path to the Excel file to export

Options:
  -s, --sheet <SHEET>    Sheet name to process (can be specified multiple times)
  -a, --all-sheets       Process all sheets in the workbook
  -f, --file <FILE>      Output file path (defaults to stdout)
  -v, --verbose          Enable verbose logging
      --summary          Show summary instead of full output
  -h, --help             Print help information
```

Note: The `-s` and `-a` options are mutually exclusive. If neither is specified, the first sheet is processed.

## Output Format

The tool exports data in different JSON structures depending on whether you're processing single or multiple sheets:

### Single Sheet Output

When processing a single sheet (default behavior or with a single `-s` option):

```json
{
  "success": true,
  "data": [
    {
      "column1": "value1",
      "column2": "value2",
      "column3": "value3"
    },
    {
      "column1": "value4",
      "column2": "value5",
      "column3": "value6"
    }
  ],
  "metadata": {
    "total_rows_processed": 100,
    "valid_records": 100,
    "invalid_records": 0,
    "processing_time_ms": 150,
    "warnings": null
  }
}
```

### Multiple Sheets Output

When processing multiple sheets (using multiple `-s` options or `-a` for all sheets):

```json
{
  "success": true,
  "data": [
    {
      "sheet": "Sheet1",
      "rows": [
        {
          "column1": "value1",
          "column2": "value2",
          "column3": "value3"
        },
        {
          "column1": "value4",
          "column2": "value5",
          "column3": "value6"
        }
      ]
    },
    {
      "sheet": "Sheet2",
      "rows": [
        {
          "columnA": "valueA",
          "columnB": "valueB"
        }
      ]
    }
  ],
  "metadata": {
    "total_rows_processed": 200,
    "valid_records": 195,
    "invalid_records": 5,
    "processing_time_ms": 250,
    "warnings": null
  }
}
```

### Data Structure

- **`success`**: Boolean indicating if the export was successful
- **`data`**: Array of objects, where each object represents a row from the Excel sheet
  - Keys are derived from the first row (headers) of the Excel sheet
  - Values are the corresponding cell values
  - Empty cells are represented as empty strings
- **`metadata`**: Processing statistics and information
  - `total_rows_processed`: Total number of rows read from the Excel sheet
  - `valid_records`: Number of successfully processed records
  - `invalid_records`: Number of records that failed validation
  - `processing_time_ms`: Time taken to process the file in milliseconds
  - `warnings`: Array of warning messages, if any

## Language Integration Examples

### JavaScript/Node.js

```javascript
const { exec } = require('child_process');
const util = require('util');
const execPromise = util.promisify(exec);

// Single sheet processing
async function excelToJson(inputFile, sheetName = null) {
  try {
    const command = sheetName
      ? `excel-to-json "${inputFile}" -s "${sheetName}"`
      : `excel-to-json "${inputFile}"`;

    const { stdout, stderr } = await execPromise(command);

    if (stderr) {
      console.error('Warning:', stderr);
    }

    const result = JSON.parse(stdout);

    if (!result.success) {
      throw new Error(result.error);
    }

    return result.data;
  } catch (error) {
    console.error('Excel conversion failed:', error);
    throw error;
  }
}

// Multiple sheets processing
async function excelMultipleSheetsToJson(inputFile, sheetNames = null, allSheets = false) {
  try {
    let command = `excel-to-json "${inputFile}"`;
    
    if (allSheets) {
      command += ' --all-sheets';
    } else if (Array.isArray(sheetNames)) {
      for (const sheet of sheetNames) {
        command += ` -s "${sheet}"`;
      }
    }

    const { stdout, stderr } = await execPromise(command);

    if (stderr) {
      console.error('Warning:', stderr);
    }

    const result = JSON.parse(stdout);

    if (!result.success) {
      throw new Error(result.error);
    }

    // Organize data by sheet name for easier access
    const organizedData = {};
    result.data.forEach(sheetData => {
      organizedData[sheetData.sheet] = sheetData.rows;
    });

    return {
      sheets: organizedData,
      metadata: result.metadata
    };
  } catch (error) {
    console.error('Excel conversion failed:', error);
    throw error;
  }
}

// Usage examples
// Single sheet
excelToJson('sales_data.xlsx', 'Q1 Sales')
  .then(data => {
    console.log(`Converted ${data.length} rows`);
    // Process your data here
  });

// Multiple sheets
excelMultipleSheetsToJson('quarterly_report.xlsx', ['Q1 Sales', 'Q2 Sales', 'Q3 Sales'])
  .then(result => {
    console.log(`Processed ${Object.keys(result.sheets).length} sheets`);
    
    // Process each sheet
    Object.entries(result.sheets).forEach(([sheetName, rows]) => {
      console.log(`${sheetName}: ${rows.length} rows`);
      // Process sheet data...
    });
  });

// All sheets
excelMultipleSheetsToJson('workbook.xlsx', null, true)
  .then(result => {
    console.log('All sheets processed:', Object.keys(result.sheets));
  });
```

### TypeScript

```typescript
import { exec } from 'child_process';
import { promisify } from 'util';

const execAsync = promisify(exec);

interface ExcelRow {
  [key: string]: string | number | null;
}

interface ExcelResult {
  success: boolean;
  data: ExcelRow[];
  metadata: {
    total_rows_processed: number;
    valid_records: number;
    invalid_records: number;
    processing_time_ms: number;
    warnings?: string[];
  };
  error?: string;
}

async function convertExcelToJson(
  inputFile: string,
  sheetName?: string
): Promise<ExcelRow[]> {
  const args = sheetName ? `-s "${sheetName}"` : '';
  const command = `excel-to-json "${inputFile}" ${args}`;

  try {
    const { stdout } = await execAsync(command);
    const result: ExcelResult = JSON.parse(stdout);

    if (!result.success) {
      throw new Error(result.error || 'Conversion failed');
    }

    console.log(`Processed ${result.metadata.total_rows_processed} rows`);
    return result.data;
  } catch (error) {
    throw new Error(`Excel conversion failed: ${error}`);
  }
}

// Usage with async/await
async function processExcelData() {
  try {
    const data = await convertExcelToJson('inventory.xlsx', 'Products');
    data.forEach(row => {
      // Type-safe access to row properties
      console.log(row);
    });
  } catch (error) {
    console.error(error);
  }
}
```

### PHP

```php
<?php

function excelToJson($inputFile, $sheetName = null) {
    // Build command
    $command = 'excel-to-json ' . escapeshellarg($inputFile);
    if ($sheetName) {
        $command .= ' -s ' . escapeshellarg($sheetName);
    }
    $command .= ' 2>&1';

    // Execute command
    $output = shell_exec($command);

    if ($output === null) {
        throw new Exception('Failed to execute excel-to-json');
    }

    // Parse JSON output
    $result = json_decode($output, true);

    if (json_last_error() !== JSON_ERROR_NONE) {
        throw new Exception('Invalid JSON: ' . json_last_error_msg());
    }

    if (!$result['success']) {
        throw new Exception($result['error']);
    }

    return $result['data'];
}

// Usage
try {
    $data = excelToJson('customers.xlsx', 'Active Customers');

    echo "Converted " . count($data) . " rows\n";

    foreach ($data as $row) {
        // Process each row
        echo "Customer: {$row['name']}, Email: {$row['email']}\n";
    }
} catch (Exception $e) {
    echo "Error: " . $e->getMessage() . "\n";
}

// Laravel Integration Example
class ExcelImportService
{
    public function import(string $filePath, ?string $sheet = null): array
    {
        $command = sprintf(
            'excel-to-json %s %s',
            escapeshellarg($filePath),
            $sheet ? '-s ' . escapeshellarg($sheet) : ''
        );

        $output = shell_exec($command);
        $result = json_decode($output, true);

        if (!$result['success']) {
            throw new \RuntimeException($result['error']);
        }

        return $result['data'];
    }
}
```

### Rust

```rust
use std::process::Command;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
struct ExcelResult {
    success: bool,
    data: Vec<Value>,
    metadata: Metadata,
    error: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Metadata {
    total_rows_processed: usize,
    valid_records: usize,
    invalid_records: usize,
    processing_time_ms: u64,
    warnings: Option<Vec<String>>,
}

fn excel_to_json(input_file: &str, sheet_name: Option<&str>) -> Result<Vec<Value>, Box<dyn std::error::Error>> {
    let mut cmd = Command::new("excel-to-json");
    cmd.arg(input_file);

    if let Some(sheet) = sheet_name {
        cmd.arg("-s").arg(sheet);
    }

    let output = cmd.output()?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Command failed: {}", error).into());
    }

    let stdout = String::from_utf8(output.stdout)?;
    let result: ExcelResult = serde_json::from_str(&stdout)?;

    if !result.success {
        return Err(result.error.unwrap_or_else(|| "Unknown error".to_string()).into());
    }

    println!("Processed {} rows", result.metadata.total_rows_processed);
    Ok(result.data)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Convert Excel to JSON
    let data = excel_to_json("products.xlsx", Some("Inventory"))?;

    // Process the data
    for row in data {
        if let Some(product_name) = row.get("product_name") {
            println!("Product: {}", product_name);
        }
    }

    Ok(())
}
```

### Go

```go
package main

import (
    "encoding/json"
    "fmt"
    "os/exec"
    "log"
)

type ExcelResult struct {
    Success  bool                     `json:"success"`
    Data     []map[string]interface{} `json:"data"`
    Metadata Metadata                 `json:"metadata"`
    Error    string                   `json:"error,omitempty"`
}

type Metadata struct {
    TotalRowsProcessed int      `json:"total_rows_processed"`
    ValidRecords       int      `json:"valid_records"`
    InvalidRecords     int      `json:"invalid_records"`
    ProcessingTimeMs   int64    `json:"processing_time_ms"`
    Warnings          []string  `json:"warnings,omitempty"`
}

func excelToJSON(inputFile string, sheetName string) ([]map[string]interface{}, error) {
    args := []string{inputFile}
    if sheetName != "" {
        args = append(args, "-s", sheetName)
    }

    cmd := exec.Command("excel-to-json", args...)
    output, err := cmd.Output()

    if err != nil {
        return nil, fmt.Errorf("command failed: %w", err)
    }

    var result ExcelResult
    if err := json.Unmarshal(output, &result); err != nil {
        return nil, fmt.Errorf("failed to parse JSON: %w", err)
    }

    if !result.Success {
        return nil, fmt.Errorf("conversion failed: %s", result.Error)
    }

    log.Printf("Processed %d rows in %dms",
        result.Metadata.TotalRowsProcessed,
        result.Metadata.ProcessingTimeMs)

    return result.Data, nil
}

func main() {
    // Convert Excel file to JSON
    data, err := excelToJSON("sales.xlsx", "Q1 Data")
    if err != nil {
        log.Fatal(err)
    }

    // Process the data
    for i, row := range data {
        fmt.Printf("Row %d: %v\n", i+1, row)

        // Access specific fields
        if customerName, ok := row["customer_name"].(string); ok {
            fmt.Printf("Customer: %s\n", customerName)
        }
    }
}
```

### Python

```python
import subprocess
import json
from typing import List, Dict, Optional, Any

def excel_to_json(
    input_file: str,
    sheet_name: Optional[str] = None
) -> List[Dict[str, Any]]:
    """Convert Excel file to JSON using excel-to-json binary."""

    cmd = ['excel-to-json', input_file]
    if sheet_name:
        cmd.extend(['-s', sheet_name])

    try:
        result = subprocess.run(
            cmd,
            capture_output=True,
            text=True,
            check=True
        )

        data = json.loads(result.stdout)

        if not data['success']:
            raise Exception(f"Conversion failed: {data.get('error', 'Unknown error')}")

        print(f"Processed {data['metadata']['total_rows_processed']} rows")
        return data['data']

    except subprocess.CalledProcessError as e:
        raise Exception(f"Command failed: {e.stderr}")
    except json.JSONDecodeError as e:
        raise Exception(f"Invalid JSON output: {e}")

# Usage
if __name__ == "__main__":
    try:
        # Convert Excel to JSON
        rows = excel_to_json('inventory.xlsx', 'Products')

        # Process the data
        for row in rows:
            print(f"Product: {row.get('product_name')}, Stock: {row.get('quantity')}")

    except Exception as e:
        print(f"Error: {e}")
```

## Error Handling

The tool provides detailed error messages for common issues:

- File not found
- Invalid Excel format
- Sheet not found (lists available sheets)
- Formula evaluation errors
- Invalid data rows

Error responses follow this format:

```json
{
  "success": false,
  "error": "Sheet 'InvalidSheet' not found",
  "data": []
}
```

## Performance

The tool is optimized for performance:

- Processes large Excel files (10,000+ rows) efficiently
- Streaming processing to minimize memory usage
- Formula evaluation is handled efficiently
- Typical processing time: ~1-2ms per row

## Development

### Building from Source

```bash
# Development build
cargo build

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run -- test.xlsx
```

### Project Structure

```
src/
├── main.rs           # CLI entry point and argument handling
├── excel_reader.rs   # Excel file reading and formula evaluation
├── models.rs         # Data structures for records
├── processor.rs      # Core processing logic
└── output.rs         # JSON formatting and output
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contact

Ryan Ogden  
Email: ryan@ajility.dev
