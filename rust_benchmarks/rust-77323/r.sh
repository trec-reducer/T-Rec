#!/usr/bin/env bash
set -o nounset
set -o pipefail

readonly SIGNATURE='the compiler unexpectedly panicked. this is a bug.'
readonly OUTPUT="compilation_output.txt"
readonly INPUT=small.rs
trap "rm -f $OUTPUT" EXIT
if timeout -s 9 30 rustup run nightly-2020-09-28 rustc --crate-type=staticlib -C debuginfo=2 -C opt-level=z -C target-cpu=skylake ${INPUT} &> "${OUTPUT}" ; then 
  exit 1
fi
if grep --quiet "${SIGNATURE}" "${OUTPUT}" ; then 
  exit 0
else
  exit 1
fi
