use tucan_connector::{moduledetails::index::moduledetails, root::root, Tucan};
use tucant_types::{moduledetails::ModuleDetailsRequest, LoginResponse};

#[tokio::test]
pub async fn root_page() {
    let tucan = Tucan::new().await.unwrap();
    root(&tucan.client).await.unwrap();
}
