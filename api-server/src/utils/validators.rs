use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(try_from = "String")]
pub struct MaxSizedString<const SIZE: usize>(pub String);

impl<const SIZE: usize> TryFrom<String> for MaxSizedString<SIZE> {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() <= SIZE {
            Ok(Self(value))
        } else {
            Err(format!(
                "This field only allow {SIZE} characters. Found {}.",
                value.len()
            ))
        }
    }
}

#[test]
fn parse_valid_max_sized_string() {
    let string = MaxSizedString::<16>::try_from(String::from("hello"));
    assert!(string.is_ok())
}

#[test]
fn parse_invalid_sized_string() {
    let string = MaxSizedString::<2>::try_from(String::from("hello"));
    assert!(string.is_err())
}
