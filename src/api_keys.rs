#[derive(Debug, Clone)]
pub struct ApiKeys {
    pub public_key: String,
    pub private_key: String,
}

impl ApiKeys {
    pub fn new(public_key: impl Into<String>, private_key: impl Into<String>) -> Self {
        Self {
            public_key: public_key.into(),
            private_key: private_key.into(),
        }
    }
}
