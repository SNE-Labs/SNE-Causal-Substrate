use std::fs;

use serde_json::Value;

fn decode_hex(value: &str) -> Vec<u8> {
    assert_eq!(value.len() % 2, 0, "hex fixture must have even length");
    value
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| {
            let high = hex_nibble(pair[0]);
            let low = hex_nibble(pair[1]);
            (high << 4) | low
        })
        .collect()
}

fn hex_nibble(value: u8) -> u8 {
    match value {
        b'0'..=b'9' => value - b'0',
        b'a'..=b'f' => value - b'a' + 10,
        b'A'..=b'F' => value - b'A' + 10,
        _ => panic!("invalid hex digit in pinned fixture"),
    }
}

#[test]
fn origin_fixture_bytes_are_preserved_exactly() {
    let manifest: Value = serde_json::from_slice(
        &fs::read("fixtures/e1/manifest.json").expect("read E1 fixture manifest"),
    )
    .expect("parse E1 fixture manifest");

    let origin = &manifest["consumer_b_opaque_payload_origin"];
    assert_eq!(origin["source_repository"], "SNE-Labs/Lastro");
    assert_eq!(
        origin["source_commit"],
        "7f08a980c0387836731aa9842ecb517a5d6db773"
    );
    assert_eq!(
        origin["source_path"],
        "compatibility/journal-process-crash-v1/manifest.json"
    );
    assert_eq!(
        origin["source_git_blob"],
        "63d4a816bf3a7c36ff05f28a362898a684ea0c74"
    );

    let payloads = origin["payloads"]
        .as_array()
        .expect("payload fixture array");
    assert_eq!(payloads.len(), 3);

    for payload in payloads {
        assert_eq!(payload["copied_byte_exact"], true);
        let path = payload["substrate_path"].as_str().expect("fixture path");
        let expected = decode_hex(payload["bytes_hex"].as_str().expect("fixture bytes hex"));
        let actual = fs::read(path).expect("read byte-exact fixture");
        assert_eq!(actual, expected, "origin bytes changed for {path}");
    }

    assert_eq!(manifest["origin_fixture_bytes_may_be_rewritten"], false);
    assert_eq!(manifest["consumer_history_migration_performed"], false);
    assert_eq!(manifest["consumer_cutover_authorized"], false);
}

#[test]
fn proven_sources_are_pinned_by_commit_and_git_blob() {
    let manifest: Value = serde_json::from_slice(
        &fs::read("fixtures/e1/manifest.json").expect("read E1 fixture manifest"),
    )
    .expect("parse E1 fixture manifest");

    let generic = &manifest["consumer_a_generic_framing_witness"];
    assert_eq!(generic["source_repository"], "SNE-Labs/SNE-Agency");
    assert_eq!(
        generic["source_commit"],
        "fe4d1a7ad962ab74ac2950246d7cd57ef3eb050d"
    );
    assert_eq!(
        generic["source_git_blob"],
        "7e8ad8a979f77243d376b8e43a9704ac99b52d4a"
    );

    let writer = &manifest["consumer_b_writer_witness"];
    assert_eq!(writer["source_repository"], "SNE-Labs/Lastro");
    assert_eq!(
        writer["source_git_blob"],
        "4a2bd27c9cf37aeae2b304636a40e96c24eb8dfc"
    );

    let gate = &manifest["joint_gate_1_witness"];
    assert_eq!(
        gate["generic_framing_and_crash_atomicity"],
        "CLOSED_JOINT_PASS"
    );
}
