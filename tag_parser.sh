#!/bin/bash

# store all arguments in an array
TAGS=("$@")

# run search for each tag, store matching test names in array
declare TESTS_MAP
for tag in "${TAGS[@]}"; do
    FOUND_TESTS=$(grep -r --include "*.rs" "#\[hashtag(\"$tag\")]" ./src ./tests | awk -F ':' '{print $2}' | awk -F ' ' '{print $3}')

    # Check if any tests are found
    if [ -z "$FOUND_TESTS" ]; then
        echo "No tests found for tag: $tag"
        exit 1
    fi

    for test in $FOUND_TESTS; do
        TESTS_MAP[$test]=$((TESTS_MAP[$test] + 1))
    done
done

# Check if any tests are selected
if [ ${#TESTS_MAP[@]} -eq 0 ]; then
    echo "No tests selected."
    exit 1
fi

# Prepare test names for cargo test
TEST_NAMES=""
for test in "${!TESTS_MAP[@]}"; do
    if [[ ${TESTS_MAP[$test]} -eq ${#TAGS[@]} ]]; then
        TEST_NAMES="$TEST_NAMES $test"
    fi
done

# Run tests
if [ -n "$TEST_NAMES" ]; then
    echo "Running tests: $TEST_NAMES"
    cargo test -- --nocapture $TEST_NAMES
else
    echo "No tests match all specified tags."
    exit 1
fi
