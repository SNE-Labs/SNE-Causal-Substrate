# E1 CONTRACT — Record Framing Candidate

Status: `TECTON_CANDIDATE`

This contract defines the minimum byte-level representation of one record frame.
It is fully explainable with `Consumer A` and `Consumer B`; neither consumer's
payload ontology is part of the mechanism.

## Question

What is the minimum mechanical representation that lets two independent
consumers place arbitrary bytes inside a deterministic, versioned record
boundary and reject malformed or partial frames without interpreting those
bytes?

## Canonical byte layout

All integer fields use unsigned big-endian encoding.

```text
offset   size   field
0        8      format discriminator = b"SNECFRM\\0"
8        2      protocol version = 1 (u16)
10       8      structural ordinal (u64)
18       8      payload length N (u64)
26       N      opaque payload bytes
```

Fixed header length: `26` bytes.

## Field justification

`format discriminator`
: Distinguishes this frame family from unrelated or misaligned bytes before any
  variable-length material is accepted.

`protocol version`
: Makes the on-disk protocol explicit in every frame and creates a fail-closed
  boundary for future incompatible layouts.

`structural ordinal`
: Preserves the mechanically shared ordinal carried by proven consumer record
  forms. E1 treats it only as an unsigned field. E1 does not require zero-based,
  monotonic, contiguous, admission-ordered, or otherwise related ordinals.

`payload length`
: Makes the frame boundary decidable without delimiter scanning, text decoding,
  or payload inspection. Embedded LF, CRLF, NUL, delimiter-like bytes, and
  non-UTF8 bytes therefore remain ordinary payload material.

`opaque payload`
: Exact consumer-provided bytes. E1 does not decode, normalize, validate, or
  classify them.

## Canonical encoding invariants

1. The format discriminator is byte-exact.
2. The version is exactly `1`.
3. The ordinal is encoded as exactly eight big-endian bytes.
4. The payload length is encoded as exactly eight big-endian bytes.
5. The accepted frame length is exactly `26 + payload_length`.
6. No trailing bytes are accepted after the declared payload.
7. No truncated header or truncated payload is accepted.
8. Accepted payload bytes round-trip byte-identically.
9. Encoding the same ordinal and payload produces the same bytes on every run.
10. Re-encoding any accepted canonical frame reproduces the exact accepted
    bytes.

## Strict decoding

A decoder rejects at least these malformed classes:

- frame shorter than the fixed header;
- format discriminator mismatch;
- unsupported protocol version;
- payload-length conversion or arithmetic overflow;
- declared payload longer than available bytes;
- bytes remaining after the declared payload.

No best-effort prefix interpretation is permitted for a malformed single frame.

## E1 / E2 boundary

E1 contains no digest field and no predecessor field.

Those fields recur in previously proven physical implementations, but the
constitutional extraction sequence assigns digest and predecessor chaining to
E2. Freezing a digest algorithm, digest width, predecessor representation, or
chain-validation policy in E1 is therefore unnecessary to record framing and
would collapse two extraction steps.

E2 may define a later protocol version or another constitutionally accepted
extension. E1 makes no decision for E2.

## Explicit non-claims

This contract does **not** establish:

- payload integrity by cryptographic digest;
- predecessor or head chaining;
- ordinal continuity across frames;
- strict replay of a history;
- append or expected-head behavior;
- writer exclusion or locking;
- staging, rename, synchronization, or durability semantics;
- snapshot binding;
- payload meaning, truth, ontology, or policy;
- consumer cutover or history migration;
- package release;
- institutional acceptance of E1.

Tecton's implementation and tribunals under this contract can produce only
`CANDIDATE_EVIDENCE` and at most a `PASS_CANDIDATE` verdict.
