#!/usr/bin/env fish
# call like `scripts/run.sh opencl 24/22/1`

echo -n "compiling $argv[2] for $argv[1]..."
futhark "$argv[1]" "src/$argv[2]/main.fut" -o target/main

echo " running..."
"src/$argv[2]/input.sh" | target/main
