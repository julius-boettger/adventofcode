#!/bin/sh
# call like `scripts/profile.sh y24d11p1`

# required for samply
echo -1 | sudo tee /proc/sys/kernel/perf_event_paranoid > /dev/null

# heap profiling + open viewer
cargo profile-heap --bin $1
xdg-open dhat_view/dh_view.html
# profiling (automatically opens viewer)
cargo build --release --bin $1
samply record target/release/$1
