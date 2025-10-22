#!/bin/bash

# Build script for Vortex FM website using Zola

echo "🚀 Building Vortex FM website..."

# Check if Zola is installed
if ! command -v zola &> /dev/null; then
    echo "❌ Zola is not installed. Please install Zola first:"
    echo "   curl -sSL https://get.zola.rs | sh -s - --locked"
    exit 1
fi

# Clean previous build
echo "🧹 Cleaning previous build..."
rm -rf public

# Build the site
echo "🔨 Building site with Zola..."
zola build

if [ $? -eq 0 ]; then
    echo "✅ Site built successfully!"
    echo "📁 Output directory: public/"
    echo "🌐 To preview locally, run: zola serve"
    echo "📤 To deploy to GitHub Pages, push to master branch"
else
    echo "❌ Build failed!"
    exit 1
fi
