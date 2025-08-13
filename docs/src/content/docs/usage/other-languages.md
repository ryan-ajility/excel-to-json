---
title: Other Languages
description: Using excel-to-json with Python, PHP, Ruby, Go, and other programming languages
---

# Other Language Integrations

excel-to-json can be integrated with any programming language that can execute shell commands. This guide provides examples for popular languages.

## Python

### Basic Usage

```python
import subprocess
import json

def excel_to_json(file_path, sheet_name=None):
    """Convert Excel file to JSON using excel-to-json CLI."""
    cmd = ['excel-to-json', file_path]
    
    if sheet_name:
        cmd.extend(['-s', sheet_name])
    
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, check=True)
        return json.loads(result.stdout)
    except subprocess.CalledProcessError as e:
        raise Exception(f"Excel conversion failed: {e.stderr}")
    except json.JSONDecodeError as e:
        raise Exception(f"Failed to parse JSON output: {e}")

# Usage
data = excel_to_json('inventory.xlsx')
if data['success']:
    print(f"Processed {data['metadata']['total_rows_processed']} rows")
    for row in data['data']:
        print(row)
```

### Advanced Python Class

```python
import subprocess
import json
import os
from pathlib import Path
from typing import Dict, List, Optional, Any

class ExcelConverter:
    """Python wrapper for excel-to-json CLI tool."""
    
    def __init__(self, command: str = 'excel-to-json'):
        self.command = command
    
    def convert(self, file_path: str, sheet: Optional[str] = None) -> Dict[str, Any]:
        """Convert Excel file to JSON."""
        if not Path(file_path).exists():
            raise FileNotFoundError(f"File not found: {file_path}")
        
        cmd = [self.command, file_path]
        if sheet:
            cmd.extend(['-s', sheet])
        
        result = subprocess.run(cmd, capture_output=True, text=True)
        if result.returncode != 0:
            raise Exception(f"Conversion failed: {result.stderr}")
        
        data = json.loads(result.stdout)
        if not data.get('success'):
            raise Exception(data.get('error', 'Unknown conversion error'))
        
        return data
    
    def list_sheets(self, file_path: str) -> List[str]:
        """List available sheets in Excel file."""
        cmd = [self.command, file_path, '--list-sheets']
        result = subprocess.run(cmd, capture_output=True, text=True, check=True)
        
        sheets = []
        for line in result.stdout.splitlines():
            if line.strip() and '.' in line:
                sheet_name = line.split('.', 1)[1].strip()
                sheets.append(sheet_name)
        return sheets
    
    def convert_all_sheets(self, file_path: str) -> Dict[str, Any]:
        """Convert all sheets to JSON."""
        cmd = [self.command, file_path, '--all-sheets']
        result = subprocess.run(cmd, capture_output=True, text=True, check=True)
        return json.loads(result.stdout)

# Usage example
converter = ExcelConverter()

# Convert single sheet
try:
    result = converter.convert('data.xlsx', sheet='Sales')
    print(f"Converted {len(result['data'])} rows")
except Exception as e:
    print(f"Error: {e}")

# List and process all sheets
sheets = converter.list_sheets('workbook.xlsx')
for sheet in sheets:
    data = converter.convert('workbook.xlsx', sheet=sheet)
    print(f"Sheet '{sheet}': {len(data['data'])} rows")
```

## PHP

### Basic Usage

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
    $output = [];
    $returnCode = 0;
    exec($command, $output, $returnCode);
    
    if ($returnCode !== 0) {
        throw new Exception('Excel conversion failed: ' . implode("\n", $output));
    }
    
    // Parse JSON output
    $jsonString = implode("\n", $output);
    $result = json_decode($jsonString, true);
    
    if (json_last_error() !== JSON_ERROR_NONE) {
        throw new Exception('Failed to parse JSON: ' . json_last_error_msg());
    }
    
    if (!$result['success']) {
        throw new Exception($result['error'] ?? 'Conversion failed');
    }
    
    return $result;
}

// Usage
try {
    $data = excelToJson('inventory.xlsx', 'Products');
    echo "Processed {$data['metadata']['total_rows_processed']} rows\n";
    
    foreach ($data['data'] as $row) {
        echo "Product: {$row['ProductName']}, Price: {$row['Price']}\n";
    }
} catch (Exception $e) {
    echo "Error: " . $e->getMessage() . "\n";
}
?>
```

### PHP Class Implementation

```php
<?php
class ExcelConverter {
    private $command;
    
    public function __construct($command = 'excel-to-json') {
        $this->command = $command;
    }
    
    public function convert($filePath, $options = []) {
        if (!file_exists($filePath)) {
            throw new Exception("File not found: $filePath");
        }
        
        $cmd = $this->command . ' ' . escapeshellarg($filePath);
        
        if (!empty($options['sheet'])) {
            $cmd .= ' -s ' . escapeshellarg($options['sheet']);
        }
        
        if (!empty($options['allSheets'])) {
            $cmd .= ' --all-sheets';
        }
        
        $output = shell_exec($cmd . ' 2>&1');
        
        if ($output === null) {
            throw new Exception('Command execution failed');
        }
        
        $result = json_decode($output, true);
        
        if (json_last_error() !== JSON_ERROR_NONE) {
            throw new Exception('JSON parsing failed: ' . json_last_error_msg());
        }
        
        return $result;
    }
    
    public function listSheets($filePath) {
        $cmd = $this->command . ' ' . escapeshellarg($filePath) . ' --list-sheets';
        $output = shell_exec($cmd);
        
        $sheets = [];
        $lines = explode("\n", trim($output));
        
        foreach ($lines as $line) {
            if (preg_match('/^\d+\.\s*(.+)$/', $line, $matches)) {
                $sheets[] = trim($matches[1]);
            }
        }
        
        return $sheets;
    }
    
    public function convertAllSheets($filePath) {
        return $this->convert($filePath, ['allSheets' => true]);
    }
}

// Usage with error handling
$converter = new ExcelConverter();

try {
    // List sheets
    $sheets = $converter->listSheets('workbook.xlsx');
    echo "Available sheets: " . implode(', ', $sheets) . "\n";
    
    // Convert specific sheet
    $result = $converter->convert('workbook.xlsx', ['sheet' => 'Sales']);
    
    if ($result['success']) {
        echo "Successfully converted {$result['metadata']['valid_records']} records\n";
        
        // Process data
        foreach ($result['data'] as $row) {
            // Process each row
            print_r($row);
        }
    }
} catch (Exception $e) {
    echo "Error: " . $e->getMessage() . "\n";
}
?>
```

## Ruby

### Basic Usage

```ruby
require 'json'
require 'open3'

def excel_to_json(file_path, sheet_name = nil)
  cmd = ['excel-to-json', file_path]
  cmd += ['-s', sheet_name] if sheet_name
  
  stdout, stderr, status = Open3.capture3(*cmd)
  
  unless status.success?
    raise "Excel conversion failed: #{stderr}"
  end
  
  result = JSON.parse(stdout)
  
  unless result['success']
    raise result['error'] || 'Conversion failed'
  end
  
  result
end

# Usage
begin
  data = excel_to_json('inventory.xlsx', 'Products')
  puts "Processed #{data['metadata']['total_rows_processed']} rows"
  
  data['data'].each do |row|
    puts "#{row['ProductName']}: $#{row['Price']}"
  end
rescue => e
  puts "Error: #{e.message}"
end
```

### Ruby Class

```ruby
require 'json'
require 'open3'

class ExcelConverter
  def initialize(command = 'excel-to-json')
    @command = command
  end
  
  def convert(file_path, sheet: nil, all_sheets: false)
    raise "File not found: #{file_path}" unless File.exist?(file_path)
    
    cmd = [@command, file_path]
    cmd += ['-s', sheet] if sheet
    cmd << '--all-sheets' if all_sheets
    
    stdout, stderr, status = Open3.capture3(*cmd)
    
    raise "Conversion failed: #{stderr}" unless status.success?
    
    JSON.parse(stdout)
  end
  
  def list_sheets(file_path)
    cmd = [@command, file_path, '--list-sheets']
    stdout, _, status = Open3.capture3(*cmd)
    
    raise "Failed to list sheets" unless status.success?
    
    sheets = []
    stdout.each_line do |line|
      if match = line.match(/^\d+\.\s*(.+)$/)
        sheets << match[1].strip
      end
    end
    
    sheets
  end
  
  def convert_all_sheets(file_path)
    convert(file_path, all_sheets: true)
  end
end

# Usage
converter = ExcelConverter.new

begin
  # List sheets
  sheets = converter.list_sheets('workbook.xlsx')
  puts "Sheets: #{sheets.join(', ')}"
  
  # Convert specific sheet
  result = converter.convert('workbook.xlsx', sheet: 'Sales')
  puts "Converted #{result['data'].length} rows"
  
  # Convert all sheets
  all_data = converter.convert_all_sheets('workbook.xlsx')
  all_data['sheets'].each do |sheet_name, sheet_data|
    puts "#{sheet_name}: #{sheet_data['data'].length} rows"
  end
rescue => e
  puts "Error: #{e.message}"
end
```

## Go

### Basic Usage

```go
package main

import (
    "encoding/json"
    "fmt"
    "os/exec"
)

type ExcelResult struct {
    Success  bool                   `json:"success"`
    Data     []map[string]interface{} `json:"data"`
    Metadata struct {
        TotalRowsProcessed int      `json:"total_rows_processed"`
        ValidRecords       int      `json:"valid_records"`
        InvalidRecords     int      `json:"invalid_records"`
        ProcessingTimeMs   int      `json:"processing_time_ms"`
        Warnings          []string `json:"warnings,omitempty"`
    } `json:"metadata"`
    Error string `json:"error,omitempty"`
}

func excelToJSON(filePath string, sheetName string) (*ExcelResult, error) {
    args := []string{filePath}
    if sheetName != "" {
        args = append(args, "-s", sheetName)
    }
    
    cmd := exec.Command("excel-to-json", args...)
    output, err := cmd.Output()
    if err != nil {
        return nil, fmt.Errorf("command failed: %v", err)
    }
    
    var result ExcelResult
    if err := json.Unmarshal(output, &result); err != nil {
        return nil, fmt.Errorf("failed to parse JSON: %v", err)
    }
    
    if !result.Success {
        return nil, fmt.Errorf("conversion failed: %s", result.Error)
    }
    
    return &result, nil
}

func main() {
    result, err := excelToJSON("inventory.xlsx", "Products")
    if err != nil {
        fmt.Printf("Error: %v\n", err)
        return
    }
    
    fmt.Printf("Processed %d rows\n", result.Metadata.TotalRowsProcessed)
    
    for _, row := range result.Data {
        fmt.Printf("Row: %v\n", row)
    }
}
```

## Rust

If you want to use excel-to-json from another Rust application:

```rust
use std::process::Command;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct ExcelResult {
    success: bool,
    data: Vec<serde_json::Value>,
    metadata: Metadata,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Metadata {
    total_rows_processed: usize,
    valid_records: usize,
    invalid_records: usize,
    processing_time_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    warnings: Option<Vec<String>>,
}

fn excel_to_json(file_path: &str, sheet_name: Option<&str>) -> Result<ExcelResult, Box<dyn std::error::Error>> {
    let mut cmd = Command::new("excel-to-json");
    cmd.arg(file_path);
    
    if let Some(sheet) = sheet_name {
        cmd.arg("-s").arg(sheet);
    }
    
    let output = cmd.output()?;
    
    if !output.status.success() {
        return Err(format!("Command failed: {}", String::from_utf8_lossy(&output.stderr)).into());
    }
    
    let result: ExcelResult = serde_json::from_slice(&output.stdout)?;
    
    if !result.success {
        return Err(result.error.unwrap_or_else(|| "Unknown error".to_string()).into());
    }
    
    Ok(result)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = excel_to_json("inventory.xlsx", Some("Products"))?;
    
    println!("Processed {} rows", result.metadata.total_rows_processed);
    
    for row in &result.data {
        println!("{:?}", row);
    }
    
    Ok(())
}
```

## Shell Scripts (Bash)

For automation and scripting:

```bash
#!/bin/bash

# Function to convert Excel to JSON
excel_to_json() {
    local file="$1"
    local sheet="$2"
    
    if [ ! -f "$file" ]; then
        echo "Error: File not found: $file" >&2
        return 1
    fi
    
    if [ -n "$sheet" ]; then
        excel-to-json "$file" -s "$sheet"
    else
        excel-to-json "$file"
    fi
}

# Process multiple files
process_excel_files() {
    local dir="${1:-.}"
    
    for file in "$dir"/*.xlsx; do
        [ -f "$file" ] || continue
        
        echo "Processing: $file"
        output="${file%.xlsx}.json"
        
        if excel-to-json "$file" > "$output"; then
            echo "  ✓ Saved to: $output"
        else
            echo "  ✗ Failed to process" >&2
        fi
    done
}

# Extract specific data using jq
extract_products() {
    local file="$1"
    
    excel-to-json "$file" | jq -r '
        .data[] | 
        select(.InStock == true) |
        "\(.ProductName): $\(.Price)"
    '
}

# Usage examples
echo "Converting single file:"
excel_to_json "inventory.xlsx" "Products" | jq '.metadata'

echo -e "\nProcessing directory:"
process_excel_files "./excel_files"

echo -e "\nExtracting in-stock products:"
extract_products "inventory.xlsx"
```

## PowerShell

For Windows automation:

```powershell
function Convert-ExcelToJson {
    param(
        [Parameter(Mandatory=$true)]
        [string]$FilePath,
        
        [string]$SheetName = $null
    )
    
    if (-not (Test-Path $FilePath)) {
        throw "File not found: $FilePath"
    }
    
    $args = @($FilePath)
    if ($SheetName) {
        $args += "-s", $SheetName
    }
    
    $output = & excel-to-json @args 2>&1
    
    if ($LASTEXITCODE -ne 0) {
        throw "Conversion failed: $output"
    }
    
    return $output | ConvertFrom-Json
}

# Usage
try {
    $result = Convert-ExcelToJson -FilePath "inventory.xlsx" -SheetName "Products"
    
    Write-Host "Processed $($result.metadata.total_rows_processed) rows"
    
    foreach ($row in $result.data) {
        Write-Host "$($row.ProductName): $$($row.Price)"
    }
} catch {
    Write-Error $_.Exception.Message
}
```

## Best Practices Across Languages

1. **Error Handling**: Always check for command execution errors and JSON parsing errors
2. **File Validation**: Verify file existence before attempting conversion
3. **Encoding**: Ensure proper UTF-8 handling for international characters
4. **Timeouts**: Set appropriate timeouts for large file conversions
5. **Memory Management**: For large files, consider streaming or chunking approaches
6. **Logging**: Log conversion metrics from metadata for monitoring

## Next Steps

- Review [CLI Options](/reference/cli-options/) for all available command-line options
- Check [Output Format](/reference/output-format/) documentation for detailed JSON structure
- See [Error Handling](/reference/error-handling/) for comprehensive error management
