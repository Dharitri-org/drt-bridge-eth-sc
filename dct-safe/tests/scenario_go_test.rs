#[test]
fn claim_fees_go() {
    dharitri_sc_scenario::run_go("denali/distribute_fees.scen.json");
}

#[test]
fn create_another_tx_ok_go() {
    dharitri_sc_scenario::run_go("denali/create_another_tx_ok.scen.json");
}

#[test]
fn create_another_tx_too_late_for_batch_go() {
    dharitri_sc_scenario::run_go("denali/create_another_tx_too_late_for_batch.scen.json");
}

#[test]
fn create_transaction_ok_go() {
    dharitri_sc_scenario::run_go("denali/create_transaction_ok.scen.json");
}

#[test]
fn execute_batch_both_rejected_go() {
    dharitri_sc_scenario::run_go("denali/execute_batch_both_rejected.scen.json");
}

#[test]
fn execute_batch_both_success_go() {
    dharitri_sc_scenario::run_go("denali/execute_batch_both_success.scen.json");
}

#[test]
fn execute_batch_one_success_one_rejected_go() {
    dharitri_sc_scenario::run_go("denali/execute_batch_one_success_one_rejected.scen.json");
}

#[test]
fn execute_transaction_rejected_go() {
    dharitri_sc_scenario::run_go("denali/execute_transaction_rejected.scen.json");
}

#[test]
fn execute_transaction_success_go() {
    dharitri_sc_scenario::run_go("denali/execute_transaction_success.scen.json");
}

#[test]
fn get_next_pending_tx_go() {
    dharitri_sc_scenario::run_go("denali/get_next_pending_tx.scen.json");
}

#[test]
fn get_next_tx_batch_go() {
    dharitri_sc_scenario::run_go("denali/get_next_tx_batch.scen.json");
}

#[test]
fn get_next_tx_batch_too_early_go() {
    dharitri_sc_scenario::run_go("denali/get_next_tx_batch_too_early.scen.json");
}

#[test]
fn setup_accounts_go() {
    dharitri_sc_scenario::run_go("denali/setup_accounts.scen.json");
}

#[test]
fn zero_fees_go() {
    dharitri_sc_scenario::run_go("denali/zero_fees.scen.json");
}
