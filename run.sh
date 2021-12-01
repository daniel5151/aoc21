#!/bin/bash

set -e

YEAR=2021
DAY=$1
INPUTFILE=./inputs/$DAY.txt

mkdir -p ./inputs/

if [ ! -f "$INPUTFILE" ]; then
    if [[ -f cookie.txt ]] ; then
        curl "https://adventofcode.com/$YEAR/day/$DAY/input" -H "cookie: $(cat cookie.txt)" --compressed > "$INPUTFILE"
    else
        echo 'warning: missing cookie.txt. skipping input download...'
    fi
fi

cargo run -- "$@"
