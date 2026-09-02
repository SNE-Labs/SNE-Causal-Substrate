use sne_causal_substrate::{
    FRAME_HEADER_LEN, FRAME_MAGIC, FRAME_VERSION, FrameError, RecordFrame, decode_frame,
    encode_frame,
};

const JSON_EVENT_BYTES: &[u8] =
    include_bytes!("../fixtures/e1/lastro-7f08a98/json-event-bytes.bin");
const EMBEDDED_DELIMITERS: &[u8] =
    include_bytes!("../fixtures/e1/lastro-7f08a98/embedded-delimiters.bin");
const NON_UTF8_BINARY: &[u8] = include_bytes!("../fixtures/e1/lastro-7f08a98/non-utf8-binary.bin");

#[test]
fn canonical_layout_is_fixed_and_big_endian() {
    let ordinal = 0x0102_0304_0506_0708_u64;
    let payload = [0xaa, 0xbb, 0xcc];
    let bytes = encode_frame(ordinal, &payload).expect("encode frame");

    assert_eq!(FRAME_HEADER_LEN, 26);
    assert_eq!(bytes.len(), FRAME_HEADER_LEN + payload.len());
    assert_eq!(&bytes[0..8], &FRAME_MAGIC);
    assert_eq!(&bytes[8..10], &FRAME_VERSION.to_be_bytes());
    assert_eq!(&bytes[10..18], &ordinal.to_be_bytes());
    assert_eq!(&bytes[18..26], &(payload.len() as u64).to_be_bytes());
    assert_eq!(&bytes[26..], payload);
}

#[test]
fn opaque_origin_payloads_round_trip_byte_exactly() {
    let cases: &[(u64, &[u8])] = &[
        (0, JSON_EVENT_BYTES),
        (1, EMBEDDED_DELIMITERS),
        (u64::MAX, NON_UTF8_BINARY),
    ];

    for (ordinal, payload) in cases {
        let encoded = encode_frame(*ordinal, payload).expect("encode origin payload");
        let decoded = decode_frame(&encoded).expect("decode origin payload");
        assert_eq!(decoded.ordinal(), *ordinal);
        assert_eq!(decoded.payload(), *payload);
        assert_eq!(decoded.encode().expect("re-encode frame"), encoded);
    }
}

#[test]
fn empty_payload_is_a_valid_frame() {
    let frame = RecordFrame::new(7, Vec::new());
    let bytes = frame.encode().expect("encode empty payload");
    assert_eq!(bytes.len(), FRAME_HEADER_LEN);
    assert_eq!(decode_frame(&bytes).expect("decode empty payload"), frame);
}

#[test]
fn encoding_is_deterministic() {
    let first = encode_frame(42, NON_UTF8_BINARY).expect("first encode");
    let second = encode_frame(42, NON_UTF8_BINARY).expect("second encode");
    assert_eq!(first, second);
}

#[test]
fn every_truncated_header_is_rejected() {
    for actual in 0..FRAME_HEADER_LEN {
        let bytes = vec![0_u8; actual];
        assert_eq!(
            decode_frame(&bytes),
            Err(FrameError::HeaderTruncated {
                minimum: FRAME_HEADER_LEN,
                actual,
            })
        );
    }
}

#[test]
fn format_mismatch_is_rejected() {
    let mut bytes = encode_frame(0, b"payload").expect("encode frame");
    bytes[0] ^= 0xff;
    assert_eq!(decode_frame(&bytes), Err(FrameError::FormatMismatch));
}

#[test]
fn unsupported_version_is_rejected() {
    let mut bytes = encode_frame(0, b"payload").expect("encode frame");
    bytes[8..10].copy_from_slice(&2_u16.to_be_bytes());
    assert_eq!(decode_frame(&bytes), Err(FrameError::UnsupportedVersion(2)));
}

#[test]
fn truncated_payload_is_rejected() {
    let mut bytes = encode_frame(0, b"payload").expect("encode frame");
    bytes.pop();
    assert_eq!(
        decode_frame(&bytes),
        Err(FrameError::LengthMismatch {
            expected: FRAME_HEADER_LEN + b"payload".len(),
            actual: FRAME_HEADER_LEN + b"payload".len() - 1,
        })
    );
}

#[test]
fn trailing_bytes_are_rejected() {
    let mut bytes = encode_frame(0, b"payload").expect("encode frame");
    bytes.push(0);
    assert_eq!(
        decode_frame(&bytes),
        Err(FrameError::LengthMismatch {
            expected: FRAME_HEADER_LEN + b"payload".len(),
            actual: FRAME_HEADER_LEN + b"payload".len() + 1,
        })
    );
}

#[test]
fn impossible_declared_length_is_rejected_without_allocation() {
    let mut bytes = encode_frame(0, &[]).expect("encode frame");
    bytes[18..26].copy_from_slice(&u64::MAX.to_be_bytes());
    assert_eq!(decode_frame(&bytes), Err(FrameError::LengthOverflow));
}

#[test]
fn ordinal_is_structural_not_cross_frame_policy() {
    let first =
        decode_frame(&encode_frame(900, b"a").expect("encode first")).expect("decode first");
    let second =
        decode_frame(&encode_frame(3, b"b").expect("encode second")).expect("decode second");

    assert_eq!(first.ordinal(), 900);
    assert_eq!(second.ordinal(), 3);
}

#[test]
fn payload_mutation_remains_a_well_formed_e1_frame() {
    let mut bytes = encode_frame(5, b"abc").expect("encode frame");
    bytes[FRAME_HEADER_LEN + 1] ^= 0xff;

    let decoded = decode_frame(&bytes).expect("E1 checks framing, not payload integrity");
    assert_eq!(decoded.ordinal(), 5);
    assert_eq!(decoded.payload(), &[b'a', b'b' ^ 0xff, b'c']);
}
