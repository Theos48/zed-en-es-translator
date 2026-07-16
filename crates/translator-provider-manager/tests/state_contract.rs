use translator_provider_manager::state::InstallationState;

const PROFILE: &str = "bergamot-en-es-linux-x86_64-v1";
const A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

#[test]
fn state_should_promote_candidate_without_losing_previous_current() {
    let mut state = InstallationState::empty(PROFILE);
    state.stage_candidate(A).expect("candidate A should stage");
    state
        .promote_candidate()
        .expect("candidate A should promote");
    state.stage_candidate(B).expect("candidate B should stage");
    state
        .promote_candidate()
        .expect("candidate B should promote");

    assert_eq!(state.references(), (Some(B), Some(A), None));
}

#[test]
fn state_should_rollback_only_to_a_previous_verified_reference() {
    let mut state = InstallationState::empty(PROFILE);
    state.stage_candidate(A).expect("candidate A should stage");
    state
        .promote_candidate()
        .expect("candidate A should promote");
    state.stage_candidate(B).expect("candidate B should stage");
    state
        .promote_candidate()
        .expect("candidate B should promote");
    state.rollback().expect("previous A should rollback");

    assert_eq!(state.references(), (Some(A), Some(B), None));
}

#[test]
fn state_should_leave_current_unchanged_when_candidate_is_rejected() {
    let mut state = InstallationState::empty(PROFILE);
    state.stage_candidate(A).expect("candidate A should stage");
    state
        .promote_candidate()
        .expect("candidate A should promote");
    state.stage_candidate(B).expect("candidate B should stage");
    state.reject_candidate().expect("candidate B should reject");

    assert_eq!(state.references(), (Some(A), None, None));
}

#[test]
fn state_should_reject_unknown_schema_versions() {
    let json = format!(
        r#"{{"schema_version":2,"generation":0,"profile_id":"{PROFILE}","current":null,"previous":null,"candidate":null,"last_operation":"none","last_outcome":"ready"}}"#
    );

    let error = InstallationState::from_json(&json).expect_err("schema 2 must fail");

    assert_eq!(error.code(), "STATE_INVALID");
}
