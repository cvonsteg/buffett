#!/usr/bin/bash

cargo -b --release

ln -s $PWD/target/release/buffett /usr/local/bin/buffett