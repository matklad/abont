pub struct AText {
    text: String,
}

impl From<&'_ str> for AText {
    fn from(value: &str) -> AText {
        AText {
            text: value.to_string(),
        }
    }
}
