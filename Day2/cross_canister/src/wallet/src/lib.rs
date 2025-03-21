use ic_cdk::api::call::call;
use candid::Principal;
use ic_cdk::export_candid;

#[ic_cdk::query]
fn get_balance() -> u64 {
    BALANCE.with(|b| *b.borrow())
}

#[ic_cdk::update]
fn deposit(amount: u64) {
    BALANCE.with(|b| *b.borrow_mut() += amount);
}

#[ic_cdk::update]
fn withdraw(amount: u64) -> Result<(), String> {
    BALANCE.with(|b| {
        let mut balance = b.borrow_mut();
        if *balance >= amount {
            *balance -= amount;
            Ok(())
        } else {
            Err("Insufficient funds".to_string())
        }
    })
}

thread_local! {
    static BALANCE: std::cell::RefCell<u64> = std::cell::RefCell::new(100);
}

export_candid!();
