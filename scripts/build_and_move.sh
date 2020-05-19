#!/bin/bash


if [ -f ./Cargo.toml ]; then

    # # Build project
    cargo build --release

    if [ -f /usr/local/bin/civa ]; then
        rm /usr/local/bin/civa
    fi

    # # Move to bins
    mv ./target/release/civa /usr/local/bin/

    echo "Moved to /usr/local/bin/"

else
    echo "Run script from project root"
fi
