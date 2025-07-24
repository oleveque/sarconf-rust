use eframe::egui::{Color32, Ui};
use egui_plot::{Plot, Line, PlotPoints, Legend, Corner};

fn arc_to_points(radius: f64, start_angle_deg: f64, end_angle_deg: f64, center: (f64, f64)) -> PlotPoints<'static> {
    let points: Vec<[f64; 2]> = (0..=100)
        .map(|i| {
            let t = i as f64 / 100.0;
            let angle = start_angle_deg.to_radians() + t * (end_angle_deg - start_angle_deg).to_radians();
            [
                center.0 + radius * angle.cos(),
                center.1 - radius * angle.sin(),
            ]
        })
        .collect();
    points.into()
}

fn arc_to_points_auto(radius: f64, center: (f64, f64)) -> PlotPoints<'static> {
    let start_angle_deg = 0.0;    
    if radius <= center.1 {
        let end_angle_deg = 90.0;
        return arc_to_points(radius, start_angle_deg, end_angle_deg, center.clone());
    } else {
        let end_angle_deg = 90.0 - (center.1 / radius).acos().to_degrees();
        return arc_to_points(radius, start_angle_deg, end_angle_deg, center.clone());
    }
}

pub fn plot(ui: &mut Ui, position: (f64, f64), look_angle_deg: f64, aperture_angles_deg: Option<(f64, f64)>, numerization_window: Option<(f64, f64)>) {

    let line = Line::new(
        "Carrier",
        vec![[0.0, 0.0], [position.0, position.1]],
    ).color(Color32::WHITE).style(egui_plot::LineStyle::Dashed { length: 5.0 });

    let target = Line::new(
        "Target",
        vec![[position.0, position.1], [position.0 + position.1 * look_angle_deg.to_radians().tan(), 0.0]],
    ).color(Color32::DARK_GREEN);

    let legend = Legend::default().position(Corner::RightTop);

    Plot::new("Geometry")
        .data_aspect(1.0)
        .height(300.0)
        .legend(legend)
        .x_axis_formatter(|x, _| format!("{:.1} m", x.value))
        .y_axis_formatter(|y, _| format!("{:.1} m", y.value))
        .show(ui, |plot_ui| {
            plot_ui.line(line);
            plot_ui.line(target);

            if let Some(lobe) = aperture_angles_deg {
                plot_ui.line(
                    Line::new(
                        "Lobe",
                        vec![
                            [position.0 + position.1 * lobe.0.to_radians().tan(), 0.0],
                            [position.0, position.1],
                            [position.0 + position.1 * lobe.1.to_radians().tan(), 0.0],
                        ],
                    ).color(Color32::BLUE)
                );
            }

            if let Some((start, end)) = numerization_window && start < end {
                plot_ui.line(
                    Line::new("RX Window", arc_to_points_auto(start, position.clone()))
                        .color(Color32::YELLOW)
                        .width(2.0)
                );
                plot_ui.line(
                    Line::new("RX Window", arc_to_points_auto(end, position.clone()))
                        .style(egui_plot::LineStyle::Dashed { length: 5.0 })
                        .color(Color32::YELLOW)
                        .width(2.0)
                );
            }
        });
}
