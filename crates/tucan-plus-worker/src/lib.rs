pub mod models;
pub mod schema;

pub enum Request {
    FetchAnmeldungen { course_of_study: String },
}
