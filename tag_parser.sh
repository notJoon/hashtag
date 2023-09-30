#!/bin/bash

# store all arguments in an array
TAGS=("$@")

# run search for each tag, store matching test names in array 
declare -a TESTS_MAP
for tag in "${TAGS[@]}"; do
    FOUND_TESTS=$(grep -E "^\s*#\[hashtag\(\"$tag\"\)\]" -A 1 ./tests/*.rs ./src/*.rs)

    # TODO handle attribue macro's next line is not a function definition
    # Process each found test
    IFS=$'\n' # set Internal Field Separator to newline for the loop
    for line in $FOUND_TESTS; do
        # Extract only the function name
        if [[ $line =~ fn\ ([a-zA-Z0-9_]+)\( ]]; then
            # Extract the test name from the function name 
            test_name="${BASH_REMATCH[1]}"
            # Increment the test count for this test name
            index=$(echo "$test_name" | awk '{print $1}')
            # If the test name is not in the map, add it with a count of 1
            TESTS_MAP[$index]=$((TESTS_MAP[$index] + 1))
        fi
    done

    echo "Found tests for tag: $tag"

    # Check if any tests are found
    if [ -z "$FOUND_TESTS" ]; then
        echo "No tests found for tag: $tag"
        exit 1
    fi
done

echo TAGS: "${TAGS[@]}"
echo TESTS_MAP: "${TESTS_MAP[@]}"

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
