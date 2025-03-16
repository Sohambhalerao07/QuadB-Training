use ic_cdk_macros::{query, update};

#[query]
fn greet(name: String) -> String {
    format!("Hello, {}! Welcome to the Internet Computer!!", name)
}

#[update]
fn set_greeting(greeting: String) {
    ic_cdk::println!("greeting: {}", greeting);
}
