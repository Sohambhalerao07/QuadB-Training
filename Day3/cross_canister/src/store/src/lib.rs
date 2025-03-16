use ic_cdk::api::call::call;
use candid::Principal;
use ic_cdk::export_candid;

#[ic_cdk::update]
async fn buy_item(wallet_canister_id: String, price: u64) -> String {
    let wallet: Principal = wallet_canister_id.parse().unwrap();

    let result: Result<(), String> = call(wallet, "withdraw", (price,)).await
        .map_err(|(code, msg)| format!("Error: {:?} - {}", code, msg));  // Using `{:?}` to print `RejectionCode`

    match result {
        Ok(_) => "Purchase successful!".to_string(),
        Err(e) => format!("Purchase failed: {}", e),
    }
}

export_candid!();
