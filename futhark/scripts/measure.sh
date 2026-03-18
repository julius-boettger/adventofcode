#!/usr/bin/env fish
# call like `scripts/measure.sh opencl 24/22/1`

echo "compiling $argv[2] for $argv[1]..."
futhark "$argv[1]" "src/$argv[2]/main.fut" -o target/main

set COMMAND "src/$argv[2]/input.sh | target/main"

set_color --bold
echo -n "Maximum memory usage"
set_color normal
echo -n " (RSS): "
set_color green --bold
command time -v $COMMAND > /dev/null 2>| \
    grep "Maximum resident set size" | \
    awk '{print $6 "K"}' | \
    numfmt --from si --to si
set_color normal
hyperfine $COMMAND
