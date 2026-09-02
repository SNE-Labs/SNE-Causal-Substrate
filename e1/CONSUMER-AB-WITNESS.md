# E1 Consumer A / Consumer B substitution witness

Consumer A produces arbitrary byte payloads.
Consumer B produces arbitrary byte payloads.

Both need the same mechanical operation:

```text
(structural ordinal, opaque bytes)
        ↓
canonical record-frame encoding
        ↓
strict record-frame decoding
        ↓
(structural ordinal, identical opaque bytes)
```

The mechanism needs no knowledge of why Consumer A created its bytes, why
Consumer B created its bytes, whether those bytes are text, or what either
consumer will do after decoding them.

The frame contract answers only:

- which byte family is being decoded;
- which protocol version is present;
- which unsigned structural ordinal is carried;
- exactly how many opaque payload bytes belong to this frame;
- whether the supplied bytes are exactly one canonical frame.

It does not answer:

- whether an ordinal is valid relative to another frame;
- whether one frame follows another;
- whether payload bytes are authentic or truthful;
- whether a frame should enter any consumer history;
- whether a consumer is permitted to write anything.

Therefore replacing either consumer with any other producer of opaque bytes does
not change the core E1 explanation or implementation.
