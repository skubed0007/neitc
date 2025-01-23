#!/bin/bash

# Function to ask yes/no questions
ask_yes_no() {
    while true; do
        read -p "$1 [y/n]: " yn
        case $yn in
            [Yy]* ) return 0;;
            [Nn]* ) return 1;;
            * ) echo "Please answer yes or no.";;
        esac
    done
}

# Build for Windows
echo "Building for Windows..."
cargo build --target x86_64-pc-windows-gnu --release
if [ $? -eq 0 ]; then
    echo "Windows build successful!"
else
    echo "Windows build failed!"
    exit 1
fi

# Build for Linux
echo "Building for Linux..."
cargo build --target x86_64-unknown-linux-gnu --release
if [ $? -eq 0 ]; then
    echo "Linux build successful!"
else
    echo "Linux build failed!"
    exit 1
fi

# Ask to build for macOS
if ask_yes_no "Do you want to build for macOS?"; then
    echo "Building for macOS..."
    cargo build --target x86_64-apple-darwin --release
    if [ $? -eq 0 ]; then
        echo "macOS build successful!"
    else
        echo "macOS build failed!"
        exit 1
    fi
else
    echo "Skipping macOS build."
fi

# Ask to build for BSD
if ask_yes_no "Do you want to build for BSD?"; then
    echo "Building for BSD..."

    # Install cross if not already installed
    if ! command -v cross &> /dev/null; then
        echo "Installing cross for BSD cross-compilation..."
        cargo install cross
    fi

    # Build for BSD using cross
    cross build --target x86_64-unknown-freebsd --release
    if [ $? -eq 0 ]; then
        echo "BSD build successful!"
    else
        echo "BSD build failed!"
        exit 1
    fi
else
    echo "Skipping BSD build."
fi

echo "All builds completed successfully!"