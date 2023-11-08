#!/bin/bash

# Define the name of your project binary
BINARY_NAME="c2p"

# Build the release binary using cargo
echo "Building $BINARY_NAME..."
cargo build --release

# Check if the build succeeded
if [ $? -eq 0 ]; then
    echo "Build successful. Installing $BINARY_NAME to /usr/local/bin"

    # Copy the binary to /usr/local/bin
    sudo cp ./target/release/$BINARY_NAME /usr/local/bin/

    # Verify that the binary is installed
    if [ -f "/usr/local/bin/$BINARY_NAME" ]; then
        echo "$BINARY_NAME installed successfully."
    else
        echo "Installation failed."
        exit 1
    fi
else
    echo "Build failed."
    exit 1
fi

# Cleanup (optional)
# Uncomment the next line if you want to remove the build artifacts after installation
# cargo clean

exit 0
