use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct RegistryStruct {
    pub name: String,
    pub description: String,
    pub homepage: String,
    pub licenses: Vec<String>,
    pub languages: Vec<String>,
    pub categories: Vec<String>,
    pub source: SourceStruct,
    pub bin: Option<HashMap<String, String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceStruct {
    pub id: String,
    pub asset: Option<Vec<Assets>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Assets {
    pub target: String,
    pub file: String,
    pub bin: String,
}
