use std::env::var;

use dotenvy::dotenv;
use mongodb::{error::Error, options::ClientOptions, Client, Collection, Database};

use crate::models::{TucanSession, UndoneUser, Module, ModuleCourse};

#[derive(Clone)]
pub struct MongoDb {
    pub client: Client,
    pub database: Database,
    pub users_unfinished: Collection<UndoneUser>,
    pub modules_unfinished: Collection<Module>,
    pub sessions: Collection<TucanSession>,
    pub module_courses: Collection<ModuleCourse>,
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

        Ok(Self {
            users_unfinished: database.collection("users_unfinished"),
            sessions: database.collection("sessions"),
            modules_unfinished: database.collection("modules_unfinished"),
            module_courses: database.collection("module_courses"),
            client,
            database,
        })
    }
}
