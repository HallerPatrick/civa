#!/bin/bash

if [ -f ./Cargo.toml ]; then

    # # Build project
    cargo build --release

    if [ -f /usr/local/bin/civa ]; then
        rm /usr/local/bin/civa
    fi

    # # Move to bins
    mv ./target/release/civa /usr/local/bin/

    out=bash -c "cat /etc/shells | grep civa"

    if [ $out  ]; then
        echo "Found civa in allowed shells"
    else
        sudo bash -c "echo '/usr/local/bin/civa' >> /etc/shells"
    fi

    # # Set as standard shell
    chsh -s /usr/local/bin/civa

    # echo "Run command to add civa to allowed shells: \n\t chsh -s /usr/local/bin/civa"

else
    echo "Run script from project root"
fi
