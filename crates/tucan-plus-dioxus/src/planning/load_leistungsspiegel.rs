use dioxus::{hooks::use_context, html::MouseData};
use fragile::Fragile;
use log::info;
use tucan_plus_worker::{
    ChildUrl,
    models::{AnmeldungEntry, Semester, State},
};
use tucan_types::{
    LeistungsspiegelGrade, LoginResponse, RevalidationStrategy, SemesterId, Tucan as _,
    student_result::{StudentResultLevel, StudentResultResponse},
};
use web_sys::Worker;

use crate::{RcTucanType, send_message, student_result::StudentResult};

pub async fn recursive_update(
    worker: Fragile<Worker>,
    course_of_study: &str,
    url: String,
    level: StudentResultLevel,
) {
    for child in level.children {
        let name = child.name.as_ref().unwrap();
        let child_url = send_message(
            &worker,
            ChildUrl {
                course_of_study: course_of_study.to_string(),
                url: url.clone(),
                name: name.clone(),
                child,
            },
        )
        .await;
        info!("updated");
        Box::pin(recursive_update(worker, course_of_study, child_url, child)).await;
    }
    let inserts: Vec<_> = level
        .entries
        .iter()
        .map(|entry| AnmeldungEntry {
            course_of_study: course_of_study.to_owned(),
            available_semester: Semester::Sommersemester, // TODO FIXME
            anmeldung: url,
            module_url: "TODO".to_owned(), // TODO FIXME
            id: entry.id.as_ref().unwrap_or(&entry.name).to_owned(), /* TODO FIXME, use two columns
                                            * and both as primary key */
            credits: i32::try_from(entry.used_cp.unwrap_or_else(|| {
                if level.name.as_deref() == Some("Masterarbeit") {
                    30
                } else {
                    0
                }
            }))
            .unwrap(),
            name: entry.name,
            state: if matches!(
                entry.grade,
                LeistungsspiegelGrade::Grade(_) | LeistungsspiegelGrade::BestandenOhneNote
            ) {
                State::Done
            } else {
                State::Planned
            },
            year: None,
            semester: None,
        })
        .collect();
    diesel::insert_into(anmeldungen_entries::table)
        .values(&inserts)
        .on_conflict((
            anmeldungen_entries::course_of_study,
            anmeldungen_entries::anmeldung,
            anmeldungen_entries::available_semester,
            anmeldungen_entries::id,
        ))
        .do_update()
        .set((
            anmeldungen_entries::state.eq(excluded(anmeldungen_entries::state)),
            (anmeldungen_entries::credits.eq(excluded(anmeldungen_entries::credits))),
        ))
        .execute(&mut *connection_clone.borrow_mut())
        .expect("Error saving anmeldungen");
}

pub async fn load_leistungsspiegel(
    current_session: LoginResponse,
    tucan: RcTucanType,
    student_result: StudentResultResponse,
    course_of_study: String,
) {
    let worker: Fragile<Worker> = use_context();

    // top level anmeldung has name "M.Sc. Informatik (2023)"
    // top level leistungsspiegel has "Informatik"

    let name = &student_result
        .course_of_study
        .iter()
        .find(|e| e.selected)
        .unwrap()
        .name;
    let the_url: String = diesel::update(QueryDsl::filter(
        anmeldungen_plan::table,
        anmeldungen_plan::course_of_study
            .eq(&course_of_study)
            .and(anmeldungen_plan::name.eq(name)),
    ))
    .set((
        anmeldungen_plan::min_cp.eq(student_result.level0.rules.min_cp as i32),
        anmeldungen_plan::max_cp.eq(student_result.level0.rules.max_cp.map(|v| v as i32)),
        anmeldungen_plan::min_modules.eq(student_result.level0.rules.min_modules as i32),
        anmeldungen_plan::max_modules.eq(student_result.level0.rules.max_modules.map(|v| v as i32)),
    ))
    .returning(anmeldungen_plan::url)
    .get_result(&mut *connection_clone.borrow_mut())
    .expect("Error updating anmeldungen");

    recursive_update(&course_of_study, the_url, student_result.level0).await;

    let semesters = tucan
        .course_results(
            &current_session,
            RevalidationStrategy::cache(),
            SemesterId::current(),
        )
        .await
        .unwrap();
    for semester in semesters.semester {
        let result = tucan
            .course_results(
                &current_session,
                RevalidationStrategy::cache(),
                semester.value,
            )
            .await
            .unwrap();
        for module in result.results {
            diesel::update(anmeldungen_entries::table)
                .filter(
                    anmeldungen_entries::course_of_study
                        .eq(&course_of_study)
                        .and(
                            anmeldungen_entries::id
                                .eq(module.nr)
                                // TODO FIXME if you can register it at multiple paths
                                // this will otherwise break
                                .and(anmeldungen_entries::state.ne(State::NotPlanned)),
                        ),
                )
                .set((
                    anmeldungen_entries::semester.eq(if semester.name.starts_with("SoSe ") {
                        Semester::Sommersemester
                    } else {
                        Semester::Wintersemester
                    }),
                    (anmeldungen_entries::year.eq(semester.name[5..9].parse::<i32>().unwrap())),
                ))
                .execute(&mut *connection_clone.borrow_mut())
                .expect("Error updating anmeldungen");
        }
    }
}
