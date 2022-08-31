#!/bin/sh
set -e

echo "Compiling"
cargo build --release

# continuously test the system, using alternating values for the release profile and the number of test threads
i=1
while [ true ]; do
	echo "Iteration $i [$(date)]"
	i=$(($i + 1))
	cargo run --release
done
