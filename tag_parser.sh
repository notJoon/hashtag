#!/bin/bash

# store all arguments in an array
TAGS=("$@")

# run search for each tag, store matching test names in array
declare -A TESTS_MAP
for tag in "${TAGS[@]}"; do
    FOUND_TESTS=$(
        grep -r --include "*.rs" "#\[hashtag(\"$tag\"\)]" ./src 
        | awk -F ':' '{print $2}' 
        | awk -F ' ' '{print $3}'
    )

    for test in $FOUND_TESTS; do
        TESTS_MAP[$test]=$((TESTS_MAP[$test] + 1))
    done
done

# run only tests that include all specified tags
for test in "${!TESTS_MAP[@]}"; do
    if [[ ${TESTS_MAP[$test]} -eq ${#TAGS[@]} ]]; then
        echo "Running test: $test"
        cargo test -- --nocapture $test
    fi
done

# ./tag_parser.sh tag1 tag2 tag3 ... tagN