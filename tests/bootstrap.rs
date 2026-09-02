use std::fs;

use serde_json::Value;

fn bootstrap() -> Value {
    serde_json::from_slice(
        &fs::read("constitutional/bootstrap.json").expect("read bootstrap manifest"),
    )
    .expect("parse bootstrap manifest")
}

#[test]
fn bootstrap_is_bound_to_the_joint_r9_and_r10_receipts() {
    let manifest = bootstrap();
    assert_eq!(
        manifest["pins"]["r9_constitution"]["git_blob_sha1"],
        "0ed951cbdc6245ea134b56acf34ec5a328157636"
    );
    assert_eq!(
        manifest["pins"]["r10_joint_lastro"]["git_blob_sha1"],
        "96e61f7e48150467fcd12bc4916e74e28e534a57"
    );
    assert_eq!(
        manifest["pins"]["r10_joint_agency"]["git_blob_sha1"],
        manifest["pins"]["r10_joint_lastro"]["git_blob_sha1"]
    );
}

#[test]
fn creation_and_extraction_are_authorized_but_cutover_is_not() {
    let manifest = bootstrap();
    assert_eq!(
        manifest["joint_state"]["all_seven_shared_gates_closed"],
        true
    );
    assert_eq!(manifest["joint_state"]["joint_r10"], "AUTHORIZED");
    assert_eq!(
        manifest["joint_state"]["repository_creation_authorized"],
        true
    );
    assert_eq!(
        manifest["joint_state"]["shared_extraction_authorized"],
        true
    );
    assert_eq!(
        manifest["joint_state"]["consumer_cutover_authorized"],
        false
    );
}

#[test]
fn scaffold_contains_no_extracted_mechanism_or_release_claim() {
    let manifest = bootstrap();
    assert_eq!(
        manifest["repository_state"]["extracted_mechanisms"]
            .as_array()
            .unwrap()
            .len(),
        0
    );
    assert_eq!(
        manifest["repository_state"]["next_checkpoint"],
        "E1_RECORD_FRAMING_CANDIDATE"
    );
    assert_eq!(
        manifest["governance"]["unilateral_change_or_release"],
        false
    );
    assert_eq!(manifest["governance"]["import_grants_authority"], false);
    assert_eq!(manifest["governance"]["first_release_authorized"], false);
}

#[test]
fn package_import_exposes_no_authority() {
    assert!(!std::hint::black_box(
        sne_causal_substrate::PACKAGE_IMPORT_GRANTS_AUTHORITY
    ));
    assert_eq!(
        sne_causal_substrate::NEXT_EXTRACTION_STEP,
        "E1_RECORD_FRAMING"
    );
}
