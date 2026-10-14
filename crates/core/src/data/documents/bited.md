# bited: small integers, explicit contracts, and a faster verified encoder

Dmytro Yemelianov · 12 October 2026 · bited 0.1.0

An integer codec is easy to underestimate. The happy path is a loop that emits seven bits at a time. The contract also has to answer what happens when a buffer ends halfway through a number, when the number overflows, when its representation is longer than necessary, or when a caller supplies too little output space.

**bited** is one small Rust library for 32- and 64-bit base-128 varints and signed ZigZag mappings, with a corresponding Lean 4 proof package. It is `no_std`, has zero runtime dependencies, forbids unsafe code, and performs no heap allocation. Its encoder produces the shortest representation within this format. Its runtime performance is measured separately.

The project began with an explicit specification and 351 conformance cases. All eighteen required proof obligations are now proved, including correspondence between the Rust-derived model and the mathematical contract. A capacity specialization reduces full-width `u64` encoding time by 52.5% in the matched Apple M5 experiment, with a small one-byte regression recorded alongside it.

## The format in thirty seconds

Each varint byte carries seven payload bits. The high bit means another byte follows. Payload groups appear least significant first: 300 becomes `ac 02`, because `44 + 2 * 128 = 300`.

```text
Unsigned value  Canonical bytes (hex)          Length
--------------  -----------------------------  ------
0               00                             1
127             7f                             1
128             80 01                          2
300             ac 02                          2
16,383          ff 7f                          2
16,384          80 80 01                       3
u64::MAX        ff ff ff ff ff ff ff ff ff 01  10
```


The exact length is one for zero, and `ceil(bit_length(value) / 7)` otherwise. The width bound is five bytes for `u32` and ten for `u64`.

ZigZag places small signed magnitudes near zero in the unsigned domain: `0 -> 0`, `-1 -> 1`, `1 -> 2`, `-2 -> 3`, `2 -> 4`. At the mathematical level, it maps nonnegative `x` to `2x` and negative `x` to `-2x-1`. The implementation handles the full fixed-width signed range, including the minimum signed integer.

## A deterministic core with thin adapters

The boundaries are logical boundaries inside one library. Callers do not assemble separate services, processes, or utility packages.

```text
values / bytes -> thin adapters -> Rust codec
                                      |
                       Charon -> Aeneas -> Lean contracts
                                      |
                          proof replay + axiom audit
```

**Unsigned and signed entry points**
Caller gets: u32/u64 varints and i32/i64 ZigZag wrappers

**Fixed-array encoding**
Caller gets: Owned stack output and its used prefix

**Caller-buffer encoding**
Caller gets: No allocation; insufficient capacity leaves every byte unchanged

**Prefix decoding**
Caller gets: A value, canonicality flag, and precise consumption count

**Exact decoding**
Caller gets: One complete value; trailing bytes are an error

**Incremental decoding**
Caller gets: Bounded state carried across input chunks

**Explicit policies**
Caller gets: Canonical-only or bounded noncanonical acceptance


For example, `80 00` denotes zero with a redundant byte. Canonical decoding rejects it; bounded noncanonical decoding accepts zero and marks it noncanonical. Neither policy turns into an unbounded scan. Overflow, excessive length, truncation, noncanonical representation, and trailing bytes have specified behavior and consumption counts.

```rust
use bited::{decode_u64, encode_u64, DecodePolicy};

let bytes = encode_u64(300);
assert_eq!(bytes.as_bytes(), &[0xac, 0x02]);
let number = decode_u64(bytes.as_bytes(), DecodePolicy::Canonical).unwrap();
assert_eq!((number.value, number.consumed, number.canonical), (300, 2, true));
```

## What the proofs establish

The Lean package covers all eighteen obligations in the specification. They include ZigZag bijection, encoder and decoder correctness, canonical uniqueness, minimal encoded length, streaming equivalence, bounds, buffer preservation, and Rust-to-model correspondence.

**Lean contracts and refinement**
What it supports: Mathematical behavior and correspondence of extracted Rust core entry points
Boundary: Pinned extraction profile: Apple ARM, 64-bit usize

**Axiom audit and fresh proof replay**
What it supports: Stored proofs check with the pinned Lean kernel; the audit traverses 1,810 project declarations
Boundary: Standard axioms propext, Quot.sound, Classical.choice

**351 original vectors**
What it supports: Contract examples, boundaries, malformed inputs, streaming and buffer behavior
Boundary: Finite fixtures

**200,000 seeded ASan fuzz executions**
What it supports: Additional exercised behavior under instrumented execution
Boundary: Finite campaign

**Host and WASI execution; RISC-V compilation**
What it supports: Practical portability checks, including 32-bit targets
Boundary: Not a compiler-correctness proof

**Archive rebuilds and checksums**
What it supports: Released artifacts can be tied to verified source and evidence
Boundary: A changed release candidate requires a new strict gate


Source refinement does not prove the Rust compiler, machine code, CPU, or operating system correct. The separately invoked proof replay uses the same pinned Lean kernel; it is not an independent second kernel. Users of the Rust crate need Rust 1.88 or later and do not need Lean installed.

## Making a useful fact visible to the compiler

The encoder must accept arbitrary caller slices. Yet no `u64` varint needs more than ten bytes. The optimized entry point makes that bound explicit:

```rust
if out.len() >= 10 {
    encode_u64_into_checked(value, &mut out[..10])
} else {
    encode_u64_into_checked(value, out)
}
```

The checked emitter still computes the required length and checks capacity before writing. For a sufficiently large buffer, it sees a ten-byte slice. Smaller buffers keep the original checked path. The Lean correspondence proof reconstructs the whole caller buffer, including preservation of the suffix outside the ten-byte view.

This is a small source change with a distribution-dependent result. The benchmark passes an opaque caller slice to both implementations, so the comparison does not rely on one caller revealing its array size while the other hides it.

## The measurements, including the inconvenient rows

The following results were recorded on **12 October 2026**, on **Apple M5**, using **Rust 1.88.0**, one thread, optimization level 3, overflow checks enabled, and no LTO. Fifteen repetitions rotate participant order. Before timing, 9,306,112 checks compare canonical bytes, decoded values, and consumption. The previous implementation is the frozen source snapshot `0fdaadf304e6e53ad38942b2ca820c14b8ab771d`.

The values below are mean thread CPU nanoseconds per value, with approximate 95% half-widths. Lower is faster. `short-80-15-5` mixes one-byte, two-byte, and full-width values in those proportions. Full-width uses uniformly generated `u64` values; the large dataset exercises a larger traversal with uncontrolled cache state.

```text
ENCODE: Apple M5, mean CPU ns/value (lower is faster)
Full-width:
bited     ###                 2.657
previous  ######              5.593
leb128    ################## 18.233
int-enc   #####               5.320

One-byte: bited 1.086 / previous 1.059
Large:    bited 3.754 / previous 6.281
```

Peers: leb128 0.2.5; integer-encoding 4.1.0.

```text
Dataset           bited  Previous  leb128  int-enc
----------------  -----  --------  ------  -------
One-byte          1.086  1.059     1.790   1.092
95% half-width    0.010  0.018     0.102   0.011
Short 80/15/5     1.326  1.304     2.918   1.387
95% half-width    0.056  0.030     0.035   0.052
Full-width        2.657  5.593     18.233  5.320
95% half-width    0.041  0.049     0.155   0.055
Large full-width  3.754  6.281     19.049  6.036
95% half-width    0.099  0.190     0.570   0.162
```


Paired repetitions give a **52.5% time reduction** for full-width encoding (approximately 95% interval: 51.9-53.1%) and **40.2%** for the large traversal (38.7-41.6%). One-byte encoding is **2.7% slower** (0.6-4.8%). The short mixture is 1.8% slower on the point estimate, but its interval crosses zero.

Decoding deserves its own table. The capacity change is an encoder optimization; decoder timing differences are not attributed to it.

Peers: leb128 0.2.5; integer-encoding 4.1.0.

```text
Dataset           bited  Previous  leb128  int-enc
----------------  -----  --------  ------  -------
One-byte          1.041  4.847     5.570   1.116
95% half-width    0.041  0.029     0.048   0.100
Short 80/15/5     5.011  5.010     5.667   5.057
95% half-width    0.068  0.048     0.058   0.063
Full-width        8.159  8.208     7.365   8.246
95% half-width    0.065  0.066     0.076   0.063
Large full-width  8.571  8.537     8.482   8.573
95% half-width    0.140  0.107     0.110   0.109
```


`leb128` is faster at full-width decoding in this experiment. The peer APIs also differ in malformed-input behavior: both accept redundant zero in the recorded observations, while bited exposes an explicit canonical policy. Valid-input timings compare speed, not identical feature sets or a universal quality ranking.

The shared host introduces frequency, migration, and cache uncertainty even though thread CPU time excludes descheduling. These results do not establish browser/WASM speed, other-CPU rankings, native `u32` or signed-wrapper performance, PMU counters, or cycles per value. Twelve stack profiles help locate hot functions; they do not remove those limits.

## Weight the workload, not the percentages

A 52.5% improvement on long values and a 2.7% regression on short values cannot be averaged as percentages. Weight the time per operation instead.

Let `p` be the fraction of one-byte values, and let the remaining values follow the measured full-width workload:

```text
T_previous(p) = p * 1.059 + (1 - p) * 5.593
T_bited(p)    = p * 1.086 + (1 - p) * 2.657
```

Using the unrounded recorded means, the crossover is approximately **99.08% one-byte values**: roughly **108 short values per full-width value**. Below that fraction, the capacity change improves the model's mean encoding time. This is a two-population estimate, not a threshold for every possible mixture of two- through ten-byte values.

```text
One-byte share | Previous | bited (modeled CPU ns)
0%             | 5.593    | 2.657
80%            | 1.966    | 1.401
95%            | 1.286    | 1.165
99.08%         | ~1.10    | ~1.10 (crossover)
100%           | 1.059    | 1.086
Remaining inputs follow the full-width distribution.
```

Uniformly random `u64` values have one-byte probability `128 / 2^64`, approximately `6.94 * 10^-18`. They overwhelmingly favor the long-value path. Counters, lengths, and delta-coded identifiers can have very different distributions; measure the actual encoded lengths before applying the result.

The practical hardware ceiling also depends on workload. An encoder must write the encoded bytes and read its input. Memory traffic bounds sustained batch throughput, while branches, instruction dependencies, and stores matter for hot scalar calls. These experiments do not determine a universal theoretical maximum.

## Where bited fits

It fits inside binary protocol implementations, append-only logs, storage formats, delta-coded integer streams, embedded firmware, and WebAssembly applications that need explicit bounded behavior. ZigZag is useful when small negative and positive deltas are common. A varint codec alone does not provide message framing, compression, integrity, or application-level size limits.

The smallest base-128 representation is the encoder's proved format-level optimum. Whether that format is best for a particular dataset remains an application decision: mostly full-width random numbers consume more bytes than a fixed eight-byte integer.

## Status and reproducibility

Version 0.1.1 is published on [crates.io](https://crates.io/crates/bited/0.1.1), with a public [GitHub release](https://github.com/dmytro-yemelianov/bited/releases/tag/v0.1.1). Install it with `cargo add bited@0.1.1`. It adds an inline one- and two-byte u64 decoder fast path that Lean proves equal to the verified scan; on the recorded Apple M5 comparison against 0.1.0 source, short-weighted decoding uses about 81% less CPU time per value and full-width decoding about 17% less, with every dataset and interval in the [decoder comparison](https://github.com/dmytro-yemelianov/bited/blob/main/verification/performance/RESULTS.md). The measurements in this article describe 0.1.0. The strict Rust/Lean release gate passed on source commit `b46480e1e2cf400a40276ffabfffae44cf7df87c`; the release assets bind that exact commit, both packages and their evidence through recorded SHA-256 digests. Later documentation changes do not alter the frozen release evidence.

Portable Rust checks now pass on the repository-scoped `bited-ci-x64` runner on `raps-ci`. Full formal release verification remains a separate pinned local workflow.

Readers can inspect the specification, conformance vectors, proof status, trust boundary, and raw benchmark results. Run `scripts/compare-performance` for the comparison and `scripts/verify` for the strict gate using the documented toolchain. Charts are regenerated from the recorded JSON by `scripts/article-charts.py`; raw timing metadata records the source hashes and the dirty working tree used at measurement time.

The public article also appears as a project detail inside the site's WASM portfolio.
