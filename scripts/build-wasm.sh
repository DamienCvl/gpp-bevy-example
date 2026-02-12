#!/bin/bash
set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
OUTPUT_DIR="${PROJECT_ROOT}/docs"
BIN_DIR="${PROJECT_ROOT}/src/bin"

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Discover all binaries in src/bin/
BINARIES=($(ls -d "$BIN_DIR"/*/ 2>/dev/null | xargs -n 1 basename | sort))

if [ ${#BINARIES[@]} -eq 0 ]; then
    echo "Error: No binaries found in $BIN_DIR"
    exit 1
fi

echo "Found ${#BINARIES[@]} binary(ies): ${BINARIES[*]}"

for BINARY in "${BINARIES[@]}"; do
    # Convert underscores to dashes for cargo binary name
    BINARY_NAME="${BINARY//_/-}"
    echo "Building $BINARY_NAME for WASM..."

    # Build with wasm target
    cargo build --profile wasm-release \
        --bin "$BINARY_NAME" \
        --target wasm32-unknown-unknown

    # Generate WASM bindings (keep original name for file, convert - to _)
    WASM_FILE="target/wasm32-unknown-unknown/wasm-release/${BINARY_NAME}.wasm"
    OUTPUT_SUBDIR="$OUTPUT_DIR/${BINARY}"

    mkdir -p "$OUTPUT_SUBDIR"

    wasm-bindgen "$WASM_FILE" \
        --out-dir "$OUTPUT_SUBDIR" \
        --out-name module \
        --target web \
        --no-typescript

    echo "Generated WASM for $BINARY_NAME in $OUTPUT_SUBDIR"

    cp "assets/index.html" "$OUTPUT_SUBDIR/index.html"
done


echo "WASM build complete!"
