#!/bin/bash

# Build wheels for all Mac architectures
echo "Building wheels for macOS..."

# Clean previous builds
rm -rf dist/*
rm -rf target/wheels/*

# Build for Intel Macs (x86_64)
echo "Building for Intel Macs (x86_64)..."
python3 -m maturin build --release --strip

# Build for Apple Silicon (ARM64)
echo "Building for Apple Silicon (ARM64)..."
python3 -m maturin build --release --strip --target aarch64-apple-darwin

# Copy all wheels to dist/
echo "Copying wheels to dist/..."
cp target/wheels/*.whl dist/

echo "✅ Build complete! Wheels available in dist/:"
ls -la dist/*.whl

echo ""
echo "To publish to PyPI:"
echo "1. Install twine: python3 -m pip install twine"
echo "2. Upload to PyPI: python3 -m twine upload dist/*" 