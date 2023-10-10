use fuels::accounts::fuel_crypto::rand::{self, Rng};
use fuels::prelude::*;
use fuels::programs::call_response::FuelCallResponse;
use fuels::types::Identity;

abigen!(Contract(
    name = "TestToken",
    abi = "out/debug/asset-error-demo-abi.json"
));

const MOCK_TEST_TOKEN_BINARY_PATH: &str = "out/debug/asset-error-demo.bin";

#[tokio::test]
async fn proper_initialization() {
    let (test_token_a, test_token_b, wallet, _) = setup(Some(4)).await;
    let amount: u64 = 1000;
    let recipient: Identity = Identity::Address(wallet.address().into());

    mint_to(&test_token_a, recipient.clone(), amount).await;
    mint_to(&test_token_b, recipient, amount).await;

    set_recievable(&test_token_a, test_token_b.contract_id().into()).await;

    let asset_id: AssetId = test_token_b
        .contract_id()
        .asset_id(&BASE_ASSET_ID.into())
        .into();

    println!("asset_id: {:?}", asset_id);

    on_recieve(&test_token_a, amount, asset_id).await;
}

pub async fn deploy_mock_test_token_contract(wallet: &WalletUnlocked) -> TestToken<WalletUnlocked> {
    let mut rng = rand::thread_rng();
    let salt = rng.gen::<[u8; 32]>();
    let tx_parms = TxParameters::default().with_gas_price(1);

    let id = Contract::load_from(
        &MOCK_TEST_TOKEN_BINARY_PATH,
        LoadConfiguration::default().with_salt(salt),
    )
    .unwrap()
    .deploy(&wallet.clone(), tx_parms)
    .await
    .unwrap();

    TestToken::new(id, wallet.clone())
}

pub async fn mint_to(
    test_token: &TestToken<WalletUnlocked>,
    recipient: Identity,
    amount: u64,
) -> FuelCallResponse<()> {
    let tx_params = TxParameters::default().with_gas_price(1);

    test_token
        .methods()
        .mint_to_id(amount, recipient)
        .tx_params(tx_params)
        .append_variable_outputs(3)
        .call()
        .await
        .unwrap()
}

pub async fn set_recievable(
    test_token: &TestToken<WalletUnlocked>,
    contract: ContractId,
) -> FuelCallResponse<()> {
    let tx_params = TxParameters::default().with_gas_price(1);

    test_token
        .methods()
        .set_recievable(contract)
        .tx_params(tx_params)
        .call()
        .await
        .unwrap()
}

pub async fn on_recieve(
    test_token: &TestToken<WalletUnlocked>,
    amount: u64,
    asset_id: AssetId,
) -> FuelCallResponse<bool> {
    let tx_params = TxParameters::default().with_gas_price(1);

    let call_params: CallParameters = CallParameters::default()
        .with_amount(amount)
        .with_asset_id(asset_id);

    test_token
        .methods()
        .on_recieve()
        .call_params(call_params)
        .unwrap()
        .tx_params(tx_params)
        .append_variable_outputs(3)
        .call()
        .await
        .unwrap()
}

pub async fn set_asset_id(
    test_token: &TestToken<WalletUnlocked>,
    asset_id: AssetId,
) -> FuelCallResponse<()> {
    let tx_params = TxParameters::default().with_gas_price(1);

    test_token
        .methods()
        .set_asset_id(asset_id)
        .tx_params(tx_params)
        .call()
        .await
        .unwrap()
}

pub async fn setup(
    num_wallets: Option<u64>,
) -> (
    TestToken<WalletUnlocked>,
    TestToken<WalletUnlocked>,
    WalletUnlocked,
    Vec<WalletUnlocked>,
) {
    // Launch a local network and deploy the contract
    let config = Config {
        manual_blocks_enabled: true, // Necessary so the `produce_blocks` API can be used locally
        ..Config::local_node()
    };

    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            num_wallets,         /* Single wallet */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        Some(config),
        None,
    )
    .await;

    let wallet = wallets.pop().unwrap();

    let test_token_instance = deploy_mock_test_token_contract(&wallet).await;
    let test_token_instance_b = deploy_mock_test_token_contract(&wallet).await;

    (test_token_instance, test_token_instance_b, wallet, wallets)
}
