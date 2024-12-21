use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
