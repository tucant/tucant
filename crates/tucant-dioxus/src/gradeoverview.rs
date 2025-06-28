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
        let mut output = String::new();
        {
            let root = SVGBackend::with_string(&mut output, (640, 480)).into_drawing_area();
            root.fill(&WHITE);
            let root = root.margin(10, 10, 10, 10);
            // After this point, we should be able to construct a chart context
            let mut chart = ChartBuilder::on(&root)
                // Set the caption of the chart
                .caption("This is our first plot", ("sans-serif", 40).into_font())
                // Set the size of the label region
                .x_label_area_size(20)
                .y_label_area_size(40)
                // Finally attach a coordinate on the drawing area and make a chart context
                .build_cartesian_2d(0f32..10f32, 0f32..10f32)?;

            // Then we can draw a mesh
            chart
                .configure_mesh()
                // We can customize the maximum number of labels allowed for each axis
                .x_labels(5)
                .y_labels(5)
                // We can also change the format of the label text
                .y_label_formatter(&|x| format!("{:.3}", x))
                .draw()?;

            // And we can draw something in the drawing area
            chart.draw_series(LineSeries::new(vec![(0.0, 0.0), (5.0, 5.0), (8.0, 7.0)], &RED))?;
            // Similarly, we can draw point series
            chart.draw_series(PointSeries::of_element(vec![(0.0, 0.0), (5.0, 5.0), (8.0, 7.0)], 5, &RED, &|c, s, st| {
                return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
            + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
            + Text::new(format!("{:?}", c), (10, 0), ("sans-serif", 10).into_font());
            }))?;
            root.present()?;
        }

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
                    div { dangerous_inner_html: output }
                }
            }
        }
    })
}
