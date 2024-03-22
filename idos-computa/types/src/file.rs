use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct File<T>
where
    T: Clone,
{
    #[serde(rename = "type", default = "default_file_type")]
    pub r#type: String,
    pub params: FileParams,
    pub payload: Vec<T>,
}

pub fn default_file_type() -> String {
    "file".to_string()
}

impl<T> File<T>
where
    T: Clone,
{
    pub fn latest(self) -> Option<T> {
        self.payload.last().cloned()
    }

    pub fn commit(self, i: usize) -> T {
        self.payload[i].clone()
    }

    pub fn commit_id(self, i: usize) -> String {
        self.params.commit_ids[i].clone()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileParams {
    pub file_id: String,
    pub commit_ids: Vec<String>,
    pub attrs: Vec<String>,
    pub extractor: Vec<(String, String)>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Folder<T> {
    #[serde(rename = "type", default = "default_folder_type")]
    pub r#type: String,
    pub params: FolderParams,
    pub payload: HashMap<String, Vec<T>>,
}

fn default_folder_type() -> String {
    "folder".to_string()
}

impl<T> Folder<T>
where
    T: Clone,
{
    pub fn model(self, model_id: String) -> Option<Vec<T>> {
        self.payload.get(&model_id).map(Clone::clone)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FolderParams {
    pub folder_id: String,
    pub models: HashMap<String, Vec<String>>,
}
