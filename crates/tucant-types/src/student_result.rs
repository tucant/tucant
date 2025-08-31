use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::LeistungsspiegelGrade;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CourseOfStudySelection {
    pub name: String,
    pub value: u64,
    pub selected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct StudentResultEntry {
    /// None means Anerkennung
    pub id: Option<String>,
    pub name: String,
    pub resultdetails_url: Option<String>,
    pub cp: Option<u64>,
    pub used_cp: Option<u64>,
    pub grade: LeistungsspiegelGrade,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub enum StudentResultState {
    Bestanden,
    NichtBestanden,
    Unvollstaendig,
    Offen,
}

impl From<(&str, &str, &str)> for StudentResultState {
    fn from(value: (&str, &str, &str)) -> Self {
        match value {
            ("/img/individual/pass.gif", "Bestanden", "Bestanden") => Self::Bestanden,
            ("/img/individual/fail.gif", "Nicht Bestanden", "Nicht Bestanden") => Self::NichtBestanden,
            ("/img/individual/incomplete.gif", "Unvollständig", "Unvollständig") => Self::Unvollstaendig,
            ("/img/individual/open.gif", "Offen", "Offen") => Self::Offen,
            s => panic!("{s:?}"),
        }
    }
}

impl std::fmt::Display for StudentResultState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bestanden => write!(f, "Bestanden"),
            Self::NichtBestanden => write!(f, "Nicht Bestanden"),
            Self::Unvollstaendig => write!(f, "Unvollständig"),
            Self::Offen => write!(f, "Offen"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct StudentResultLevel {
    /// DO NOT ASK why there are sections with no title
    pub name: Option<String>,
    pub entries: Vec<StudentResultEntry>,
    pub sum_cp: Option<u64>,
    pub sum_used_cp: Option<u64>,
    pub state: Option<StudentResultState>,
    pub rules: StudentResultRules,
    pub children: Vec<StudentResultLevel>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct StudentResultResponse {
    pub course_of_study: Vec<CourseOfStudySelection>,
    pub level0: StudentResultLevel,
    pub total_gpa: String,
    pub main_gpa: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct StudentResultRules {
    pub min_cp: u64,
    pub max_cp: Option<u64>,
    pub min_modules: u64,
    pub max_modules: Option<u64>,
}
