---
title: TypeScript
description: Type-safe integration of excel-to-json with TypeScript applications
---

# TypeScript Integration

This guide demonstrates how to use excel-to-json with TypeScript for type-safe Excel to JSON conversions.

## Type Definitions

First, define the types for excel-to-json output:

```typescript
import { exec } from 'child_process';
import { promisify } from 'util';

const execAsync = promisify(exec);

// Define the structure of an Excel row
interface ExcelRow {
  [key: string]: string | number | boolean | null;
}

// Define the complete result structure
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

// Multi-sheet result structure
interface MultiSheetResult {
  success: boolean;
  sheets: {
    [sheetName: string]: {
      data: ExcelRow[];
      metadata: ExcelResult['metadata'];
    };
  };
}
```

## Basic Usage

Create a type-safe wrapper function:

```typescript
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

## Creating a Typed Excel Converter Class

Build a comprehensive TypeScript class for excel-to-json:

```typescript
import { exec, ExecException } from 'child_process';
import { promisify } from 'util';
import * as fs from 'fs/promises';
import * as path from 'path';

const execAsync = promisify(exec);

export interface ConverterOptions {
  command?: string;
  timeout?: number;
}

export interface ConvertOptions {
  sheet?: string;
  allSheets?: boolean;
  quiet?: boolean;
}

export class ExcelConverter {
  private readonly command: string;
  private readonly timeout: number;

  constructor(options: ConverterOptions = {}) {
    this.command = options.command || 'excel-to-json';
    this.timeout = options.timeout || 30000; // 30 seconds default
  }

  async convert(filePath: string, options: ConvertOptions = {}): Promise<ExcelResult> {
    await this.validateFile(filePath);
    
    const args = this.buildArgs(options);
    const command = `${this.command} "${filePath}" ${args}`;
    
    try {
      const { stdout } = await execAsync(command, { timeout: this.timeout });
      const result: ExcelResult = JSON.parse(stdout);
      
      if (!result.success) {
        throw new Error(result.error || 'Conversion failed');
      }
      
      return result;
    } catch (error) {
      if (this.isExecError(error) && error.code === 'ETIMEDOUT') {
        throw new Error(`Conversion timed out after ${this.timeout}ms`);
      }
      throw new Error(`Excel conversion failed: ${error}`);
    }
  }

  async convertSheet(filePath: string, sheetName: string): Promise<ExcelResult> {
    return this.convert(filePath, { sheet: sheetName });
  }

  async convertAllSheets(filePath: string): Promise<MultiSheetResult> {
    const command = `${this.command} "${filePath}" --all-sheets`;
    
    try {
      const { stdout } = await execAsync(command, { timeout: this.timeout });
      return JSON.parse(stdout);
    } catch (error) {
      throw new Error(`Failed to convert all sheets: ${error}`);
    }
  }

  async listSheets(filePath: string): Promise<string[]> {
    const command = `${this.command} "${filePath}" --list-sheets`;
    
    try {
      const { stdout } = await execAsync(command);
      return this.parseSheetList(stdout);
    } catch (error) {
      throw new Error(`Failed to list sheets: ${error}`);
    }
  }

  private async validateFile(filePath: string): Promise<void> {
    try {
      await fs.access(filePath);
    } catch {
      throw new Error(`File not found: ${filePath}`);
    }

    const ext = path.extname(filePath).toLowerCase();
    if (!['.xlsx', '.xls', '.xlsm'].includes(ext)) {
      throw new Error(`Invalid file type: ${ext}. Expected .xlsx, .xls, or .xlsm`);
    }
  }

  private buildArgs(options: ConvertOptions): string {
    const args: string[] = [];
    
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

  private parseSheetList(output: string): string[] {
    return output
      .split('\n')
      .filter(line => line.trim().match(/^\d+\./))
      .map(line => line.replace(/^\d+\.\s*/, '').trim());
  }

  private isExecError(error: unknown): error is ExecException {
    return error instanceof Error && 'code' in error;
  }
}
```

## Working with Strongly Typed Data

Define specific interfaces for your Excel data:

```typescript
// Define your data structure
interface Product {
  ProductID: string;
  ProductName: string;
  Price: number;
  Quantity: number;
  InStock: boolean;
}

// Type guard to validate data
function isProduct(row: ExcelRow): row is Product {
  return (
    typeof row.ProductID === 'string' &&
    typeof row.ProductName === 'string' &&
    typeof row.Price === 'number' &&
    typeof row.Quantity === 'number' &&
    typeof row.InStock === 'boolean'
  );
}

// Typed conversion function
async function getProducts(filePath: string): Promise<Product[]> {
  const converter = new ExcelConverter();
  const result = await converter.convert(filePath);
  
  const products: Product[] = [];
  const errors: string[] = [];
  
  result.data.forEach((row, index) => {
    if (isProduct(row)) {
      products.push(row);
    } else {
      errors.push(`Invalid product data at row ${index + 2}`);
    }
  });
  
  if (errors.length > 0) {
    console.warn('Data validation errors:', errors);
  }
  
  return products;
}

// Usage
async function processProducts() {
  const products = await getProducts('products.xlsx');
  
  // Type-safe operations
  const totalValue = products.reduce((sum, p) => sum + (p.Price * p.Quantity), 0);
  const inStockProducts = products.filter(p => p.InStock);
  
  console.log(`Total inventory value: $${totalValue}`);
  console.log(`Products in stock: ${inStockProducts.length}`);
}
```

## Generic Type-Safe Converter

Create a generic converter for any data type:

```typescript
export class TypedExcelConverter<T extends ExcelRow> {
  private converter: ExcelConverter;
  
  constructor(
    private validator: (row: ExcelRow) => row is T,
    options?: ConverterOptions
  ) {
    this.converter = new ExcelConverter(options);
  }
  
  async convert(filePath: string, options?: ConvertOptions): Promise<{
    data: T[];
    invalid: Array<{ row: number; data: ExcelRow }>;
    metadata: ExcelResult['metadata'];
  }> {
    const result = await this.converter.convert(filePath, options);
    
    const data: T[] = [];
    const invalid: Array<{ row: number; data: ExcelRow }> = [];
    
    result.data.forEach((row, index) => {
      if (this.validator(row)) {
        data.push(row);
      } else {
        invalid.push({ row: index + 2, data: row });
      }
    });
    
    return {
      data,
      invalid,
      metadata: result.metadata
    };
  }
}

// Usage with a specific type
const productConverter = new TypedExcelConverter<Product>(isProduct);

async function importProducts() {
  const { data, invalid, metadata } = await productConverter.convert('products.xlsx');
  
  console.log(`Successfully imported ${data.length} products`);
  
  if (invalid.length > 0) {
    console.warn(`Failed to import ${invalid.length} rows:`);
    invalid.forEach(({ row, data }) => {
      console.warn(`  Row ${row}:`, data);
    });
  }
}
```

## Express.js with TypeScript

Type-safe Express endpoint for Excel uploads:

```typescript
import express, { Request, Response } from 'express';
import multer from 'multer';
import { ExcelConverter } from './excel-converter';
import * as fs from 'fs/promises';

const app = express();
const upload = multer({ dest: 'uploads/' });
const converter = new ExcelConverter();

interface UploadResponse {
  success: boolean;
  filename?: string;
  rows?: number;
  data?: ExcelRow[];
  error?: string;
}

app.post('/upload/excel', 
  upload.single('file'),
  async (req: Request, res: Response<UploadResponse>) => {
    try {
      if (!req.file) {
        return res.status(400).json({
          success: false,
          error: 'No file uploaded'
        });
      }

      const result = await converter.convert(req.file.path);
      
      // Clean up
      await fs.unlink(req.file.path);
      
      res.json({
        success: true,
        filename: req.file.originalname,
        rows: result.metadata.total_rows_processed,
        data: result.data
      });
    } catch (error) {
      console.error('Conversion error:', error);
      res.status(500).json({
        success: false,
        error: 'Failed to convert Excel file'
      });
    }
  }
);
```

## Error Handling with Custom Types

Create custom error types for better error handling:

```typescript
export class ExcelConversionError extends Error {
  constructor(
    message: string,
    public readonly code: string,
    public readonly details?: Record<string, unknown>
  ) {
    super(message);
    this.name = 'ExcelConversionError';
  }
}

export async function safeConvert<T extends ExcelRow>(
  filePath: string,
  validator: (row: ExcelRow) => row is T
): Promise<T[]> {
  const converter = new ExcelConverter();
  
  try {
    const result = await converter.convert(filePath);
    
    if (result.metadata.warnings && result.metadata.warnings.length > 0) {
      console.warn('Conversion warnings:', result.metadata.warnings);
    }
    
    const validData: T[] = [];
    const invalidRows: number[] = [];
    
    result.data.forEach((row, index) => {
      if (validator(row)) {
        validData.push(row);
      } else {
        invalidRows.push(index + 2);
      }
    });
    
    if (invalidRows.length > 0) {
      throw new ExcelConversionError(
        `Invalid data in rows: ${invalidRows.join(', ')}`,
        'VALIDATION_ERROR',
        { invalidRows, totalRows: result.data.length }
      );
    }
    
    return validData;
  } catch (error) {
    if (error instanceof ExcelConversionError) {
      throw error;
    }
    
    throw new ExcelConversionError(
      `Failed to convert Excel file: ${error}`,
      'CONVERSION_ERROR',
      { originalError: error }
    );
  }
}
```

## Testing with Jest and TypeScript

Example test suite:

```typescript
import { ExcelConverter } from './excel-converter';
import * as fs from 'fs/promises';
import * as path from 'path';

describe('ExcelConverter', () => {
  const converter = new ExcelConverter();
  const testFile = path.join(__dirname, '__fixtures__', 'test-data.xlsx');
  
  beforeAll(async () => {
    await fs.access(testFile);
  });
  
  describe('convert', () => {
    it('should convert Excel to JSON with correct types', async () => {
      const result = await converter.convert(testFile);
      
      expect(result.success).toBe(true);
      expect(Array.isArray(result.data)).toBe(true);
      expect(result.metadata).toMatchObject({
        total_rows_processed: expect.any(Number),
        valid_records: expect.any(Number),
        invalid_records: expect.any(Number),
        processing_time_ms: expect.any(Number)
      });
    });
    
    it('should throw error for non-existent file', async () => {
      await expect(converter.convert('nonexistent.xlsx'))
        .rejects
        .toThrow('File not found');
    });
    
    it('should throw error for invalid file type', async () => {
      const txtFile = path.join(__dirname, 'test.txt');
      await fs.writeFile(txtFile, 'test');
      
      await expect(converter.convert(txtFile))
        .rejects
        .toThrow('Invalid file type');
      
      await fs.unlink(txtFile);
    });
  });
});
```

## Next Steps

- Learn about [Other Language Integrations](/usage/other-languages/)
- Check the [CLI Options Reference](/reference/cli-options/) for all available options
- See the [API Reference](/reference/output-format/) for detailed output format documentation
