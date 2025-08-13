---
title: Command Line
description: Complete guide to using excel-to-json from the command line
---

# Command Line Usage

excel-to-json is primarily a command-line tool designed for ease of use and integration into various workflows. This guide covers all command-line features and options.

## Basic Syntax

```bash
excel-to-json [OPTIONS] <FILE>
```

- `FILE`: Path to the Excel file to convert (required)
- `OPTIONS`: Various flags and parameters to control conversion behavior

## Common Usage Patterns

### Simple Conversion

Convert an Excel file and output JSON to stdout:

```bash
excel-to-json data.xlsx
```

### Save to File

Redirect output to save JSON to a file:

```bash
excel-to-json data.xlsx > output.json
```

### Pretty Print

Use with `jq` for formatted output:

```bash
excel-to-json data.xlsx | jq '.'
```

## Command Options

### Sheet Selection

#### `-s, --sheet <SHEET_NAME>`

Convert a specific sheet by name:

```bash
excel-to-json workbook.xlsx -s "Sales Data"
excel-to-json workbook.xlsx --sheet "Products"
```

#### `--all-sheets`

Convert all sheets in the workbook:

```bash
excel-to-json workbook.xlsx --all-sheets
```

Output structure for all sheets:
```json
{
  "success": true,
  "sheets": {
    "Sheet1": { "data": [...], "metadata": {...} },
    "Sheet2": { "data": [...], "metadata": {...} }
  }
}
```

#### `--list-sheets`

List all available sheets without converting:

```bash
excel-to-json workbook.xlsx --list-sheets
```

Output:
```
Available sheets:
  1. Sales
  2. Inventory
  3. Customers
```

### Output Control

#### `--quiet`

Suppress informational messages, output only JSON:

```bash
excel-to-json data.xlsx --quiet
```

#### `--pretty`

Format JSON output with indentation:

```bash
excel-to-json data.xlsx --pretty
```

#### `--compact`

Minimize JSON output (no whitespace):

```bash
excel-to-json data.xlsx --compact
```

### Data Processing

#### `--skip-empty-rows`

Skip rows that are completely empty:

```bash
excel-to-json data.xlsx --skip-empty-rows
```

#### `--skip-empty-columns`

Skip columns that have no header:

```bash
excel-to-json data.xlsx --skip-empty-columns
```

#### `--max-rows <NUMBER>`

Limit the number of rows to process:

```bash
excel-to-json large_file.xlsx --max-rows 1000
```

#### `--start-row <NUMBER>`

Start processing from a specific row (1-indexed):

```bash
excel-to-json data.xlsx --start-row 5
```

### Header Options

#### `--no-header`

Treat the first row as data, not headers:

```bash
excel-to-json data.xlsx --no-header
```

Output uses column indices as keys:
```json
{
  "data": [
    { "A": "value1", "B": "value2", "C": "value3" }
  ]
}
```

#### `--header-row <NUMBER>`

Specify which row contains headers (1-indexed):

```bash
excel-to-json data.xlsx --header-row 3
```

### Type Handling

#### `--force-text`

Treat all values as text (no type detection):

```bash
excel-to-json data.xlsx --force-text
```

#### `--date-format <FORMAT>`

Specify date output format:

```bash
excel-to-json data.xlsx --date-format "%Y-%m-%d"
excel-to-json data.xlsx --date-format "iso8601"
```

#### `--number-format <FORMAT>`

Control number formatting:

```bash
excel-to_json data.xlsx --number-format "%.2f"  # Two decimal places
```

### Error Handling

#### `--strict`

Stop on first error (default is to continue):

```bash
excel-to-json data.xlsx --strict
```

#### `--ignore-errors`

Continue processing and exclude invalid rows:

```bash
excel-to-json data.xlsx --ignore-errors
```

### Information Options

#### `-h, --help`

Display help information:

```bash
excel-to-json --help
```

#### `-V, --version`

Display version information:

```bash
excel-to-json --version
```

## Advanced Examples

### Processing Multiple Sheets

Convert each sheet to a separate file:

```bash
#!/bin/bash
sheets=$(excel-to-json workbook.xlsx --list-sheets | grep -E '^\s*[0-9]+\.' | sed 's/^.*\. //')

while IFS= read -r sheet; do
  excel-to-json workbook.xlsx -s "$sheet" > "${sheet// /_}.json"
  echo "Converted sheet: $sheet"
done <<< "$sheets"
```

### Filtering Data

Convert and filter in one command:

```bash
# Only get rows where status is "active"
excel-to-json data.xlsx | jq '.data | map(select(.status == "active"))'

# Get first 10 rows
excel-to-json data.xlsx | jq '.data | .[0:10]'

# Count rows by category
excel-to-json data.xlsx | jq '.data | group_by(.category) | map({category: .[0].category, count: length})'
```

### Data Validation

Check for conversion issues:

```bash
#!/bin/bash
result=$(excel-to-json data.xlsx)
success=$(echo "$result" | jq -r '.success')
invalid=$(echo "$result" | jq -r '.metadata.invalid_records')

if [ "$success" = "true" ] && [ "$invalid" = "0" ]; then
  echo "✓ Conversion successful with no errors"
  echo "$result" > output.json
else
  echo "✗ Conversion had issues:"
  echo "$result" | jq '.metadata'
  exit 1
fi
```

### Batch Processing

Process all Excel files in a directory:

```bash
#!/bin/bash
find . -name "*.xlsx" -type f | while read -r file; do
  output="${file%.xlsx}.json"
  
  if excel-to-json "$file" --quiet > "$output"; then
    echo "✓ Converted: $file → $output"
  else
    echo "✗ Failed: $file"
  fi
done
```

### Pipeline Integration

Use in data processing pipelines:

```bash
# Convert, transform, and upload
excel-to-json sales.xlsx \
  | jq '.data | map({id: .ProductID, name: .ProductName, revenue: (.Price * .Quantity)})' \
  | curl -X POST -H "Content-Type: application/json" -d @- \
    https://api.example.com/products/import

# Convert and import to PostgreSQL
excel-to-json customers.xlsx \
  | jq -r '.data[] | [.CustomerID, .Name, .Email, .Phone] | @csv' \
  | psql -d mydb -c "COPY customers FROM STDIN WITH CSV"
```

### Monitoring and Logging

Add logging to conversions:

```bash
#!/bin/bash
LOG_FILE="excel_conversions.log"

convert_with_logging() {
  local file="$1"
  local start_time=$(date +%s)
  
  result=$(excel-to-json "$file" 2>&1)
  exit_code=$?
  
  local end_time=$(date +%s)
  local duration=$((end_time - start_time))
  
  if [ $exit_code -eq 0 ]; then
    metadata=$(echo "$result" | jq -r '.metadata')
    echo "[$(date)] SUCCESS: $file (${duration}s) - $metadata" >> "$LOG_FILE"
    echo "$result"
  else
    echo "[$(date)] FAILURE: $file (${duration}s) - $result" >> "$LOG_FILE"
    return 1
  fi
}

# Usage
convert_with_logging "data.xlsx" > output.json
```

## Environment Variables

excel-to-json respects certain environment variables:

### `EXCEL_TO_JSON_DATE_FORMAT`

Default date format if not specified via command line:

```bash
export EXCEL_TO_JSON_DATE_FORMAT="%Y-%m-%d"
excel-to-json data.xlsx
```

### `EXCEL_TO_JSON_TIMEOUT`

Maximum time for conversion (in seconds):

```bash
export EXCEL_TO_JSON_TIMEOUT=60
excel-to-json large_file.xlsx
```

### `NO_COLOR`

Disable colored output:

```bash
export NO_COLOR=1
excel-to-json data.xlsx
```

## Exit Codes

excel-to-json uses standard exit codes:

- `0`: Success
- `1`: General error
- `2`: Invalid arguments
- `3`: File not found
- `4`: Parse error
- `5`: Timeout

Example usage in scripts:

```bash
excel-to-json data.xlsx > output.json
case $? in
  0) echo "Success" ;;
  3) echo "File not found" ;;
  4) echo "Failed to parse Excel file" ;;
  *) echo "Unknown error" ;;
esac
```

## Performance Tips

1. **Use `--quiet`** for faster processing when you don't need progress information
2. **Limit rows** with `--max-rows` when testing or sampling data
3. **Process sheets individually** for very large workbooks
4. **Use `--compact`** to reduce output size for large datasets
5. **Pipe directly** to compression for huge files: `excel-to-json large.xlsx | gzip > output.json.gz`

## Troubleshooting

### Common Issues

**"Command not found"**
- Ensure excel-to-json is in your PATH
- Try using the full path: `/usr/local/bin/excel-to-json`

**"Permission denied"**
- Check file permissions: `ls -la data.xlsx`
- Ensure you have read access to the Excel file

**"Invalid UTF-8"**
- File may contain special characters
- Try: `excel-to-json data.xlsx | iconv -f utf-8 -t utf-8 -c`

**Memory issues with large files**
- Use `--max-rows` to process in chunks
- Increase system limits: `ulimit -m unlimited`

## Next Steps

- Learn about [JavaScript Integration](/usage/javascript/) for Node.js applications
- Explore [TypeScript Integration](/usage/typescript/) for type-safe usage
- Check the [API Reference](/reference/cli-options/) for detailed option documentation
