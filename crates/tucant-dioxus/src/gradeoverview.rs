use dioxus::prelude::*;
use plotters::prelude::*;
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
                    {
                    let mut output = String::new();
                    {
                        let root = SVGBackend::with_string(&mut output, (640, 480)).into_drawing_area();
                        root.fill(&WHITE)?;

                        let mut chart = ChartBuilder::on(&root)
                            .x_label_area_size(35)
                            .y_label_area_size(40)
                            .margin(5)
                            .caption("Histogram Test", ("sans-serif", 50.0))
                            .build_cartesian_2d((0usize..15usize).into_segmented(), 0u32..100u32)?;

                        chart
                            .configure_mesh()
                            .disable_x_mesh()
                            .bold_line_style(WHITE.mix(0.3))
                            .y_desc("Count")
                            .x_desc("Bucket")
                            .axis_desc_style(("sans-serif", 15))
                            .draw()?;

                        chart.draw_series(
                            Histogram::vertical(&chart)
                                .style(RED.mix(0.5).filled())
                                .data(grades.columns.iter().enumerate().map(|(idx, column)| {
                                    (idx, column.1.parse::<u32>().unwrap())
                                })),
                        )?;

                        root.present()?;
                    }

                    rsx! { div { dangerous_inner_html: output } }
                }
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
