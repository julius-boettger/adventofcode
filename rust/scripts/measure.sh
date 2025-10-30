#!/usr/bin/env fish
# call like `scripts/measure.sh y24d11p1`

cargo build --release --bin $argv[1]

set_color --bold
echo -n "Maximum memory usage"
set_color normal
echo -n " (RSS): "
set_color green --bold
command time -v $argv > /dev/null 2>| \
    grep "Maximum resident set size" | \
    awk '{print $6 "K"}' | \
    numfmt --from si --to si
set_color normal
hyperfine --shell none target/release/$argv[1]
