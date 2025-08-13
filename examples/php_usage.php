<?php
/**
 * Example PHP script demonstrating how to use the excel-to-json binary
 * with JSON output format for both single and multiple sheets
 */

// Configuration
$binaryPath = './target/debug/excel-to-json'; // Adjust path as needed
$excelFile = 'resources/Item Master Field Values.xlsx'; // Your Excel file
$sheetName = 'Cascade Fields';

/**
 * Import data from single sheet
 */
function importExcelSheet($binaryPath, $excelFile, $sheetName = null) {
    // Build command
    $command = sprintf(
        '%s %s',
        escapeshellcmd($binaryPath),
        escapeshellarg($excelFile)
    );
    
    if ($sheetName) {
        $command .= ' -s ' . escapeshellarg($sheetName);
    }
    
    $command .= ' 2>/dev/null';
    
    // Execute command
    $output = shell_exec($command);
    
    if ($output === null) {
        throw new Exception('Failed to execute excel-to-json binary');
    }
    
    // Parse JSON output
    $result = json_decode($output, true);
    
    if (json_last_error() !== JSON_ERROR_NONE) {
        throw new Exception('Invalid JSON response: ' . json_last_error_msg());
    }
    
    return $result;
}

/**
 * Import data from multiple sheets
 */
function importMultipleSheets($binaryPath, $excelFile, $sheetNames = null, $allSheets = false) {
    // Build command
    $command = sprintf(
        '%s %s',
        escapeshellcmd($binaryPath),
        escapeshellarg($excelFile)
    );
    
    if ($allSheets) {
        $command .= ' --all-sheets';
    } elseif (is_array($sheetNames)) {
        foreach ($sheetNames as $sheetName) {
            $command .= ' -s ' . escapeshellarg($sheetName);
        }
    }
    
    $command .= ' 2>/dev/null';
    
    // Execute command
    $output = shell_exec($command);
    
    if ($output === null) {
        throw new Exception('Failed to execute excel-to-json binary');
    }
    
    // Parse JSON output
    $result = json_decode($output, true);
    
    if (json_last_error() !== JSON_ERROR_NONE) {
        throw new Exception('Invalid JSON response: ' . json_last_error_msg());
    }
    
    return $result;
}

// Example 1: Single Sheet Import
echo "=== EXAMPLE 1: Single Sheet Import ===\n";
try {
    echo "Importing from: $excelFile\n";
    echo "Sheet: $sheetName\n";
    echo str_repeat('-', 50) . "\n";
    
    // Import single sheet
    $result = importExcelSheet($binaryPath, $excelFile, $sheetName);
    
    if ($result['success']) {
        $metadata = $result['metadata'];
        
        echo "✓ Import successful!\n";
        echo "  Total rows processed: {$metadata['total_rows_processed']}\n";
        echo "  Valid records: {$metadata['valid_records']}\n";
        echo "  Invalid records: {$metadata['invalid_records']}\n";
        echo "  Processing time: {$metadata['processing_time_ms']}ms\n";
        
        // Handle the data array - it will be array of sheet objects
        if (!empty($result['data'])) {
            $sheetData = $result['data'][0]; // First (and only) sheet
            $rows = $sheetData['rows'];
            
            echo "\n  Sheet: {$sheetData['sheet']}\n";
            echo "  Number of rows: " . count($rows) . "\n";
            
            // Show first few records
            if (!empty($rows)) {
                echo "\n  First 3 records:\n";
                for ($i = 0; $i < min(3, count($rows)); $i++) {
                    $row = $rows[$i];
                    echo "    Record " . ($i + 1) . ": ";
                    echo json_encode($row, JSON_UNESCAPED_UNICODE) . "\n";
                }
            }
        }
    } else {
        echo "✗ Import failed!\n";
        echo "Error: {$result['error']}\n";
    }
    
} catch (Exception $e) {
    echo "✗ Exception occurred: " . $e->getMessage() . "\n";
}

echo "\n";

// Example 2: Multiple Sheets Import
echo "=== EXAMPLE 2: Multiple Sheets Import ===\n";
try {
    // Import multiple specific sheets
    $sheetsToImport = ['Main', 'Sub', 'Cascade Fields'];
    echo "Importing sheets: " . implode(', ', $sheetsToImport) . "\n";
    echo str_repeat('-', 50) . "\n";
    
    $result = importMultipleSheets($binaryPath, $excelFile, $sheetsToImport);
    
    if ($result['success']) {
        $metadata = $result['metadata'];
        
        echo "✓ Multi-sheet import successful!\n";
        echo "  Total rows processed: {$metadata['total_rows_processed']}\n";
        echo "  Valid records: {$metadata['valid_records']}\n";
        echo "  Invalid records: {$metadata['invalid_records']}\n";
        echo "  Processing time: {$metadata['processing_time_ms']}ms\n";
        
        if (!empty($metadata['warnings'])) {
            echo "\n  ⚠ Warnings:\n";
            foreach ($metadata['warnings'] as $warning) {
                echo "    - $warning\n";
            }
        }
        
        // Process each sheet
        echo "\n  Sheets processed:\n";
        foreach ($result['data'] as $sheetData) {
            $sheetName = $sheetData['sheet'];
            $rows = $sheetData['rows'];
            
            echo "    - {$sheetName}: " . count($rows) . " rows\n";
            
            // Show sample data from each sheet
            if (!empty($rows)) {
                $sampleRow = $rows[0];
                echo "      Sample columns: " . implode(', ', array_keys($sampleRow)) . "\n";
            }
        }
    } else {
        echo "✗ Multi-sheet import failed!\n";
        echo "Error: {$result['error']}\n";
    }
    
} catch (Exception $e) {
    echo "✗ Exception occurred: " . $e->getMessage() . "\n";
}

echo "\n";

// Example 3: Import All Sheets
echo "=== EXAMPLE 3: Import All Sheets ===\n";
try {
    echo "Importing all sheets from: $excelFile\n";
    echo str_repeat('-', 50) . "\n";
    
    $result = importMultipleSheets($binaryPath, $excelFile, null, true);
    
    if ($result['success']) {
        $metadata = $result['metadata'];
        
        echo "✓ All sheets import successful!\n";
        echo "  Total sheets: " . count($result['data']) . "\n";
        echo "  Total rows processed: {$metadata['total_rows_processed']}\n";
        echo "  Valid records: {$metadata['valid_records']}\n";
        echo "  Invalid records: {$metadata['invalid_records']}\n";
        echo "  Processing time: {$metadata['processing_time_ms']}ms\n";
        
        // List all sheets with their row counts
        echo "\n  All sheets in workbook:\n";
        $totalDataRows = 0;
        
        foreach ($result['data'] as $sheetData) {
            $sheetName = $sheetData['sheet'];
            $rowCount = count($sheetData['rows']);
            $totalDataRows += $rowCount;
            
            echo "    - {$sheetName}: {$rowCount} data rows\n";
        }
        
        echo "\n  Total data rows across all sheets: $totalDataRows\n";
        
        // Example: Process data by sheet type
        echo "\n  Processing data by sheet...\n";
        foreach ($result['data'] as $sheetData) {
            $sheetName = $sheetData['sheet'];
            $rows = $sheetData['rows'];
            
            if (!empty($rows)) {
                // Example processing based on sheet name
                switch (strtolower($sheetName)) {
                    case 'cascade fields':
                        echo "    → Processing cascade field definitions: " . count($rows) . " items\n";
                        break;
                    case 'main':
                        echo "    → Processing main categories: " . count($rows) . " items\n";
                        break;
                    case 'sub':
                        echo "    → Processing sub-categories: " . count($rows) . " items\n";
                        break;
                    default:
                        echo "    → Processing {$sheetName}: " . count($rows) . " items\n";
                        break;
                }
            }
        }
    } else {
        echo "✗ All sheets import failed!\n";
        echo "Error: {$result['error']}\n";
    }
    
} catch (Exception $e) {
    echo "✗ Exception occurred: " . $e->getMessage() . "\n";
}

echo "\n" . str_repeat('=', 60) . "\n";
echo "All examples completed!\n";
echo "\nTips for integration:\n";
echo "- Use single sheet import for simple cases\n";
echo "- Use multi-sheet import when you need specific sheets\n";
echo "- Use all-sheets import for complete workbook processing\n";
echo "- Always check the 'success' field before processing data\n";
echo "- Handle warnings in the metadata for data quality insights\n";
