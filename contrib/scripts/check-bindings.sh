#!/bin/bash

set -euo pipefail

# Check bindings
buildargs=(
    "-p nostr-sdk-cpp"
    "-p nostr-sdk-ffi"
    "-p nostr-sdk-flutter"
    "-p nostr-sdk-js --target wasm32-unknown-unknown"
)

for arg in "${buildargs[@]}"; do
    echo  "Checking '$arg'"

    cargo check $arg

    if [[ $arg != *"--target wasm32-unknown-unknown"* ]];
    then
        cargo test $arg
    fi

    cargo clippy $arg -- -D warnings

    echo
done
