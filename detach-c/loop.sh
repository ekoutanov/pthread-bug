#!/bin/sh
set -e

src=b.c
tgt=b.out
echo "Compiling $src to $tgt"
gcc -g $src -o $tgt

# continuously test the system, using alternating values for the release profile and the number of test threads
i=1
while [ true ]; do
	echo "Iteration $i [$(date)]"
	i=$(($i + 1))
	./$tgt
done
