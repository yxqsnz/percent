use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct AccountCreate {
    #[validate(length(min = 2, max = 32))]
    pub full_name: Option<String>,
    #[validate(length(min = 3, max = 16))]
    pub nick: String,
    #[validate(length(min = 8, max = 72))]
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct AccountJoin {
    #[validate(length(min = 3, max = 16))]
    pub nick: String,
    #[validate(length(min = 8, max = 72))]
    pub password: String,
}

#[test]
fn account_create_password_too_small() {
    assert!(AccountCreate {
        full_name: None,
        nick: "me123".into(),
        password: String::new(),
    }
    .validate()
    .is_err())
}

#[test]
fn account_create_nick_too_small() {
    assert!(AccountCreate {
        full_name: None,
        nick: "me".into(),
        password: String::from("12345678"),
    }
    .validate()
    .is_err())
}

#[test]
fn account_join_nick_too_small() {
    assert!(AccountJoin {
        nick: "me".into(),
        password: String::from("12345678"),
    }
    .validate()
    .is_err())
}

#[test]
fn account_join_password_too_small() {
    assert!(AccountJoin {
        nick: "me123".into(),
        password: String::new(),
    }
    .validate()
    .is_err())
}
#[test]
fn account_create_valid() {
    assert!(AccountCreate {
        full_name: None,
        nick: "me_at_you".into(),
        password: String::from("12345678"),
    }
    .validate()
    .is_ok())
}
#[test]
fn account_join_valid() {
    assert!(AccountJoin {
        nick: "me_at_you".into(),
        password: String::from("12345678"),
    }
    .validate()
    .is_ok())
}
