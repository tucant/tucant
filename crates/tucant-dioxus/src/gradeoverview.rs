use dioxus::prelude::*;
use tucant_types::{
    Tucan,
    gradeoverview::{GradeOverviewRequest, GradeOverviewResponse},
    moduledetails::ModuleDetailsRequest,
};

use crate::{RcTucanType, common::use_authenticated_data_loader};

#[component]
pub fn GradeOverview(gradeoverview: ReadOnlySignal<GradeOverviewRequest>) -> Element {
    let handler = async |tucan: RcTucanType, current_session, revalidation_strategy, additional| tucan.gradeoverview(&current_session, revalidation_strategy, additional).await;

    use_authenticated_data_loader(handler, gradeoverview, 14 * 24 * 60 * 60, 60 * 60, |gradeoverview: GradeOverviewResponse, reload| {
        rsx! {
            div {
                h1 {
                    { gradeoverview.module_and_semester }
                }
                h3 {
                    { gradeoverview.modulangebot }
                }
                h3 {
                    { gradeoverview.studienleistung }
                }
                if let Some(grades) = gradeoverview.maybe_grades {
                    { grades.columns.iter().map(|column| {
                        rsx! {
                            {column.0.clone()}
                            " "
                            {column.1.clone()}
                            br {}
                         }
                    }) }
                    {
                        grades.infos.iter().map(|info| {
                            rsx! {
                                {info.clone()}
                                br {}
                            }
                        })
                    }
                }
            }
        }
    })
}
