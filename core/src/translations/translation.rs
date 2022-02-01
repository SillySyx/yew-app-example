#[derive(Clone, PartialEq)]
pub struct Translation {
    pub key: String,
    pub value: String,
}

impl Translation {
    pub fn from(key: &'static str, value: &'static str) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}