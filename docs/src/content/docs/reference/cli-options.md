---
title: CLI Options
description: Complete reference for all excel-to-json command-line options
---

# CLI Options Reference

Complete reference for all command-line options available in excel-to-json.

## Synopsis

```bash
excel-to-json [OPTIONS] <FILE>
```

## Required Arguments

### `<FILE>`

Path to the Excel file to convert.

- **Type**: String (file path)
- **Required**: Yes
- **Accepts**: `.xlsx`, `.xls`, `.xlsm` files

Examples:
```bash
excel-to-json data.xlsx
excel-to-json /path/to/file.xlsx
excel-to-json "../reports/sales.xlsx"
```

## Options

### Sheet Selection

#### `-s, --sheet <SHEET_NAME>`

Select a specific sheet to convert by name.

- **Type**: String
- **Default**: First sheet in workbook
- **Conflicts with**: `--all-sheets`

```bash
excel-to-json data.xlsx -s "Sales"
excel-to-json data.xlsx --sheet "January 2024"
```

#### `--all-sheets`

Convert all sheets in the workbook.

- **Type**: Flag
- **Default**: false
- **Conflicts with**: `-s, --sheet`

```bash
excel-to-json workbook.xlsx --all-sheets
```

#### `--list-sheets`

List available sheets without converting.

- **Type**: Flag
- **Default**: false
- **Output**: List of sheet names to stdout

```bash
excel-to-json workbook.xlsx --list-sheets
```

### Output Formatting

#### `--pretty`

Format JSON output with indentation.

- **Type**: Flag
- **Default**: false
- **Indent**: 2 spaces

```bash
excel-to-json data.xlsx --pretty
```

#### `--compact`

Minimize JSON output size (no unnecessary whitespace).

- **Type**: Flag
- **Default**: false
- **Conflicts with**: `--pretty`

```bash
excel-to-json data.xlsx --compact
```

#### `--quiet`

Suppress all non-JSON output.

- **Type**: Flag
- **Default**: false

```bash
excel-to-json data.xlsx --quiet
```

### Data Processing

#### `--skip-empty-rows`

Skip rows where all cells are empty.

- **Type**: Flag
- **Default**: false

```bash
excel-to-json data.xlsx --skip-empty-rows
```

#### `--skip-empty-columns`

Skip columns that have no header.

- **Type**: Flag
- **Default**: false

```bash
excel-to-json data.xlsx --skip-empty-columns
```

#### `--max-rows <NUMBER>`

Limit the number of data rows to process.

- **Type**: Integer
- **Default**: No limit
- **Minimum**: 1

```bash
excel-to-json large_file.xlsx --max-rows 100
```

#### `--start-row <NUMBER>`

Start processing from a specific row.

- **Type**: Integer
- **Default**: 2 (after header row)
- **Minimum**: 1

```bash
excel-to-json data.xlsx --start-row 5
```

#### `--end-row <NUMBER>`

Stop processing at a specific row.

- **Type**: Integer
- **Default**: Last row with data
- **Minimum**: 1

```bash
excel-to-json data.xlsx --end-row 1000
```

### Header Configuration

#### `--no-header`

Treat first row as data, not headers.

- **Type**: Flag
- **Default**: false
- **Result**: Uses column letters as keys (A, B, C...)

```bash
excel-to-json data.xlsx --no-header
```

#### `--header-row <NUMBER>`

Specify which row contains headers.

- **Type**: Integer
- **Default**: 1
- **Minimum**: 1

```bash
excel-to-json data.xlsx --header-row 3
```

#### `--custom-headers <HEADERS>`

Provide custom headers (comma-separated).

- **Type**: String
- **Format**: "Header1,Header2,Header3"

```bash
excel-to-json data.xlsx --custom-headers "ID,Name,Email,Phone"
```

### Type Handling

#### `--force-text`

Treat all cell values as text.

- **Type**: Flag
- **Default**: false
- **Disables**: Automatic type detection

```bash
excel-to-json data.xlsx --force-text
```

#### `--date-format <FORMAT>`

Specify date output format.

- **Type**: String
- **Default**: ISO 8601
- **Options**: 
  - `iso8601` (default)
  - `%Y-%m-%d`
  - `%m/%d/%Y`
  - Custom strftime format

```bash
excel-to-json data.xlsx --date-format "%Y-%m-%d"
excel-to-json data.xlsx --date-format "iso8601"
```

#### `--number-format <FORMAT>`

Control number formatting.

- **Type**: String
- **Default**: Original precision
- **Format**: Printf-style format string

```bash
excel-to-json data.xlsx --number-format "%.2f"
excel-to-json data.xlsx --number-format "%d"
```

#### `--boolean-values <TRUE,FALSE>`

Custom boolean representations.

- **Type**: String
- **Format**: "true_value,false_value"
- **Default**: "true,false"

```bash
excel-to-json data.xlsx --boolean-values "Yes,No"
excel-to-json data.xlsx --boolean-values "1,0"
```

### Error Handling

#### `--strict`

Stop processing on first error.

- **Type**: Flag
- **Default**: false

```bash
excel-to-json data.xlsx --strict
```

#### `--ignore-errors`

Continue processing, exclude invalid rows.

- **Type**: Flag
- **Default**: false

```bash
excel-to-json data.xlsx --ignore-errors
```

#### `--max-errors <NUMBER>`

Maximum number of errors before stopping.

- **Type**: Integer
- **Default**: No limit

```bash
excel-to-json data.xlsx --max-errors 10
```

### Performance

#### `--buffer-size <SIZE>`

Set read buffer size in KB.

- **Type**: Integer
- **Default**: 8192 (8MB)
- **Unit**: Kilobytes

```bash
excel-to-json large_file.xlsx --buffer-size 16384
```

#### `--parallel`

Enable parallel processing for multiple sheets.

- **Type**: Flag
- **Default**: false
- **Requires**: `--all-sheets`

```bash
excel-to-json workbook.xlsx --all-sheets --parallel
```

### Validation

#### `--validate`

Validate data without outputting JSON.

- **Type**: Flag
- **Default**: false
- **Output**: Validation report only

```bash
excel-to-json data.xlsx --validate
```

#### `--schema <FILE>`

Validate against JSON schema file.

- **Type**: String (file path)
- **Format**: JSON Schema

```bash
excel-to-json data.xlsx --schema schema.json
```

### Information

#### `-h, --help`

Display help information.

- **Type**: Flag

```bash
excel-to-json --help
excel-to-json -h
```

#### `-V, --version`

Display version information.

- **Type**: Flag

```bash
excel-to-json --version
excel-to-json -V
```

#### `--debug`

Enable debug output.

- **Type**: Flag
- **Default**: false
- **Output**: Verbose logging to stderr

```bash
excel-to-json data.xlsx --debug
```

## Option Combinations

### Common Combinations

#### Production Use
```bash
excel-to-json data.xlsx \
  --quiet \
  --compact \
  --skip-empty-rows \
  --ignore-errors
```

#### Development/Testing
```bash
excel-to-json data.xlsx \
  --pretty \
  --max-rows 100 \
  --debug
```

#### Data Validation
```bash
excel-to-json data.xlsx \
  --validate \
  --strict \
  --schema schema.json
```

#### Batch Processing
```bash
excel-to-json workbook.xlsx \
  --all-sheets \
  --parallel \
  --quiet \
  --compact
```

## Environment Variable Overrides

Some options can be set via environment variables:

| Option | Environment Variable | Priority |
|--------|---------------------|----------|
| `--date-format` | `EXCEL_TO_JSON_DATE_FORMAT` | CLI > ENV |
| `--number-format` | `EXCEL_TO_JSON_NUMBER_FORMAT` | CLI > ENV |
| `--buffer-size` | `EXCEL_TO_JSON_BUFFER_SIZE` | CLI > ENV |
| `--quiet` | `EXCEL_TO_JSON_QUIET` | CLI > ENV |
| `--debug` | `EXCEL_TO_JSON_DEBUG` | CLI > ENV |

Example:
```bash
export EXCEL_TO_JSON_DATE_FORMAT="%Y-%m-%d"
export EXCEL_TO_JSON_QUIET=true
excel-to-json data.xlsx  # Uses environment settings
```

## Configuration File

Options can also be specified in a configuration file:

```bash
excel-to-json data.xlsx --config config.json
```

Example `config.json`:
```json
{
  "date_format": "%Y-%m-%d",
  "skip_empty_rows": true,
  "max_rows": 1000,
  "quiet": true
}
```

## Next Steps

- See [Output Format](/reference/output-format/) for JSON structure details
- Learn about [Error Handling](/reference/error-handling/) for error management
- Check [Command Line Usage](/usage/command-line/) for practical examples
