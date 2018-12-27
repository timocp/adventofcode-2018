#! /bin/sh

cargo build --release || exit 1

if [ ! -z "$1" ]; then
    ./target/release/adventofcode-2018 $@
else
    for d in $(ls -1 input/day*.txt | grep -o '[[:digit:]]\+' | sort -n); do
        echo -n "Day $d Part One: "
        ./target/release/adventofcode-2018 $d 1
        echo -n "Day $d Part Two: "
        ./target/release/adventofcode-2018 $d 2
    done
fi
