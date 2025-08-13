---
title: Output Format
description: Detailed documentation of the JSON output structure
---

# Output Format Reference

excel-to-json produces structured JSON output with consistent formatting across all conversions. This reference describes the complete output structure.

## Basic Structure

### Single Sheet Output

```json
{
  "success": true,
  "data": [...],
  "metadata": {...},
  "error": null
}
```

### Multi-Sheet Output

When using `--all-sheets`:

```json
{
  "success": true,
  "sheets": {
    "Sheet1": {
      "data": [...],
      "metadata": {...}
    },
    "Sheet2": {
      "data": [...],
      "metadata": {...}
    }
  }
}
```

## Root Fields

### `success`

Indicates whether the conversion completed successfully.

- **Type**: Boolean
- **Values**: `true` | `false`
- **Always present**: Yes

### `data`

Array of converted rows (single sheet mode).

- **Type**: Array of Objects
- **Present when**: Single sheet conversion succeeds
- **Empty array**: When no data rows found

### `sheets`

Object containing data for multiple sheets.

- **Type**: Object
- **Present when**: Using `--all-sheets`
- **Keys**: Sheet names
- **Values**: Sheet data objects

### `error`

Error message if conversion failed.

- **Type**: String | null
- **Present when**: `success` is `false`
- **null when**: Conversion succeeds

### `metadata`

Processing information and statistics.

- **Type**: Object
- **Always present**: Yes (when successful)

## Data Array Structure

Each element in the `data` array represents one row from the Excel file:

```json
{
  "data": [
    {
      "Column1": "value1",
      "Column2": 123,
      "Column3": true,
      "Column4": null,
      "Column5": "2024-01-15T00:00:00Z"
    }
  ]
}
```

### Key Names

Keys are derived from:
1. **Header row** (default: first row)
2. **Column letters** when using `--no-header`
3. **Custom headers** when using `--custom-headers`

### Value Types

excel-to-json preserves data types:

| Excel Type | JSON Type | Example |
|------------|-----------|---------|
| Text | String | `"Hello World"` |
| Number | Number | `123.45` |
| Boolean | Boolean | `true` / `false` |
| Date/Time | String (ISO 8601) | `"2024-01-15T14:30:00Z"` |
| Empty Cell | null | `null` |
| Formula Result | Computed type | Based on result |

## Metadata Object

The `metadata` object contains processing statistics:

```json
{
  "metadata": {
    "total_rows_processed": 1000,
    "valid_records": 998,
    "invalid_records": 2,
    "processing_time_ms": 145,
    "sheet_name": "Sales",
    "file_name": "data.xlsx",
    "file_size_bytes": 524288,
    "warnings": ["Row 5: Missing required field 'ID'"],
    "skipped_rows": [5, 127],
    "column_count": 8,
    "date_processed": "2024-01-15T10:30:00Z"
  }
}
```

### Metadata Fields

#### `total_rows_processed`

Total number of rows examined (excluding header).

- **Type**: Integer
- **Minimum**: 0

#### `valid_records`

Number of successfully converted rows.

- **Type**: Integer
- **Minimum**: 0

#### `invalid_records`

Number of rows that failed conversion.

- **Type**: Integer
- **Minimum**: 0

#### `processing_time_ms`

Time taken to process the file in milliseconds.

- **Type**: Integer
- **Minimum**: 0

#### `sheet_name`

Name of the processed sheet.

- **Type**: String
- **Present when**: Single sheet mode

#### `file_name`

Name of the source Excel file.

- **Type**: String
- **Always present**: Yes

#### `file_size_bytes`

Size of the Excel file in bytes.

- **Type**: Integer
- **Optional**: May not be present for streamed input

#### `warnings`

Array of warning messages.

- **Type**: Array of Strings
- **Optional**: Only present if warnings exist
- **Empty array**: Omitted

#### `skipped_rows`

Row numbers that were skipped.

- **Type**: Array of Integers
- **Optional**: Only with `--skip-empty-rows`
- **1-indexed**: Row numbers start at 1

#### `column_count`

Number of columns processed.

- **Type**: Integer
- **Minimum**: 0

#### `date_processed`

Timestamp of when processing occurred.

- **Type**: String (ISO 8601)
- **Format**: `YYYY-MM-DDTHH:MM:SSZ`

## Type-Specific Formatting

### Strings

Text values are preserved as-is with proper escaping:

```json
{
  "name": "John \"Doc\" Smith",
  "description": "Line 1\nLine 2",
  "unicode": "Hello 世界 🌍"
}
```

### Numbers

Numbers maintain their precision:

```json
{
  "integer": 42,
  "decimal": 123.456789,
  "scientific": 1.23e-4,
  "negative": -99.99
}
```

### Dates and Times

Default format is ISO 8601:

```json
{
  "date": "2024-01-15T00:00:00Z",
  "datetime": "2024-01-15T14:30:45Z",
  "time": "14:30:45"
}
```

With custom formatting (`--date-format`):

```json
{
  "date": "2024-01-15",
  "datetime": "01/15/2024 14:30"
}
```

### Booleans

Excel boolean values are converted:

| Excel Value | JSON Value |
|-------------|------------|
| TRUE | `true` |
| FALSE | `false` |
| YES | `true` (configurable) |
| NO | `false` (configurable) |

### Null Values

Empty cells and errors become `null`:

```json
{
  "empty_cell": null,
  "error_cell": null,
  "blank_text": ""
}
```

## Error Output Format

When conversion fails (`success: false`):

```json
{
  "success": false,
  "error": "Failed to open file: data.xlsx",
  "error_code": "FILE_NOT_FOUND",
  "details": {
    "file_path": "data.xlsx",
    "attempted_at": "2024-01-15T10:30:00Z"
  }
}
```

### Error Fields

#### `error`

Human-readable error message.

- **Type**: String
- **Always present**: When `success` is `false`

#### `error_code`

Machine-readable error code.

- **Type**: String
- **Values**: See [Error Codes](#error-codes)

#### `details`

Additional error context.

- **Type**: Object
- **Optional**: May contain debugging information

## Special Cases

### Empty File

```json
{
  "success": true,
  "data": [],
  "metadata": {
    "total_rows_processed": 0,
    "valid_records": 0,
    "invalid_records": 0
  }
}
```

### Headers Only

```json
{
  "success": true,
  "data": [],
  "metadata": {
    "total_rows_processed": 0,
    "column_count": 5,
    "warnings": ["No data rows found"]
  }
}
```

### Partial Success

With `--ignore-errors`:

```json
{
  "success": true,
  "data": [...],
  "metadata": {
    "total_rows_processed": 100,
    "valid_records": 95,
    "invalid_records": 5,
    "warnings": [
      "Row 10: Type conversion error",
      "Row 25: Missing required fields"
    ]
  }
}
```

## Validation Output

With `--validate` flag:

```json
{
  "success": true,
  "validation": {
    "valid": false,
    "errors": [
      {
        "row": 5,
        "column": "Email",
        "value": "invalid-email",
        "error": "Invalid email format"
      }
    ],
    "summary": {
      "total_rows": 100,
      "valid_rows": 98,
      "invalid_rows": 2
    }
  }
}
```

## Large File Handling

For very large files, consider streaming approaches:

### NDJSON Format

With `--ndjson` flag (if available):

```json
{"row":1,"data":{"id":1,"name":"Item 1"}}
{"row":2,"data":{"id":2,"name":"Item 2"}}
{"row":3,"data":{"id":3,"name":"Item 3"}}
```

## Examples

### Successful Conversion

```json
{
  "success": true,
  "data": [
    {
      "ProductID": "P001",
      "ProductName": "Widget A",
      "Price": 19.99,
      "Quantity": 100,
      "InStock": true,
      "LastUpdated": "2024-01-15T00:00:00Z"
    },
    {
      "ProductID": "P002",
      "ProductName": "Widget B",
      "Price": 29.99,
      "Quantity": 0,
      "InStock": false,
      "LastUpdated": "2024-01-14T00:00:00Z"
    }
  ],
  "metadata": {
    "total_rows_processed": 2,
    "valid_records": 2,
    "invalid_records": 0,
    "processing_time_ms": 15,
    "sheet_name": "Products",
    "file_name": "inventory.xlsx",
    "column_count": 6,
    "date_processed": "2024-01-15T10:30:00Z"
  }
}
```

### Multi-Sheet Conversion

```json
{
  "success": true,
  "sheets": {
    "Products": {
      "data": [...],
      "metadata": {
        "total_rows_processed": 100,
        "valid_records": 100
      }
    },
    "Categories": {
      "data": [...],
      "metadata": {
        "total_rows_processed": 10,
        "valid_records": 10
      }
    }
  }
}
```

## Next Steps

- Learn about [Error Handling](/reference/error-handling/) for error management
- Check [CLI Options](/reference/cli-options/) for output customization
- See [TypeScript Integration](/usage/typescript/) for type-safe parsing
