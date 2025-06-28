use dioxus::prelude::*;
use tucant_types::{Tucan, gradeoverview::GradeOverviewRequest, moduledetails::ModuleDetailsRequest};

use crate::{RcTucanType, common::use_authenticated_data_loader};

#[component]
pub fn GradeOverview(gradeoverview: ReadOnlySignal<GradeOverviewRequest>) -> Element {
    let handler = async |tucan: RcTucanType, current_session, revalidation_strategy, additional| tucan.gradeoverview(&current_session, revalidation_strategy, additional).await;

    use_authenticated_data_loader(handler, gradeoverview, 14 * 24 * 60 * 60, 60 * 60, |gradeoverview, reload| {
        rsx! {
            div {
                h1 {
                }
            }
        }
    })
}
