use jsonptr::Pointer;
use serde_json::Value;
use serde_json_path::JsonPath;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Extractor(String, String);

impl Extractor {
    pub fn new(path: &str, patch: &str) -> Self {
        Extractor(path.into(), patch.into())
    }

    pub fn patch_from_content(&self, value: &Value, result: &mut Value) -> anyhow::Result<()> {
        let path = JsonPath::parse(&self.1)?;
        let got = path.query(&value).exactly_one()?;

        let ptr = Pointer::try_from(self.0.as_ref())?;
        ptr.assign(result, got.clone())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn patch_from_content() {
        let value = json!({
            "key": "value",
            "key2": "value2",
            "key3": {
                "key4": "value4"
            }
        });

        let mut result = json!({});

        let extractor = Extractor::new("/got1", "$.key");
        extractor.patch_from_content(&value, &mut result).unwrap();

        let expected = json!({
            "got1": "value"
        });

        assert_eq!(result, expected);
    }
}
