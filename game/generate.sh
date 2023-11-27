#!/bin/bash

# Check if output path argument is provided
if [[ -z "$1" ]]; then
  echo "Output path not provided. Usage: ./generate.sh <output_path>"
  exit 1
fi

# Detect the platform
platform=$(uname)

out_dir="$1/generated"

# Create the output directory if it doesn't exist
mkdir -p "$out_dir"
echo "Output directory: $out_dir"

# Run the appropriate commands based on the platform
if [[ "$platform" == "Darwin" ]]; then
  if ! command -v flatc &> /dev/null; then
    echo "FlatBuffers compiler (flatc) not found. Please install it by running 'brew install flatbuffers'."
    exit 1
  fi
  echo "Generating flatbuffers for Mac"
  flatc --gen-all --rust -o "$out_dir" schemas/*.fbs
  flatc --gen-object-api --ts -o sdk/src/generated schemas/*.fbs
elif [[ "$platform" == "Linux" ]]; then
  echo "Generating flatbuffers for Linux"
  if ! command -v flatc &> /dev/null; then
    echo "FlatBuffers compiler (flatc) not found. Please install it by running 'sudo apt-get install flatbuffers'."
    exit 1
  fi
  flatc --gen-all --rust -o "$out_dir" schemas/*.fbs
  flatc --gen-object-api --ts -o sdk/src/generated schemas/*.fbs
elif [[ "$platform" == "Windows" ]]; then
  echo "Generating flatbuffers for Windows"
  if ! command -v flatc &> /dev/null; then
    echo "FlatBuffers compiler (flatc) not found. Please make sure it is in the current directory."
    exit 1
  fi
  ./flatc --gen-all --rust -o "$out_dir" schemas/*.fbs
  ./flatc --gen-object-api --ts -o sdk/src/generated schemas/*.fbs
else
  echo "Unsupported platform: $platform"
  exit 1
fi
echo "Done"

# Documentation comment
# This script generates code using the FlatBuffers compiler (flatc) based on the platform.
# It generates Rust code in the specified output path's "generated" directory and TypeScript code in the specified output path's sdk/src/generated directory.
# Usage: ./generate.sh <output_path>

# Comment on how to use
# To use this script, simply run it in the terminal by executing the command "./generate.sh <output_path>".
# Make sure you have the FlatBuffers compiler (flatc) installed on your system.
# The generated code will be placed in the appropriate directories based on the platform.
# If the platform is not supported, an error message will be displayed and the script will exit with a non-zero status code.
