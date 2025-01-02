use tucan_connector::{moduledetails::index::moduledetails, Tucan};
use tucant_types::{moduledetails::ModuleDetailsRequest, LoginResponse};

#[tokio::test]
pub async fn test_1() {
    let url = "https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MODULEDETAILS&ARGUMENTS=-N000000000000001,-N000311,-N389455489906019";
    let tucan = Tucan::new().await.unwrap();
    let result = moduledetails(
        &tucan,
        &LoginResponse {
            id: 1,
            cookie_cnsc: String::new(),
        },
        ModuleDetailsRequest {
            arguments: ",-N000311,-N389455489906019".to_owned(),
        },
    )
    .await
    .unwrap();
}
