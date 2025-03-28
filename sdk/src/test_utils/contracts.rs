use {
    crate::sui::{self, traits::*},
    std::path::PathBuf,
};

/// Publishes a Move package to Sui.
///
/// `path_str` is the path relative to the project `Cargo.toml` directory.
pub async fn publish_move_package(
    wallet: &mut sui::WalletContext,
    path_str: &str,
    gas_coin: sui::Coin,
) -> sui::TransactionBlockResponse {
    let install_dir = PathBuf::from(path_str);
    let lock_file = PathBuf::from(format!("{path_str}/Move.lock"));

    let sui = wallet
        .get_client()
        .await
        .expect("Failed to get Sui client.");

    let addr = wallet
        .active_address()
        .expect("Failed to get active address.");

    let chain_id = sui
        .read_api()
        .get_chain_identifier()
        .await
        .expect("Failed to get chain identifier.");

    // Compile the package.
    let mut build_config = sui_move_build::BuildConfig::new_for_testing();
    build_config.chain_id = Some(chain_id);
    let package = build_config
        .build(&install_dir)
        .expect("Failed to build package.");

    let reference_gas_price = sui
        .read_api()
        .get_reference_gas_price()
        .await
        .expect("Failed to fetch reference gas price.");

    let with_unpublished_deps = false;

    let tx = sui
        .transaction_builder()
        .publish_tx_kind(
            addr,
            package.get_package_bytes(with_unpublished_deps),
            package.get_dependency_storage_package_ids(),
        )
        .await
        .expect("Failed to build transaction.");

    let tx_data = sui
        .transaction_builder()
        .tx_data(
            addr,
            tx,
            sui::MIST_PER_SUI,
            reference_gas_price,
            vec![gas_coin.coin_object_id],
            None,
        )
        .await
        .expect("Failed to build transaction data.");

    // Prepare some options for the transaction. Object changes and events are
    // used to parse useful IDs from.
    let envelope = wallet.sign_transaction(&tx_data);
    let resp_options = sui::TransactionBlockResponseOptions::new()
        .with_events()
        .with_effects()
        .with_object_changes();
    let resp_finality = sui::ExecuteTransactionRequestType::WaitForLocalExecution;

    // Execute the transaction.
    let response = sui
        .quorum_driver_api()
        .execute_transaction_block(envelope, resp_options, Some(resp_finality))
        .await
        .expect("Failed to execute transaction.");

    if let Some(effects) = response.effects.clone() {
        if effects.clone().into_status().is_err() {
            panic!("Transaction has erroneous effects: {path_str} {effects}");
        }
    }

    sui_package_management::update_lock_file(
        wallet,
        sui_package_management::LockCommand::Publish,
        Some(install_dir),
        Some(lock_file),
        &response,
    )
    .await
    .expect("Failed to update lock file.");

    response
}
