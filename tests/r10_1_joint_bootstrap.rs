use std::fs;

use serde_json::Value;

#[test]
fn r10_1_joint_accepts_bootstrap_without_accepting_e1_or_cutover() {
    let receipt: Value = serde_json::from_slice(
        &fs::read("constitutional/r10-1-joint-bootstrap-acceptance.json")
            .expect("read R10.1 joint receipt"),
    )
    .expect("parse R10.1 joint receipt");

    assert_eq!(receipt["joint_r10"]["authorization"], "AUTHORIZED");
    assert_eq!(
        receipt["substrate"]["bootstrap_commit"],
        "a455320bbd5adc3478a5198e394d6f67ce679b5b"
    );
    assert_eq!(
        receipt["substrate"]["bootstrap_manifest_git_blob_sha1"],
        "7121f9f27fa0f4fb7dd6070ba1e0791e074fe052"
    );
    assert_eq!(receipt["orivern_review"]["ci_status"], "SUCCESS");
    assert_eq!(receipt["aevum_review"]["linux_status"], "SUCCESS");
    assert_eq!(receipt["aevum_review"]["windows_status"], "SUCCESS");
    assert_eq!(receipt["r10_1_orivern"], "PASS");
    assert_eq!(receipt["r10_1_aevum"], "PASS");
    assert_eq!(receipt["r10_1_joint"], "CLOSED_JOINT_PASS");
    assert_eq!(receipt["constitutional_bootstrap_accepted"], true);
    assert_eq!(
        receipt["boundary"]["extracted_mechanisms"]
            .as_array()
            .unwrap()
            .len(),
        0
    );
    assert_eq!(receipt["boundary"]["e1_implemented"], false);
    assert_eq!(receipt["boundary"]["consumer_cutover_authorized"], false);
    assert_eq!(receipt["shared_extraction_authorized"], true);
    assert_eq!(receipt["next_checkpoint"], "E1_RECORD_FRAMING_CANDIDATE");
    assert_eq!(receipt["e1_implementation_authorized"], true);
    assert_eq!(receipt["e1_consumer_cutover_authorized"], false);
}
