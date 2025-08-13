---
title: Error Handling
description: Comprehensive guide to error handling and troubleshooting
---

# Error Handling Reference

excel-to-json provides comprehensive error handling with clear messages and recovery strategies. This guide covers all error scenarios and how to handle them.

## Error Response Format

When an error occurs, excel-to-json returns a structured error response:

```json
{
  "success": false,
  "error": "Human-readable error message",
  "error_code": "MACHINE_READABLE_CODE",
  "details": {
    "file": "data.xlsx",
    "sheet": "Sales",
    "row": 42,
    "column": "Price"
  }
}
```

## Error Codes

### File Errors

#### `FILE_NOT_FOUND`

The specified Excel file doesn't exist.

```json
{
  "error_code": "FILE_NOT_FOUND",
  "error": "File not found: data.xlsx"
}
```

**Resolution:**
- Check file path and spelling
- Ensure file exists
- Verify permissions

#### `FILE_ACCESS_DENIED`

Cannot read the Excel file due to permissions.

```json
{
  "error_code": "FILE_ACCESS_DENIED",
  "error": "Permission denied: data.xlsx"
}
```

**Resolution:**
- Check file permissions: `ls -la data.xlsx`
- Ensure file is not locked by another program
- Run with appropriate permissions

#### `INVALID_FILE_FORMAT`

File is not a valid Excel format.

```json
{
  "error_code": "INVALID_FILE_FORMAT",
  "error": "Not a valid Excel file: data.txt"
}
```

**Resolution:**
- Verify file extension (.xlsx, .xls, .xlsm)
- Check if file is corrupted
- Ensure file is actually an Excel file

#### `FILE_CORRUPTED`

Excel file is corrupted or damaged.

```json
{
  "error_code": "FILE_CORRUPTED",
  "error": "Cannot read Excel file: corrupted data"
}
```

**Resolution:**
- Try opening in Excel to verify
- Recover using Excel's repair feature
- Use a backup copy

### Sheet Errors

#### `SHEET_NOT_FOUND`

Specified sheet doesn't exist in workbook.

```json
{
  "error_code": "SHEET_NOT_FOUND",
  "error": "Sheet 'Sales' not found in workbook"
}
```

**Resolution:**
- List available sheets: `excel-to-json file.xlsx --list-sheets`
- Check sheet name spelling and case
- Use correct sheet name

#### `NO_SHEETS_FOUND`

Workbook contains no sheets.

```json
{
  "error_code": "NO_SHEETS_FOUND",
  "error": "Workbook contains no sheets"
}
```

**Resolution:**
- Verify workbook in Excel
- Check if sheets are hidden
- Ensure file is not empty

### Data Errors

#### `NO_DATA_FOUND`

Sheet contains no data rows.

```json
{
  "error_code": "NO_DATA_FOUND",
  "error": "No data found in sheet",
  "details": {
    "sheet": "Products",
    "headers_found": true,
    "data_rows": 0
  }
}
```

**Resolution:**
- Check if data exists below header row
- Verify `--start-row` setting
- Ensure data is not hidden or filtered

#### `INVALID_HEADERS`

Header row contains invalid or duplicate values.

```json
{
  "error_code": "INVALID_HEADERS",
  "error": "Invalid headers: duplicate column names",
  "details": {
    "duplicates": ["Name", "Email"],
    "empty_headers": 2
  }
}
```

**Resolution:**
- Ensure unique header names
- Fill in empty header cells
- Use `--custom-headers` to override

#### `TYPE_CONVERSION_ERROR`

Failed to convert cell value to expected type.

```json
{
  "error_code": "TYPE_CONVERSION_ERROR",
  "error": "Cannot convert 'abc' to number",
  "details": {
    "row": 10,
    "column": "Price",
    "value": "abc",
    "expected_type": "number"
  }
}
```

**Resolution:**
- Check data types in Excel
- Use `--force-text` to treat all as text
- Clean data before conversion

### Processing Errors

#### `MEMORY_ERROR`

Insufficient memory to process file.

```json
{
  "error_code": "MEMORY_ERROR",
  "error": "Out of memory processing large file"
}
```

**Resolution:**
- Use `--max-rows` to limit processing
- Process sheets individually
- Increase system memory limits

#### `TIMEOUT_ERROR`

Processing exceeded time limit.

```json
{
  "error_code": "TIMEOUT_ERROR",
  "error": "Processing timeout after 60 seconds"
}
```

**Resolution:**
- Process smaller chunks
- Increase timeout: `export EXCEL_TO_JSON_TIMEOUT=120`
- Optimize Excel file (remove formatting, formulas)

#### `ENCODING_ERROR`

Character encoding issues.

```json
{
  "error_code": "ENCODING_ERROR",
  "error": "Invalid UTF-8 sequence in cell",
  "details": {
    "row": 25,
    "column": "Description"
  }
}
```

**Resolution:**
- Save Excel file with UTF-8 encoding
- Use `--ignore-errors` to skip problematic rows
- Clean special characters in Excel

## Exit Codes

excel-to-json uses standard exit codes for scripting:

| Code | Meaning | Description |
|------|---------|-------------|
| 0 | Success | Conversion completed successfully |
| 1 | General Error | Unspecified error occurred |
| 2 | Invalid Arguments | Command-line arguments invalid |
| 3 | File Not Found | Input file doesn't exist |
| 4 | Parse Error | Failed to parse Excel file |
| 5 | Timeout | Processing exceeded time limit |
| 6 | Memory Error | Out of memory |
| 7 | Permission Denied | Cannot access file |
| 8 | Invalid Format | Not a valid Excel file |

### Using Exit Codes in Scripts

```bash
#!/bin/bash
excel-to-json data.xlsx > output.json
exit_code=$?

case $exit_code in
  0)
    echo "✓ Success"
    ;;
  3)
    echo "✗ File not found"
    exit 1
    ;;
  4)
    echo "✗ Failed to parse Excel file"
    exit 1
    ;;
  5)
    echo "✗ Processing timeout"
    exit 1
    ;;
  *)
    echo "✗ Unknown error (code: $exit_code)"
    exit 1
    ;;
esac
```

## Error Recovery Strategies

### Partial Processing

Continue processing despite errors:

```bash
excel-to-json data.xlsx --ignore-errors
```

Result includes successful rows with error summary:
```json
{
  "success": true,
  "data": [...],
  "metadata": {
    "valid_records": 95,
    "invalid_records": 5,
    "errors": [
      "Row 10: Type conversion error",
      "Row 25: Missing required field"
    ]
  }
}
```

### Validation Mode

Check for errors without processing:

```bash
excel-to-json data.xlsx --validate
```

Returns validation report:
```json
{
  "valid": false,
  "errors": [
    {
      "type": "TYPE_ERROR",
      "row": 10,
      "column": "Price",
      "message": "Expected number, got text"
    }
  ]
}
```

### Error Limits

Stop after a certain number of errors:

```bash
excel-to-json data.xlsx --max-errors 10
```

## Common Error Scenarios

### Scenario 1: Mixed Data Types

**Problem:** Column contains mixed types (numbers and text).

**Error:**
```json
{
  "error_code": "TYPE_CONVERSION_ERROR",
  "error": "Inconsistent data types in column 'ID'"
}
```

**Solutions:**
1. Use `--force-text` to treat all as text
2. Clean data in Excel first
3. Use `--ignore-errors` to skip problematic rows

### Scenario 2: Special Characters

**Problem:** Excel contains special Unicode characters.

**Error:**
```json
{
  "error_code": "ENCODING_ERROR",
  "error": "Invalid character encoding"
}
```

**Solutions:**
1. Save Excel with UTF-8 encoding
2. Remove special characters
3. Use encoding conversion tools

### Scenario 3: Large Files

**Problem:** File too large to process.

**Error:**
```json
{
  "error_code": "MEMORY_ERROR",
  "error": "Insufficient memory"
}
```

**Solutions:**
1. Process in chunks: `--max-rows 1000`
2. Process sheets separately
3. Increase memory limits
4. Use streaming mode (if available)

### Scenario 4: Formula Errors

**Problem:** Excel formulas return errors (#DIV/0!, #REF!).

**Error:**
```json
{
  "error_code": "FORMULA_ERROR",
  "error": "Formula evaluation failed"
}
```

**Solutions:**
1. Convert formulas to values in Excel
2. Use `--ignore-errors` to skip error cells
3. Fix formulas before conversion

## Error Handling in Applications

### JavaScript/Node.js

```javascript
const { exec } = require('child_process');

function handleExcelConversion(file) {
  exec(`excel-to-json ${file}`, (error, stdout, stderr) => {
    if (error) {
      const result = JSON.parse(stdout || '{}');
      
      switch(result.error_code) {
        case 'FILE_NOT_FOUND':
          console.error('File does not exist');
          break;
        case 'SHEET_NOT_FOUND':
          console.error('Sheet not found');
          break;
        case 'TYPE_CONVERSION_ERROR':
          console.error('Data type issues');
          // Retry with --force-text
          exec(`excel-to-json ${file} --force-text`, ...);
          break;
        default:
          console.error('Unknown error:', result.error);
      }
      return;
    }
    
    // Success
    const data = JSON.parse(stdout);
    processData(data);
  });
}
```

### Python

```python
import subprocess
import json

def convert_excel(file_path):
    try:
        result = subprocess.run(
            ['excel-to-json', file_path],
            capture_output=True,
            text=True,
            check=False
        )
        
        data = json.loads(result.stdout)
        
        if not data.get('success'):
            error_code = data.get('error_code')
            
            if error_code == 'FILE_NOT_FOUND':
                raise FileNotFoundError(f"File not found: {file_path}")
            elif error_code == 'SHEET_NOT_FOUND':
                # List sheets and retry
                sheets = list_sheets(file_path)
                if sheets:
                    return convert_excel_sheet(file_path, sheets[0])
            else:
                raise Exception(data.get('error'))
        
        return data
        
    except json.JSONDecodeError:
        raise Exception("Failed to parse converter output")
```

## Debugging

### Enable Debug Mode

Get detailed error information:

```bash
excel-to-json data.xlsx --debug
```

### Verbose Logging

```bash
export EXCEL_TO_JSON_DEBUG=1
excel-to-json data.xlsx 2> debug.log
```

### Common Debug Checks

1. **File accessibility:**
   ```bash
   file data.xlsx
   ls -la data.xlsx
   ```

2. **Excel file info:**
   ```bash
   excel-to-json data.xlsx --list-sheets
   excel-to-json data.xlsx --validate
   ```

3. **Test with sample:**
   ```bash
   excel-to-json data.xlsx --max-rows 10
   ```

## Best Practices

1. **Always check `success` field** before processing data
2. **Log error codes** for debugging
3. **Implement retry logic** for transient errors
4. **Validate data** before production use
5. **Use `--strict` mode** for critical data
6. **Handle partial failures** gracefully
7. **Set appropriate timeouts** for large files
8. **Monitor error rates** in production

## Next Steps

- Review [CLI Options](/reference/cli-options/) for error handling flags
- See [Output Format](/reference/output-format/) for error response structure
- Check language-specific guides for error handling examples
