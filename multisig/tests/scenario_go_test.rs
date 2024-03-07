#[test]
fn create_dharitri_to_ethereum_tx_batch_go() {
    dharitri_sc_scenario::run_go("denali/create_dharitri_to_ethereum_tx_batch.scen.json");
}

#[test]
fn ethereum_to_dharitri_tx_batch_ok_go() {
    dharitri_sc_scenario::run_go("denali/ethereum_to_dharitri_tx_batch_ok.scen.json");
}

#[test]
fn ethereum_to_dharitri_tx_batch_rejected_go() {
    dharitri_sc_scenario::run_go("denali/ethereum_to_dharitri_tx_batch_rejected.scen.json");
}

#[test]
fn execute_dharitri_to_ethereum_tx_batch_go() {
    dharitri_sc_scenario::run_go("denali/execute_dharitri_to_ethereum_tx_batch.scen.json");
}

#[test]
fn get_empty_batch_go() {
    dharitri_sc_scenario::run_go("denali/get_empty_batch.scen.json");
}

#[test]
fn reject_dharitri_to_ethereum_tx_batch_go() {
    dharitri_sc_scenario::run_go("denali/reject_dharitri_to_ethereum_tx_batch.scen.json");
}

#[test]
fn setup_go() {
    dharitri_sc_scenario::run_go("denali/setup.scen.json");
}

#[test]
fn unstake_go() {
    dharitri_sc_scenario::run_go("denali/unstake.scen.json");
}

/*
#[test]
fn upgrade_child_sc_go() {
    dharitri_sc_scenario::run_go("denali/upgrade_child_sc.scen.json");
}
*/
