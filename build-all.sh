#!/bin/bash
# Use Cross to automatically build all targets
# (This really needs cleaning up)

BINARY_NAME="elf-table"
ZIP_DIR="target/zip-releases"

mkdir -p "$ZIP_DIR"

if [[ "$(uname)" == "Darwin" ]]; then
    echo "-> Building for macOS (native)"
    rustup target add aarch64-apple-darwin x86_64-apple-darwin
    cargo build --release --target aarch64-apple-darwin
    cargo build --release --target x86_64-apple-darwin

    mkdir -p target/universal-apple-darwin
    lipo -create \
        target/aarch64-apple-darwin/release/$BINARY_NAME \
        target/x86_64-apple-darwin/release/$BINARY_NAME \
        -output target/universal-apple-darwin/$BINARY_NAME
fi

CROSS_TARGETS=(
    x86_64-unknown-linux-gnu
    aarch64-unknown-linux-gnu
    x86_64-unknown-freebsd
    x86_64-pc-windows-gnu
)

for target in "${CROSS_TARGETS[@]}"; do
    # Build
    echo "-> Begin build for: $target"
    cross build --release --target $target

    # Create ZIP release
done

for target in "${CROSS_TARGETS[@]}"; do
    # Create ZIP release
    EXT=""
    if [[ "$target" == *win* ]]; then
        EXT=".exe"
    fi

    BIN_PATH="target/$target/release/$BINARY_NAME$EXT"
    cp "$BIN_PATH" "$ZIP_DIR/"
    cd "$ZIP_DIR"
    zip "$BINARY_NAME-$target.zip" "$BINARY_NAME$EXT"
    rm "$BINARY_NAME$EXT"
    cd -
done

if [[ "$(uname)" == "Darwin" ]]; then
    cp target/aarch64-apple-darwin/release/$BINARY_NAME "$ZIP_DIR/"
    cd "$ZIP_DIR"
    zip "$BINARY_NAME-aarch64-apple-darwin.zip" "$BINARY_NAME"
    rm "$BINARY_NAME"
    cd -

    cp target/x86_64-apple-darwin/release/$BINARY_NAME "$ZIP_DIR/"
    cd "$ZIP_DIR"
    zip "$BINARY_NAME-x86_64-apple-darwin.zip" "$BINARY_NAME"
    rm "$BINARY_NAME"
    cd -

    cp target/universal-apple-darwin/$BINARY_NAME "$ZIP_DIR/"
    cd "$ZIP_DIR"
    zip "$BINARY_NAME-universal-apple-darwin.zip" "$BINARY_NAME"
    rm "$BINARY_NAME"
    cd -
fi