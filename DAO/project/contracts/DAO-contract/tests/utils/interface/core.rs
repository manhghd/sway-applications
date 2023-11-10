use crate::utils::setup::{DaoVoting, Proposal};
use fuels::{
    prelude::{CallParameters, TxParameters, WalletUnlocked},
    programs::{call_response::FuelCallResponse, call_utils::TxDependencyExtension}, types::AssetId,
};

pub(crate) async fn constructor(
    contract: &DaoVoting<WalletUnlocked>,
    token: AssetId,
) -> FuelCallResponse<()> {
    contract.methods().constructor(token).call().await.unwrap()
}

pub(crate) async fn create_proposal(
    contract: &DaoVoting<WalletUnlocked>,
    acceptance_percentage: u64,
    deadline: u64,
    proposal: Proposal,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .create_proposal(acceptance_percentage, deadline, proposal)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn deposit(
    contract: &DaoVoting<WalletUnlocked>,
    call_params: CallParameters,
) -> FuelCallResponse<()> {
    let tx_params = TxParameters::new(Some(0), Some(1_000_000), 0);
    contract
        .methods()
        .deposit()
        .tx_params(tx_params)
        .call_params(call_params)
        .unwrap()
        .call()
        .await
        .unwrap()
}

pub(crate) async fn withdraw(
    contract: &DaoVoting<WalletUnlocked>,
    amount: u64,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .withdraw(amount)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn vote(
    contract: &DaoVoting<WalletUnlocked>,
    approve: bool,
    proposal_id: u64,
    vote_amount: u64,
) -> FuelCallResponse<()> {
    contract
        .methods()
        .vote(approve, proposal_id, vote_amount)
        .call()
        .await
        .unwrap()
}

pub(crate) async fn execute(contract: &DaoVoting<WalletUnlocked>, id: u64) -> FuelCallResponse<()> {
    contract.methods().execute(id).call().await.unwrap()
}

pub(crate) async fn unlock_votes(
    contract: &DaoVoting<WalletUnlocked>,
    id: u64,
) -> FuelCallResponse<()> {
    contract.methods().unlock_votes(id).call().await.unwrap()
}
