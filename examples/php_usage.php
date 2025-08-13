<?php
/**
 * Example PHP script demonstrating how to use the import_cascade_fields binary
 * with the PHP array output format
 */

// Configuration
$binaryPath = './target/debug/import_cascade_fields'; // Adjust path as needed
$excelFile = 'test_data.xlsx'; // Your Excel file
$sheetName = 'Cascade Fields';

/**
 * Simple function to import cascade fields and return as PHP array
 */
function importCascadeFields($binaryPath, $excelFile, $sheetName = 'Cascade Fields') {
    // Build command with PHP output format
    $command = sprintf(
        '%s %s -s %s -o php 2>/dev/null',
        escapeshellcmd($binaryPath),
        escapeshellarg($excelFile),
        escapeshellarg($sheetName)
    );
    
    // Execute command
    $output = shell_exec($command);
    
    if ($output === null) {
        throw new Exception('Failed to execute import_cascade_fields binary');
    }
    
    // Parse JSON output
    $result = json_decode($output, true);
    
    if (json_last_error() !== JSON_ERROR_NONE) {
        throw new Exception('Invalid JSON response: ' . json_last_error_msg());
    }
    
    return $result;
}

// Main execution
try {
    echo "Importing cascade fields from: $excelFile\n";
    echo "Sheet: $sheetName\n";
    echo str_repeat('-', 50) . "\n\n";
    
    // Import the data
    $result = importCascadeFields($binaryPath, $excelFile, $sheetName);
    
    // Check if successful
    if ($result['success']) {
        $cascadeFields = $result['data'];
        $metadata = $result['metadata'];
        
        echo "✓ Import successful!\n";
        echo "  Total rows processed: {$metadata['total_rows_processed']}\n";
        echo "  Valid records: {$metadata['valid_records']}\n";
        echo "  Invalid records: {$metadata['invalid_records']}\n";
        echo "  Processing time: {$metadata['processing_time_ms']}ms\n\n";
        
        // Display warnings if any
        if (!empty($metadata['warnings'])) {
            echo "⚠ Warnings:\n";
            foreach ($metadata['warnings'] as $warning) {
                echo "  - $warning\n";
            }
            echo "\n";
        }
        
        // Display the data
        echo "Data (showing first 5 records):\n";
        echo str_repeat('-', 50) . "\n";
        
        $count = 0;
        foreach ($cascadeFields as $index => $field) {
            if ($count >= 5) break;
            
            echo "Record " . ($index + 1) . ":\n";
            
            // Each field is an associative array with the following keys
            echo "  Main: {$field['main_label']} = {$field['main_value']}\n";
            if ($field['main_description']) {
                echo "    Description: {$field['main_description']}\n";
            }
            
            echo "  Sub: {$field['sub_label']} = {$field['sub_value']}\n";
            if ($field['sub_description']) {
                echo "    Description: {$field['sub_description']}\n";
            }
            
            echo "  Major: {$field['major_label']} = {$field['major_value']}\n";
            if ($field['major_description']) {
                echo "    Description: {$field['major_description']}\n";
            }
            
            echo "  Minor: {$field['minor_label']} = {$field['minor_value']}\n";
            if ($field['minor_description']) {
                echo "    Description: {$field['minor_description']}\n";
            }
            
            echo "  Timestamps: Created {$field['created_at']}, Updated {$field['updated_at']}\n";
            echo "\n";
            
            $count++;
        }
        
        if (count($cascadeFields) > 5) {
            echo "... and " . (count($cascadeFields) - 5) . " more records\n";
        }
        
        // Example: Process the data for your application
        echo "\n" . str_repeat('-', 50) . "\n";
        echo "Processing data for application use...\n\n";
        
        // Group by main value for analysis
        $grouped = [];
        foreach ($cascadeFields as $field) {
            $mainValue = $field['main_value'];
            if (!isset($grouped[$mainValue])) {
                $grouped[$mainValue] = [];
            }
            $grouped[$mainValue][] = $field;
        }
        
        echo "Data grouped by main value:\n";
        foreach ($grouped as $mainValue => $fields) {
            echo "  $mainValue: " . count($fields) . " records\n";
        }
        
    } else {
        // Handle error
        echo "✗ Import failed!\n";
        echo "Error: {$result['error']}\n";
        
        if (!empty($result['data'])) {
            echo "Details:\n";
            print_r($result['data']);
        }
    }
    
} catch (Exception $e) {
    echo "✗ Exception occurred: " . $e->getMessage() . "\n";
    exit(1);
}

echo "\n" . str_repeat('=', 50) . "\n";
echo "Example complete!\n";
