#!/usr/bin/env bash
set -o nounset
set -o pipefail

readonly OUTPUT="temp_compilation_output.tmp.txt"
readonly INPUT=small.rs
trap "rm -f $OUTPUT" EXIT
if timeout -s 9 30 rustup run nightly-2020-10-15 rustc --crate-type=staticlib -C debuginfo=2 -C opt-level=s -C target-cpu=skylake ${INPUT} &> "${OUTPUT}" ; then 
  exit 1
fi

if ! grep --quiet --fixed-strings "error: internal compiler error: compiler/rustc_typeck/src/collect/type_of.rs:115:26: unexpected anon const res PrimTy(Uint(Usize)) in path: Path { span: small.rs:7:9: 7:23 (#0), res: PrimTy(Uint(Usize)), segments: [PathSegment { ident: usize#0, hir_id: Some(HirId { owner: DefId(0:3 ~ small[8787]::main), local_id: 57 }), res: Some(Err), args: Some(GenericArgs { args: [Const(ConstArg { value: AnonConst { hir_id: HirId { owner: DefId(0:3 ~ small[8787]::main), local_id: 54 }, body: BodyId { hir_id: HirId { owner: DefId(0:3 ~ small[8787]::main), local_id: 56 } } }, span: small.rs:7:17: 7:21 (#0) })], bindings: [], parenthesized: false }), infer_args: false }] }" "${OUTPUT}" ; then
  exit 1
fi

if ! grep --quiet --fixed-strings "thread 'rustc' panicked at 'Box<Any>'" "${OUTPUT}" ; then
  exit 1
fi

if ! grep --quiet --fixed-strings "note: the compiler unexpectedly panicked. this is a bug." "${OUTPUT}" ; then
  exit 1
fi
exit 0
