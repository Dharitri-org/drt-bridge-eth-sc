#[test]
fn batch_transfer_both_executed_go() {
    dharitri_sc_scenario::run_go("denali/batch_transfer_both_executed.scen.json");
}

#[test]
fn batch_transfer_both_failed_go() {
    dharitri_sc_scenario::run_go("denali/batch_transfer_both_failed.scen.json");
}

#[test]
fn batch_transfer_one_executed_one_failed_go() {
    dharitri_sc_scenario::run_go("denali/batch_transfer_one_executed_one_failed.scen.json");
}

#[test]
fn batch_transfer_to_frozen_account_go() {
    dharitri_sc_scenario::run_go("denali/batch_transfer_to_frozen_account.scen.json");
}

#[test]
fn setup_accounts_go() {
    dharitri_sc_scenario::run_go("denali/setup_accounts.scen.json");
}

#[test]
fn transfer_ok_go() {
    dharitri_sc_scenario::run_go("denali/transfer_ok.scen.json");
}

#[test]
fn two_transfers_same_token_go() {
    dharitri_sc_scenario::run_go("denali/two_transfers_same_token.scen.json");
}
