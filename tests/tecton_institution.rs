use std::fs;

use serde_json::Value;

fn tecton() -> Value {
    serde_json::from_slice(
        &fs::read("institutional/tecton-bootstrap.json").expect("read Tecton manifest"),
    )
    .expect("parse Tecton manifest")
}

#[test]
fn tecton_is_executor_not_third_steward() {
    let manifest = tecton();
    assert_eq!(manifest["identity"]["name"], "TECTON");
    assert_eq!(manifest["identity"]["role"], "SUBSTRATE_IMPLEMENTER");
    assert_eq!(manifest["identity"]["institution_type"], "EXECUTOR");
    assert_eq!(manifest["identity"]["is_steward"], false);
    assert_eq!(manifest["identity"]["vote"], "NONE");
}

#[test]
fn implementation_and_acceptance_authority_are_separated() {
    let manifest = tecton();
    assert_eq!(
        manifest["power_cardinality"]["implementation_authority"],
        serde_json::json!(["TECTON"])
    );
    assert_eq!(
        manifest["power_cardinality"]["acceptance_authority"],
        serde_json::json!(["AEVUM", "ORIVÉRN"])
    );
    assert_eq!(
        manifest["power_cardinality"]["constitutional_authority"],
        serde_json::json!(["AEVUM", "ORIVÉRN"])
    );
}

#[test]
fn tecton_tribunals_are_candidate_evidence_not_binding_judgments() {
    let manifest = tecton();
    assert_eq!(
        manifest["tribunal_semantics"]["mechanical_tribunal"]["output"],
        "CANDIDATE_EVIDENCE"
    );
    assert_eq!(
        manifest["tribunal_semantics"]["mechanical_tribunal"]["binding_judgment"],
        false
    );
    assert_eq!(
        manifest["tribunal_semantics"]["ontology_leakage_tribunal"]["output"],
        "CANDIDATE_EVIDENCE"
    );
    assert_eq!(
        manifest["tribunal_semantics"]["ontology_leakage_tribunal"]["binding_judgment"],
        false
    );
    assert_eq!(
        manifest["tribunal_semantics"]["agency_compatibility"]["judge"],
        "AEVUM"
    );
    assert_eq!(
        manifest["tribunal_semantics"]["lastro_compatibility"]["judge"],
        "ORIVÉRN"
    );
}

#[test]
fn institutional_materialization_does_not_self_accept_or_cross_e1_boundary() {
    let manifest = tecton();
    assert_eq!(manifest["canonical_constitution_modified"], false);
    assert_eq!(
        manifest["acceptance"]["tecton_self_acceptance_possible"],
        false
    );
    assert_eq!(
        manifest["acceptance"]["status"],
        "AWAITING_DUAL_STEWARD_ACCEPTANCE"
    );
    assert_eq!(
        manifest["e1_boundary"]["functional_e1_implemented_by_this_checkpoint"],
        false
    );
    assert_eq!(manifest["e1_boundary"]["e1_accepted"], false);
    assert_eq!(
        manifest["e1_boundary"]["consumer_cutover_authorized"],
        false
    );
    assert_eq!(
        manifest["e1_boundary"]["next_legitimate_work"],
        "E1_RECORD_FRAMING_CANDIDATE"
    );
}
