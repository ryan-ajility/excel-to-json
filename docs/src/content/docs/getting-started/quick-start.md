---
title: Quick Start
description: Get up and running with excel-to-json in minutes
---

# Quick Start Guide

This guide will help you get started with excel-to-json quickly. We'll cover basic usage and common scenarios.

## Basic Usage

The simplest way to use excel-to-json is to provide an Excel file as input:

```bash
excel-to-json data.xlsx
```

This will output JSON to stdout. To save to a file:

```bash
excel-to-json data.xlsx > output.json
```

## Example Excel File

Let's say you have an Excel file `inventory.xlsx` with the following data:

| Product ID | Product Name | Price | Quantity | In Stock |
|------------|-------------|-------|----------|----------|
| P001 | Widget A | 19.99 | 100 | TRUE |
| P002 | Widget B | 29.99 | 50 | TRUE |
| P003 | Gadget X | 49.99 | 0 | FALSE |

## Converting to JSON

Run the conversion:

```bash
excel-to-json inventory.xlsx
```

Output:
```json
{
  "success": true,
  "data": [
    {
      "Product ID": "P001",
      "Product Name": "Widget A",
      "Price": 19.99,
      "Quantity": 100,
      "In Stock": true
    },
    {
      "Product ID": "P002",
      "Product Name": "Widget B",
      "Price": 29.99,
      "Quantity": 50,
      "In Stock": true
    },
    {
      "Product ID": "P003",
      "Product Name": "Gadget X",
      "Price": 49.99,
      "Quantity": 0,
      "In Stock": false
    }
  ],
  "metadata": {
    "total_rows_processed": 3,
    "valid_records": 3,
    "invalid_records": 0,
    "processing_time_ms": 12
  }
}
```

## Working with Multiple Sheets

### List Available Sheets

First, see what sheets are in your Excel file:

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

### Convert a Specific Sheet

To convert only the "Inventory" sheet:

```bash
excel-to-json workbook.xlsx -s "Inventory"
```

### Convert All Sheets

To convert all sheets at once:

```bash
excel-to-json workbook.xlsx --all-sheets
```

This produces output with each sheet as a key:

```json
{
  "success": true,
  "sheets": {
    "Sales": {
      "data": [...],
      "metadata": {...}
    },
    "Inventory": {
      "data": [...],
      "metadata": {...}
    },
    "Customers": {
      "data": [...],
      "metadata": {...}
    }
  }
}
```

## Common Use Cases

### 1. Processing in a Script

**Bash/Shell:**
```bash
#!/bin/bash
# Convert Excel to JSON and process with jq
excel-to-json sales.xlsx | jq '.data[] | select(.amount > 1000)'
```

**Python:**
```python
import subprocess
import json

# Run excel-to-json
result = subprocess.run(
    ['excel-to-json', 'data.xlsx'],
    capture_output=True,
    text=True
)

# Parse JSON output
data = json.loads(result.stdout)
if data['success']:
    for row in data['data']:
        print(f"Processing: {row}")
```

**Node.js:**
```javascript
const { exec } = require('child_process');

exec('excel-to-json data.xlsx', (error, stdout, stderr) => {
  if (error) {
    console.error(`Error: ${error}`);
    return;
  }
  
  const result = JSON.parse(stdout);
  if (result.success) {
    console.log(`Processed ${result.metadata.total_rows_processed} rows`);
    result.data.forEach(row => {
      // Process each row
      console.log(row);
    });
  }
});
```

### 2. Data Pipeline Integration

Use excel-to-json in a data pipeline:

```bash
# Extract, transform, and load
excel-to-json raw_data.xlsx | \
  jq '.data | map(select(.status == "active"))' | \
  curl -X POST -H "Content-Type: application/json" -d @- \
  http://api.example.com/import
```

### 3. Batch Processing

Process multiple Excel files:

```bash
#!/bin/bash
# Convert all Excel files in a directory
for file in *.xlsx; do
  echo "Processing $file..."
  excel-to-json "$file" > "${file%.xlsx}.json"
done
```

### 4. Validation and Error Handling

Check for conversion errors:

```bash
# Save output and check for success
output=$(excel-to-json data.xlsx)
success=$(echo "$output" | jq -r '.success')

if [ "$success" = "true" ]; then
  echo "Conversion successful"
  echo "$output" | jq '.metadata'
else
  echo "Conversion failed"
  echo "$output" | jq -r '.error'
fi
```

## Handling Different Data Types

excel-to-json automatically detects and preserves data types:

### Numbers
Excel: `123.45`  
JSON: `123.45`

### Text
Excel: `"Hello World"`  
JSON: `"Hello World"`

### Booleans
Excel: `TRUE` / `FALSE`  
JSON: `true` / `false`

### Dates
Excel: `2024-01-15`  
JSON: `"2024-01-15T00:00:00"`

### Empty Cells
Excel: (empty)  
JSON: `null`

## Tips and Best Practices

### 1. Clean Headers
Ensure your Excel file has clean, consistent headers in the first row:
- ✅ Good: `Product_ID`, `ProductName`, `product-code`
- ❌ Avoid: Spaces at start/end, special characters, duplicate names

### 2. Handle Large Files
For large Excel files, pipe output directly to avoid memory issues:
```bash
excel-to-json large_file.xlsx | gzip > output.json.gz
```

### 3. Validate Output
Always check the metadata for warnings:
```bash
excel-to-json data.xlsx | jq '.metadata.warnings'
```

### 4. Use with Version Control
JSON output is text-based and works well with Git:
```bash
excel-to-json config.xlsx > config.json
git add config.json
git commit -m "Update configuration"
```

## Troubleshooting Quick Issues

### Empty Output
If you get empty data arrays:
- Check if the Excel file has headers in the first row
- Verify the sheet name is correct (case-sensitive)
- Ensure the file isn't corrupted

### Type Mismatches
If numbers appear as strings:
- Check Excel cell formatting
- Ensure cells don't contain leading/trailing spaces

### Performance Issues
For slow conversions:
- Consider processing sheets individually
- Check file size and complexity
- Use `--quiet` flag to reduce output verbosity

## Next Steps

Now that you're familiar with basic usage:

- Learn about [Command Line Options](/usage/command-line/) for advanced features
- Explore [Integration Examples](/usage/javascript/) for your programming language
- Check the [API Reference](/reference/cli-options/) for detailed documentation

Happy converting! 🚀
