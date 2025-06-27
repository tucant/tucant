use std::{convert::Infallible, fmt::Display, str::FromStr, sync::LazyLock};

use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, ToSchema)]
pub struct GradeOverviewRequest(String);

impl FromStr for GradeOverviewRequest {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse(s))
    }
}

impl Display for GradeOverviewRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl GradeOverviewRequest {
    #[must_use]
    pub fn parse(input: &str) -> Self {
        static GRADEOVERVIEW_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^-AEXEV,-N(?P<course_id>\d+),-N0,-N,-N(?P<semester_id>\d+),-A,-N,-A,-N,-N,-N2,-N(?P<id>\d+)$").unwrap());
        let c = &GRADEOVERVIEW_REGEX.captures(input).expect(input);
        Self(format!("-AEXEV,-N{},-N0,-N,-N{},-A,-N,-A,-N,-N,-N2,-N{}", &c["course_id"], &c["semester_id"], &c["id"]))
    }

    #[must_use]
    pub const fn inner(&self) -> &str {
        self.0.as_str()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, ToSchema)]
pub struct GradeOverviewResponse {

}