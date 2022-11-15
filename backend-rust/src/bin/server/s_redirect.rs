// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::MyError;

use actix_web::http::header;

use actix_web::HttpRequest;
use actix_web::HttpResponse;

use actix_web::web::Data;

use tucant::models::TucanSession;

use tucant::url::parse_tucan_url;

use tucant::tucan::Tucan;

pub async fn redirect(
    _session: TucanSession,
    req: HttpRequest,
    _tucan: Data<Tucan>,
) -> Result<HttpResponse, MyError> {
    // http://localhost:5173/redirect?https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=COURSEDETAILS&ARGUMENTS=-N741130168120888,-N000311,-N0,-N382005035435556,-N382005035412557,-N0,-N0,-N3,-AYzGDcSo37ZRa4WLeWSaZxWHfmDP0fNl-foV9YQcjxdZIeqKpVdeFOYGuYoPkCYm9Rjl-7jmeHfLpQuU-YfKJmWKBfYa6RgDFmImyOUcNVWUUQ-PuOz6IxZa64Wm7HDLqPDm7HIR9RWUgWuK0Ru5yHgm9VNnj3uKJxWoCmzmNxNmPfjRyfBA87giAxjmZ4vZbPjoXvo5ZWzAsxqWI4B5Nf-BAvzHpmzGoWU5IeQ5wmdHBcDRSvDmlR-ntWUW-mW5CWILXQ-U6cjD9VfKhWd6XVBZ3mdLh3u5EHZawPSWvv-ndVD5ZVdUzHfGk7jAUfB6deUPqH-mwVQ5jVgU6WMRsxMPH3IpXRq5acBLYPZUKQBeNrDGEHzHZx-RfOdGKmY69OqRhVz6sW-U0QNRbfIDZRSRtRSi-7UBNQzGUOZULPdPEcgHLrDDAOuPy7Q5KRffdQQWwxIBNHBKMVq2AOQWUxYGxRZnZYzGlxNKlYo5ZrgpD4BnZCYHZPjAyHS5V3fZXPgmdvMmvf-WN7MBtfoWTPvin
    let input = req.query_string();
    println!("{}", input);
    let url = match parse_tucan_url(input).program {
        tucant::url::TucanProgram::Registration(_) => todo!(),
        tucant::url::TucanProgram::RootRegistration(_) => todo!(),
        tucant::url::TucanProgram::StudentResult(_) => todo!(),
        tucant::url::TucanProgram::Moduledetails(_) => todo!(),
        tucant::url::TucanProgram::Coursedetails(course_details) => req.url_for(
            "course",
            [base64::encode_config(
                course_details.id,
                base64::URL_SAFE_NO_PAD,
            )],
        )?,
        tucant::url::TucanProgram::StartpageDispatch(_) => todo!(),
        tucant::url::TucanProgram::Externalpages(_) => todo!(),
        _ => return Ok(HttpResponse::NotFound().finish()),
    };

    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, url.as_str()))
        .finish())
}
