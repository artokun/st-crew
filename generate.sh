#!/bin/bash

# Detect the platform
platform=$(uname)

# Run the appropriate commands based on the platform
if [[ "$platform" == "Darwin" ]]; then
  if ! command -v flatc &> /dev/null; then
    echo "FlatBuffers compiler (flatc) not found. Please install it by running 'brew install flatbuffers'."
    exit 1
  fi
  echo "Generating flatbuffers for Mac"
  flatc --gen-all --rust -o src/generated schemas/*.fbs
  flatc --gen-object-api --ts -o sdk/src/models schemas/*.fbs
elif [[ "$platform" == "Linux" ]]; then
  echo "Generating flatbuffers for Linux"
  if ! command -v flatc &> /dev/null; then
    echo "FlatBuffers compiler (flatc) not found. Please install it by running 'sudo apt-get install flatbuffers'."
    exit 1
  fi
  flatc --gen-all --rust -o src/generated schemas/*.fbs
  flatc --gen-object-api --ts -o sdk/src/models schemas/*.fbs
elif [[ "$platform" == "Windows" ]]; then
  echo "Generating flatbuffers for Windows"
  if ! command -v flatc &> /dev/null; then
    echo "FlatBuffers compiler (flatc) not found. Please make sure it is in the current directory."
    exit 1
  fi
  ./flatc --gen-all --rust -o src/generated schemas/*.fbs
  ./flatc --gen-object-api --ts -o sdk/src/models schemas/*.fbs
else
  echo "Unsupported platform: $platform"
  exit 1
fi
echo "Done"

# Documentation comment
# This script generates code using the FlatBuffers compiler (flatc) based on the platform.
# It generates Rust code in the "src/generated" directory and TypeScript code in the "sdk/src/models" directory.
# Usage: ./generate.sh

# Comment on how to use
# To use this script, simply run it in the terminal by executing the command "./generate.sh".
# Make sure you have the FlatBuffers compiler (flatc) installed on your system.
# The generated code will be placed in the appropriate directories based on the platform.
# If the platform is not supported, an error message will be displayed and the script will exit with a non-zero status code.
