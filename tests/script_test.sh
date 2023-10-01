#!/bin/bash

ACTUAL_FOO_OUTPUT=$(./tag_parser.sh "foo")
ACTUAL_BAR_OUTPUT=$(./tag_parser.sh foo bar)
ACTUAL_BAZ_OUTPUT=$(./tag_parser.sh foo bar baz)

