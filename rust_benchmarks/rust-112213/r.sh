#!/usr/bin/env bash

set -o nounset

readonly FILE=small.rs
readonly VERSION="nightly-2023-06-03"
readonly FLAGS_BUGGY="-Zmir-opt-level=3 -Copt-level=2"
readonly FLAGS_CORRECT1="-Zmir-opt-level=3 -Copt-level=1"
readonly FLAGS_CORRECT2="-Zmir-opt-level=2 -Copt-level=1"

readonly BIN_BUGGY="./buggy.out"
if ! timeout -s 9 60 rustup run "${VERSION}" rustc ${FLAGS_BUGGY} -o "${BIN_BUGGY}" "${FILE}" ; then
  exit 1
fi

readonly BIN_CORRECT1="./correct1.out"
if ! timeout -s 9 60 rustup run "${VERSION}" rustc ${FLAGS_CORRECT1} -o "${BIN_CORRECT1}" "${FILE}" ; then
  exit 1
fi

readonly BIN_CORRECT2="./correct2.out"
if ! timeout -s 9 60 rustup run "${VERSION}" rustc ${FLAGS_CORRECT2} -o "${BIN_CORRECT2}" "${FILE}" ; then
  exit 1
fi

readonly OUTPUT_BUGGY="output_buggy.txt"
readonly OUTPUT_CORRECT1="output_correct1.txt"
readonly OUTPUT_CORRECT2="output_correct2.txt"
if ! (timeout -s 9 30 "${BIN_BUGGY}") >& "${OUTPUT_BUGGY}" ; then
  exit 1
fi

if ! (timeout -s 9 30 "${BIN_CORRECT1}") >& "${OUTPUT_CORRECT1}" ; then
  exit 1
fi

if ! (timeout -s 9 30 "${BIN_CORRECT2}") >& "${OUTPUT_CORRECT2}" ; then
  exit 1
fi

if diff "${OUTPUT_BUGGY}" "${OUTPUT_CORRECT1}" >& /dev/null ; then
  exit 1
fi

if ! diff "${OUTPUT_CORRECT1}" "${OUTPUT_CORRECT2}" >& /dev/null ; then
  exit 1
fi
exit 0

