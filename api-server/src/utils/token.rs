use crate::models::account::Account;

pub fn generate_from_account(account: &Account) -> String {
    format!("{}:{}", account.nick, account.password)
}
