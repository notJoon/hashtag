#!/bin/bash

# store all arguments in an array
TAGS=("$@")
echo "TAGS: ${TAGS[@]}"

# Declare two arrays to mimic an associative array
declare -a TESTS_MAP_KEYS
declare -a TESTS_MAP_VALUES

# run search for each tag, store matching test names in array
for tag in "${TAGS[@]}"; do
    FOUND_TESTS=$(grep -E "^\s*#\[hashtag\((?:\"[a-zA-Z0-9_]*\"(?:\,\s*)?)*\"$tag\"(?:\,\s*)?(?:\"[a-zA-Z0-9_]*\"(?:\,\s*)?)*\)\]" -A 1 ./tests/*.rs ./src/*.rs)

    # Process each found test
    IFS=$'\n' # set Internal Field Separator to newline for the loop
    for line in $FOUND_TESTS; do
        # Extract only the function name
        if [[ $line =~ fn\ ([a-zA-Z0-9_]+)\( ]]; then
            test_name="${BASH_REMATCH[1]}"

            # Search for existing key
            found_key=0
            for i in "${!TESTS_MAP_KEYS[@]}"; do
                if [[ "${TESTS_MAP_KEYS[$i]}" == "$test_name" ]]; then
                    TESTS_MAP_VALUES[$i]=$((TESTS_MAP_VALUES[$i] + 1))
                    found_key=1
                    break
                fi
            done

            # If key not found, add new key-value pair
            if [[ $found_key -eq 0 ]]; then
                TESTS_MAP_KEYS+=("$test_name")
                TESTS_MAP_VALUES+=(1)
            fi
        fi
    done

    # Check if any tests are found
    if [ -z "$FOUND_TESTS" ]; then
        echo "No tests found for tag: $tag"
        exit 1
    fi
done

# Display TESTS_MAP
for i in "${!TESTS_MAP_KEYS[@]}"; do
    echo "${TESTS_MAP_KEYS[$i]}: ${TESTS_MAP_VALUES[$i]}"
done

# Check if any tests are selected
if [ ${#TESTS_MAP_KEYS[@]} -eq 0 ]; then
    echo "No tests selected."
    exit 1
fi

TEST_NAMES=""
for test in "${!TESTS_MAP_KEYS[@]}"; do
    if [ ${TESTS_MAP_VALUES[$test]} -eq ${#TAGS[@]} ]; then
        TEST_NAMES+=" ${TESTS_MAP_KEYS[$test]}"
    fi
done

echo "TEST_NAMES: $TEST_NAMES"

if [ "$TEST_NAMES" ]; then
    cargo test -- $TEST_NAMES
else
    echo "No tests match all specified tags."
    exit 1
fi
