#!/usr/bin/env bash

set -o nounset
set -o xtrace

readonly TIMEOUT=5
readonly FILE=small.rs
readonly VERSION="nightly-2021-12-09"
readonly FLAGS_BUGGY="-O -Z mir-opt-level=4"
readonly FLAGS_CORRECT1="-O -Z mir-opt-level=3"

readonly BIN_BUGGY="./buggy.out"
if ! timeout -s 9 60 rustup run "${VERSION}" rustc ${FLAGS_BUGGY} -o "${BIN_BUGGY}" "${FILE}" &> /dev/null ; then
  exit 1
fi

readonly BIN_CORRECT1="./correct1.out"
if ! timeout -s 9 60 rustup run "${VERSION}" rustc ${FLAGS_CORRECT1} -o "${BIN_CORRECT1}" "${FILE}" &> /dev/null ; then
  exit 1
fi

readonly OUTPUT_BUGGY="output_buggy.txt"
readonly OUTPUT_CORRECT1="output_correct1.txt"

buggy_failed=false
correct_failed=false
for _ in {1..5} ; do
  if ! timeout -s 9 $TIMEOUT "${BIN_BUGGY}" >& "${OUTPUT_BUGGY}" ; then
    buggy_failed=true
  fi

  if ! timeout -s 9 $TIMEOUT "${BIN_CORRECT1}" >& "${OUTPUT_CORRECT1}" ; then
    correct_failed=true
  fi
done

if $buggy_failed && ! $correct_failed ; then
  exit 0
fi

exit 1

