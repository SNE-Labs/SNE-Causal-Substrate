use std::fs;

#[test]
fn e1_core_has_no_consumer_ontology_dependencies() {
    let source = fs::read_to_string("src/frame.rs").expect("read E1 core source");
    let lower = source.to_ascii_lowercase();
    let forbidden = [
        "worldobject",
        "evidence",
        "projection",
        "opportunity",
        "quest",
        "expedition",
        "authority",
        "admission",
        "economic",
        "cognition",
        "scheduler",
        "commitment",
        "agency",
        "lastro",
    ];

    for term in forbidden {
        assert!(
            !lower.contains(term),
            "E1 core contains forbidden semantic dependency: {term}"
        );
    }
}

#[test]
fn e1_core_does_not_smuggle_e2_digest_or_predecessor_mechanics() {
    let source = fs::read_to_string("src/frame.rs").expect("read E1 core source");
    let lower = source.to_ascii_lowercase();
    assert!(!lower.contains("digest"));
    assert!(!lower.contains("predecessor"));
    assert!(!lower.contains("sha256"));
}

#[test]
fn consumer_ab_substitution_witness_is_name_independent() {
    let witness =
        fs::read_to_string("e1/CONSUMER-AB-WITNESS.md").expect("read substitution witness");
    assert!(witness.contains("Consumer A"));
    assert!(witness.contains("Consumer B"));
    assert!(!witness.contains("Agency"));
    assert!(!witness.contains("Lastro"));
}

#[test]
fn contract_preserves_candidate_only_acceptance_boundary() {
    let contract = fs::read_to_string("e1/CONTRACT.md").expect("read E1 contract");
    assert!(contract.contains("TECTON_CANDIDATE"));
    assert!(contract.contains("CANDIDATE_EVIDENCE"));
    assert!(contract.contains("PASS_CANDIDATE"));
    assert!(contract.contains("institutional acceptance of E1"));
}
