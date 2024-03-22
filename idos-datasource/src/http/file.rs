pub static BASE_URL: &str = "https://file-relayer-temp.dataverse.art";

pub struct Client {
    base_url: String,
}

impl Client {
    pub fn new(base_url: Option<String>) -> Self {
        Client {
            base_url: base_url.unwrap_or_else(|| BASE_URL.into()),
        }
    }
}
