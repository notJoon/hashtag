#!/bin/bash

# expected output
EXPECTED_FOO_OUTPUT=true
EXPECTED_BAR_OUTPUT=true
EXPECTED_BAZ_OUTPUT=true

ACTUAL_FOO_OUTPUT=$(./tag_parser.sh "foo")
ACTUAL_BAR_OUTPUT=$(./tag_parser.sh foo bar)
ACTUAL_BAZ_OUTPUT=$(./tag_parser.sh foo bar baz)

assert_output() {
    if [ "$1" != "$2" ]; then
        echo "Test failed."
        echo "Expected output: $1"
        echo "Actual output: $2"
        exit 1
    fi
}

assert_output "$EXPECTED_FOO_OUTPUT" "$ACTUAL_FOO_OUTPUT"
assert_output "$EXPECTED_BAR_OUTPUT" "$ACTUAL_BAR_OUTPUT"
assert_output "$EXPECTED_BAZ_OUTPUT" "$ACTUAL_BAZ_OUTPUT"