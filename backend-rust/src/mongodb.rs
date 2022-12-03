use std::env::var;

use dotenvy::dotenv;
use mongodb::{error::Error, options::ClientOptions, Client, Collection, Database};

use crate::models::{TucanSession, UndoneUser};

#[derive(Clone)]
pub struct MongoDb {
    pub client: Client,
    pub database: Database,
    pub users_unfinished: Collection<UndoneUser>,
    pub sessions: Collection<TucanSession>,
}

impl MongoDb {
    pub async fn new() -> Result<Self, Error> {
        dotenv().unwrap();
        let database_url = var("DATABASE_URL").expect("DATABASE_URL must be set");

        let mut client_options = ClientOptions::parse(database_url).await?;

        client_options.app_name = Some("tucant".to_string());
        // TODO FIXME some consistency values probably need to be set here

        let client = Client::with_options(client_options)?;

        let database = client.database("tucant");

        let users_unfinished = database.collection::<UndoneUser>("users_unfinished");
        let sessions = database.collection::<TucanSession>("sessions");

        Ok(Self {
            client,
            database,
            users_unfinished,
            sessions,
        })
    }
}
