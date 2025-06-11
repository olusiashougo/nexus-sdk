use {
    crate::{command_title, display::json_output, loading, prelude::*, sui::*},
    nexus_sdk::transactions::gas,
};

/// Upload `coin` as a gas budget for the Nexus workflow.
pub(crate) async fn add_gas_budget(
    coin: sui::ObjectID,
    sui_gas_coin: Option<sui::ObjectID>,
    sui_gas_budget: u64,
) -> AnyResult<(), NexusCliError> {
    command_title!("Adding '{coin}' as gas budget for Nexus");

    // Load CLI configuration.
    let mut conf = CliConf::load().await.unwrap_or_default();

    // Nexus objects must be present in the configuration.
    let objects = &get_nexus_objects(&mut conf).await?;

    // Create wallet context, Sui client and find the active address.
    let mut wallet = create_wallet_context(&conf.sui.wallet_path, conf.sui.net).await?;
    let sui = build_sui_client(&conf.sui).await?;
    let address = wallet.active_address().map_err(NexusCliError::Any)?;

    // Fetch gas coin object.
    let gas_coin = fetch_gas_coin(&sui, address, sui_gas_coin).await?;

    // Fetch budget coin.
    let budget_coin = fetch_object_by_id(&sui, coin).await?;

    if budget_coin.object_id == gas_coin.coin_object_id {
        return Err(NexusCliError::Any(anyhow!(
            "Gas and budget coins must be different."
        )));
    }

    // Fetch reference gas price.
    let reference_gas_price = fetch_reference_gas_price(&sui).await?;

    // Craft the transaction.
    let tx_handle = loading!("Crafting transaction...");

    let mut tx = sui::ProgrammableTransactionBuilder::new();

    if let Err(e) = gas::add_budget(&mut tx, objects, address.into(), &budget_coin) {
        tx_handle.error();

        return Err(NexusCliError::Any(e));
    }

    tx_handle.success();

    let tx_data = sui::TransactionData::new_programmable(
        address,
        vec![gas_coin.object_ref()],
        tx.finish(),
        sui_gas_budget,
        reference_gas_price,
    );

    // Sign and send the TX.
    let response = sign_and_execute_transaction(&sui, &wallet, tx_data).await?;

    json_output(&json!({ "digest": response.digest }))?;

    Ok(())
}
