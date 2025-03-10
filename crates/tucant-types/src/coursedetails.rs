use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct CourseDetailsRequest {
    pub arguments: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseDetailsResponse {
    pub name: String,
    pub material_and_messages_url: Option<(String, String)>,
    pub dozent: Option<String>,
    pub r#type: String,
    pub type_number: u64,
    pub fachbereich: String,
    pub anzeige_im_stundenplan: String,
    pub shortname: String,
    pub courselevel: u64,
    pub sws: Option<u64>,
    pub credits: Option<u64>,
    pub language: String,
    pub language_id: u64,
    pub teilnehmer_range: String,
    pub teilnehmer_max: String,
    pub description: Vec<String>,
    pub uebungsgruppen: Vec<CourseUebungsGruppe>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseUebungsGruppe {
    pub name: String,
    pub uebungsleiter: String,
    pub date_range: Option<String>,
}
