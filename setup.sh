#! /bin/sh

year=2018
day=$1

if [ "$day" = "" ]; then
    day=$(TZ=US/Eastern date '+%d' | sed 's/^0//')
fi

if ! git diff --exit-code > /dev/null; then
    echo There are uncommitted changes 2>&1
    exit 1
elif [ -e input/day$day.txt ]; then
    echo Already have day $day 2>&1
    exit 1
fi

# download input (assumes w3m is already logged in)
url=https://adventofcode.com/2018/day/$day/input > input/day$day.txt
echo Fetching $url...
mkdir -p input
w3m $url > input/day$day.txt
file input/day$day.txt

# setup new rust module
echo "Creating src/day$day.rs"
cat > src/day$day.rs <<EOF
use super::{Part,Part::*};

pub fn run(part: Part, input: &str) {
    match part {
        One => println!(),
        Two => println!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
        ""
    }

    #[test]
    fn test_run() {
    }
}
EOF
echo "Editing src/main.rs"
echo "mod day$day;" >> src/main.rs
sed -i "0,/.*not implemented.*/s//        $day => day${day}::run(part, \&input),\n&/" src/main.rs
