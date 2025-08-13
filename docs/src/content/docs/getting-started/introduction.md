---
title: Introduction
description: Learn about excel-to-json and how it can help you extract data from Excel files
---

# Introduction to excel-to-json

**excel-to-json** is a high-performance command-line tool written in Rust that converts Excel files (.xlsx, .xls) into JSON format. It's designed for developers who need a reliable, fast, and easy way to extract data from Excel spreadsheets and integrate it into their applications.

## What is excel-to-json?

excel-to-json is a standalone CLI application that:
- Reads Excel files in various formats (.xlsx, .xls, .xlsm)
- Converts spreadsheet data into clean, structured JSON
- Provides detailed metadata about the conversion process
- Handles errors gracefully with informative messages

## Why Use excel-to-json?

### 🚀 Performance
Built with Rust, excel-to-json offers exceptional performance, processing large Excel files in milliseconds rather than seconds. This makes it ideal for:
- Batch processing operations
- Real-time data pipelines
- CI/CD integrations

### 🎯 Simplicity
No complex configuration files or setup required. Just install and run:
```bash
excel-to-json data.xlsx > output.json
```

### 🔧 Developer-Friendly
- **Clean JSON output**: Well-structured data that's easy to parse
- **Comprehensive metadata**: Track processing statistics and potential issues
- **Cross-language compatibility**: Use from any programming language via shell execution
- **Predictable behavior**: Consistent output format across all conversions

### 📊 Real-World Use Cases

1. **Data Migration**: Convert legacy Excel reports to JSON for modern database systems
2. **API Integration**: Transform Excel uploads into JSON for REST API consumption
3. **Configuration Management**: Use Excel as a user-friendly configuration format
4. **Report Processing**: Extract data from Excel reports for analysis tools
5. **Build Automation**: Process Excel files as part of your build pipeline

## Key Features

- **Multi-sheet support**: Process specific sheets or all sheets at once
- **Type preservation**: Maintains data types (numbers, dates, booleans, text)
- **Error resilience**: Continues processing even with malformed data
- **Unicode support**: Handles international characters correctly
- **Streaming architecture**: Memory-efficient processing of large files
- **Detailed diagnostics**: Provides warnings and processing statistics

## How It Works

1. **Parse**: excel-to-json reads your Excel file using efficient Rust libraries
2. **Transform**: Data is converted to JSON while preserving types and structure
3. **Validate**: Each row is validated and issues are reported in metadata
4. **Output**: Clean JSON is output with processing statistics

## Example Workflow

Here's a typical workflow using excel-to-json:

```bash
# Check what sheets are available
excel-to-json inventory.xlsx --list-sheets

# Convert a specific sheet
excel-to-json inventory.xlsx -s "Products" > products.json

# Process all sheets
excel-to-json inventory.xlsx --all-sheets > all_data.json

# Use in a pipeline
excel-to-json sales.xlsx | jq '.data[] | select(.amount > 1000)'
```

## Next Steps

Ready to get started? Check out our [Installation Guide](/getting-started/installation/) to install excel-to-json on your system.

For practical examples, visit our [Quick Start Guide](/getting-started/quick-start/) to see excel-to-json in action.
