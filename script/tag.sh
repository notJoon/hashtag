#!/bin/bash

# store all arguments in an array
TAGS=("$@")

# Declare two arrays to mimic an associative array
declare -a TESTS_MAP_KEYS
declare -a TESTS_MAP_VALUES

# How many lines can be skipped to find the function identifier (`fn`) in a macro
SKIPPING_LINES=15

# run search for each tag, store matching test names in array
for tag in "${TAGS[@]}"; do
    # Search for test functions with the specified tag in the source directory (especially in the tests directory)
    #
    # I used the `-A` option of the grep command to have it print up to specified(`SKIPPING_LINES`) lines after the matched line, 
    # and awk sets the flag when it finds the #[hashtag(...)] macro, 
    # and then prints and unflags the next occurrence of the fn keyword.
    #
    # To do so, This will find out the `#[hashtag(...)]` macro and the function identifier that follows it.
    FOUND_TESTS=$(grep -E "^\s*#\[hashtag\((?:\"[a-zA-Z0-9_]*\"(?:\,\s*)?)*\"$tag\"(?:\,\s*)?(?:\"[a-zA-Z0-9_]*\"(?:\,\s*)?)*\)\]" -A $SKIPPING_LINES ./tests/*.rs ./src/*.rs | awk '/#\[hashtag\(/ {flag=1; next} flag && /fn / {print; flag=0}')
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

# Check if any tests are selected
if [ ${#TESTS_MAP_KEYS[@]} -eq 0 ]; then
    echo "No tests selected."
    exit 1
fi

# run cargo command for each test
for test in "${!TESTS_MAP_KEYS[@]}"; do
    if [ ${TESTS_MAP_VALUES[$test]} -eq ${#TAGS[@]} ]; then
        cargo test ${TESTS_MAP_KEYS[$test]}
    else
        echo "No tests match all specified tags."
        exit 1
    fi
done
