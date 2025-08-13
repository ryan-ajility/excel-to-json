---
title: JavaScript/Node.js
description: Using excel-to-json with JavaScript and Node.js applications
---

# JavaScript/Node.js Integration

This guide shows how to integrate excel-to-json into your JavaScript and Node.js applications.

## Basic Usage with Node.js

### Using child_process.exec

The simplest way to use excel-to-json from Node.js:

```javascript
const { exec } = require('child_process');

exec('excel-to-json data.xlsx', (error, stdout, stderr) => {
  if (error) {
    console.error(`Error: ${error.message}`);
    return;
  }
  
  if (stderr) {
    console.error(`stderr: ${stderr}`);
    return;
  }
  
  const result = JSON.parse(stdout);
  console.log('Conversion result:', result);
});
```

### Using Promises with util.promisify

For better async/await support:

```javascript
const { exec } = require('child_process');
const { promisify } = require('util');
const execAsync = promisify(exec);

async function convertExcelToJson(filePath) {
  try {
    const { stdout } = await execAsync(`excel-to-json "${filePath}"`);
    return JSON.parse(stdout);
  } catch (error) {
    throw new Error(`Excel conversion failed: ${error.message}`);
  }
}

// Usage
(async () => {
  try {
    const data = await convertExcelToJson('inventory.xlsx');
    console.log(`Processed ${data.metadata.total_rows_processed} rows`);
    console.log('Data:', data.data);
  } catch (error) {
    console.error(error);
  }
})();
```

## Advanced Features

### Processing Specific Sheets

```javascript
async function convertSheet(filePath, sheetName) {
  const command = `excel-to-json "${filePath}" -s "${sheetName}"`;
  const { stdout } = await execAsync(command);
  return JSON.parse(stdout);
}

// Convert specific sheet
const salesData = await convertSheet('workbook.xlsx', 'Sales');
```

### Processing All Sheets

```javascript
async function convertAllSheets(filePath) {
  const command = `excel-to-json "${filePath}" --all-sheets`;
  const { stdout } = await execAsync(command);
  return JSON.parse(stdout);
}

// Convert all sheets
const allData = await convertAllSheets('workbook.xlsx');
Object.keys(allData.sheets).forEach(sheetName => {
  console.log(`Sheet: ${sheetName}`);
  console.log(`Rows: ${allData.sheets[sheetName].data.length}`);
});
```

## Creating a Reusable Module

Create a reusable wrapper for excel-to-json:

```javascript
// excel-converter.js
const { exec } = require('child_process');
const { promisify } = require('util');
const path = require('path');
const execAsync = promisify(exec);

class ExcelConverter {
  constructor(options = {}) {
    this.command = options.command || 'excel-to-json';
  }

  async convert(filePath, options = {}) {
    const args = this._buildArgs(options);
    const command = `${this.command} "${filePath}" ${args}`;
    
    try {
      const { stdout } = await execAsync(command);
      const result = JSON.parse(stdout);
      
      if (!result.success) {
        throw new Error(result.error || 'Conversion failed');
      }
      
      return result;
    } catch (error) {
      throw new Error(`Excel conversion failed: ${error.message}`);
    }
  }

  async listSheets(filePath) {
    const command = `${this.command} "${filePath}" --list-sheets`;
    const { stdout } = await execAsync(command);
    
    // Parse sheet names from output
    const sheets = stdout
      .split('\n')
      .filter(line => line.trim().match(/^\d+\./))
      .map(line => line.replace(/^\d+\.\s*/, '').trim());
    
    return sheets;
  }

  async convertSheet(filePath, sheetName) {
    return this.convert(filePath, { sheet: sheetName });
  }

  async convertAllSheets(filePath) {
    return this.convert(filePath, { allSheets: true });
  }

  _buildArgs(options) {
    const args = [];
    
    if (options.sheet) {
      args.push(`-s "${options.sheet}"`);
    }
    
    if (options.allSheets) {
      args.push('--all-sheets');
    }
    
    if (options.quiet) {
      args.push('--quiet');
    }
    
    return args.join(' ');
  }
}

module.exports = ExcelConverter;
```

Usage:

```javascript
const ExcelConverter = require('./excel-converter');

const converter = new ExcelConverter();

// Basic conversion
const result = await converter.convert('data.xlsx');

// List sheets
const sheets = await converter.listSheets('workbook.xlsx');
console.log('Available sheets:', sheets);

// Convert specific sheet
const salesData = await converter.convertSheet('workbook.xlsx', 'Sales');

// Convert all sheets
const allData = await converter.convertAllSheets('workbook.xlsx');
```

## Express.js Integration

Handle Excel file uploads in an Express application:

```javascript
const express = require('express');
const multer = require('multer');
const fs = require('fs').promises;
const ExcelConverter = require('./excel-converter');

const app = express();
const upload = multer({ dest: 'uploads/' });
const converter = new ExcelConverter();

app.post('/upload/excel', upload.single('file'), async (req, res) => {
  try {
    if (!req.file) {
      return res.status(400).json({ error: 'No file uploaded' });
    }

    // Convert Excel to JSON
    const result = await converter.convert(req.file.path);
    
    // Clean up uploaded file
    await fs.unlink(req.file.path);
    
    // Return JSON data
    res.json({
      filename: req.file.originalname,
      rows: result.metadata.total_rows_processed,
      data: result.data
    });
  } catch (error) {
    console.error('Conversion error:', error);
    res.status(500).json({ error: 'Failed to convert Excel file' });
  }
});

app.listen(3000, () => {
  console.log('Server running on port 3000');
});
```

## Stream Processing

For large files, process data in streams:

```javascript
const { spawn } = require('child_process');
const { Transform } = require('stream');

function streamExcelToJson(filePath) {
  return new Promise((resolve, reject) => {
    const child = spawn('excel-to-json', [filePath]);
    let output = '';
    
    child.stdout.on('data', (chunk) => {
      output += chunk.toString();
    });
    
    child.stderr.on('data', (data) => {
      console.error(`stderr: ${data}`);
    });
    
    child.on('close', (code) => {
      if (code !== 0) {
        reject(new Error(`Process exited with code ${code}`));
      } else {
        resolve(JSON.parse(output));
      }
    });
  });
}

// Stream processing with transformation
class ExcelProcessor extends Transform {
  constructor(options) {
    super(options);
    this.buffer = '';
  }
  
  _transform(chunk, encoding, callback) {
    this.buffer += chunk.toString();
    callback();
  }
  
  _flush(callback) {
    try {
      const data = JSON.parse(this.buffer);
      if (data.success) {
        data.data.forEach(row => {
          this.push(JSON.stringify(row) + '\n');
        });
      }
      callback();
    } catch (error) {
      callback(error);
    }
  }
}
```

## Error Handling

Comprehensive error handling example:

```javascript
class ExcelConversionError extends Error {
  constructor(message, details = {}) {
    super(message);
    this.name = 'ExcelConversionError';
    this.details = details;
  }
}

async function safeConvert(filePath, options = {}) {
  try {
    // Check if file exists
    await fs.access(filePath);
    
    // Check file extension
    if (!filePath.match(/\.(xlsx?|xlsm)$/i)) {
      throw new ExcelConversionError('Invalid file type', {
        filePath,
        expectedTypes: ['.xlsx', '.xls', '.xlsm']
      });
    }
    
    // Perform conversion
    const result = await converter.convert(filePath, options);
    
    // Check for warnings
    if (result.metadata.warnings && result.metadata.warnings.length > 0) {
      console.warn('Conversion warnings:', result.metadata.warnings);
    }
    
    // Validate data
    if (result.metadata.invalid_records > 0) {
      console.warn(`Found ${result.metadata.invalid_records} invalid records`);
    }
    
    return result;
  } catch (error) {
    if (error.code === 'ENOENT') {
      throw new ExcelConversionError('File not found', { filePath });
    }
    throw error;
  }
}

// Usage with error handling
try {
  const result = await safeConvert('data.xlsx');
  console.log('Success:', result.data);
} catch (error) {
  if (error instanceof ExcelConversionError) {
    console.error('Conversion error:', error.message);
    console.error('Details:', error.details);
  } else {
    console.error('Unexpected error:', error);
  }
}
```

## Testing

Example test suite using Jest:

```javascript
// excel-converter.test.js
const ExcelConverter = require('./excel-converter');
const fs = require('fs').promises;
const path = require('path');

describe('ExcelConverter', () => {
  const converter = new ExcelConverter();
  const testFile = path.join(__dirname, 'test-data.xlsx');
  
  beforeAll(async () => {
    // Ensure test file exists
    await fs.access(testFile);
  });
  
  test('should convert Excel to JSON', async () => {
    const result = await converter.convert(testFile);
    
    expect(result).toHaveProperty('success', true);
    expect(result).toHaveProperty('data');
    expect(Array.isArray(result.data)).toBe(true);
    expect(result).toHaveProperty('metadata');
  });
  
  test('should list sheets', async () => {
    const sheets = await converter.listSheets(testFile);
    
    expect(Array.isArray(sheets)).toBe(true);
    expect(sheets.length).toBeGreaterThan(0);
  });
  
  test('should handle missing files', async () => {
    await expect(converter.convert('nonexistent.xlsx'))
      .rejects
      .toThrow('Excel conversion failed');
  });
  
  test('should convert specific sheet', async () => {
    const sheets = await converter.listSheets(testFile);
    if (sheets.length > 0) {
      const result = await converter.convertSheet(testFile, sheets[0]);
      expect(result.success).toBe(true);
    }
  });
});
```

## Performance Optimization

Tips for optimizing performance:

```javascript
// Batch processing with concurrency control
const pLimit = require('p-limit');

async function batchConvert(files, concurrency = 4) {
  const limit = pLimit(concurrency);
  const converter = new ExcelConverter();
  
  const promises = files.map(file => 
    limit(() => converter.convert(file))
  );
  
  return Promise.all(promises);
}

// Cache results
const cache = new Map();

async function cachedConvert(filePath) {
  const stat = await fs.stat(filePath);
  const cacheKey = `${filePath}:${stat.mtime.toISOString()}`;
  
  if (cache.has(cacheKey)) {
    return cache.get(cacheKey);
  }
  
  const result = await converter.convert(filePath);
  cache.set(cacheKey, result);
  
  return result;
}
```

## Next Steps

- Explore [TypeScript Integration](/usage/typescript/) for type-safe development
- Learn about [Other Language Integrations](/usage/other-languages/)
- Check the [CLI Options Reference](/reference/cli-options/) for all available options
