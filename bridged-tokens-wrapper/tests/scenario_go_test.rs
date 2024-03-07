#[test]
fn unwrap_token_go() {
    dharitri_sc_scenario::run_go("denali/unwrap_token.scen.json");
}

#[test]
fn wrap_token_go() {
    dharitri_sc_scenario::run_go("denali/wrap_token.scen.json");
}

#[test]
fn whitelist_token_go() {
    dharitri_sc_scenario::run_go("denali/whitelist_token.scen.json");
}

#[test]
fn blacklist_token_go() {
    dharitri_sc_scenario::run_go("denali/blacklist_token.scen.json");
}

#[test]
fn add_wrapped_token_go() {
    dharitri_sc_scenario::run_go("denali/add_wrapped_token.scen.json");
}

#[test]
fn remove_wrapped_token_go() {
    dharitri_sc_scenario::run_go("denali/remove_wrapped_token.scen.json");
}
