use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ModuleDetailsRequest {
    pub arguments: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDetailsResponse {}
