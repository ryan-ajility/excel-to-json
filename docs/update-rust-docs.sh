#!/bin/bash

# Script to update the integrated Rust API documentation
# Run this after rebuilding the Rust docs with `cargo doc`

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
SOURCE_DIR="$PROJECT_ROOT/target/doc"
DEST_DIR="$SCRIPT_DIR/public/rust-api-docs"

echo "Updating Rust API documentation..."

# Check if source docs exist
if [ ! -d "$SOURCE_DIR" ]; then
    echo "Error: Rust docs not found at $SOURCE_DIR"
    echo "Please run 'cargo doc' first to generate the documentation"
    exit 1
fi

# Remove old docs if they exist
if [ -d "$DEST_DIR" ]; then
    echo "Removing old documentation..."
    rm -rf "$DEST_DIR"
fi

# Copy new docs
echo "Copying documentation from $SOURCE_DIR to $DEST_DIR..."
cp -r "$SOURCE_DIR" "$DEST_DIR"

echo "✅ Rust API documentation updated successfully!"
echo "The docs will be available at /rust-api-docs/ when you run the development server"
