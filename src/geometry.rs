use egui::{Color32, Ui};
use egui_plot::{Plot, Line};

pub fn plot(ui: &mut Ui, position: (f64, f64), depression_angle_deg: f64, aperture_angles_deg: Option<(f64, f64)>) {

    let line = Line::new(
        "Carrier",
        vec![[0.0, 0.0], [position.0, position.1]],
    ).color(Color32::WHITE).style(egui_plot::LineStyle::Dashed { length: 5.0 });

    let target = Line::new(
        "Target",
        vec![[position.0, position.1], [position.0 + position.1 * depression_angle_deg.to_radians().tan(), 0.0]],
    ).color(Color32::GREEN);

    Plot::new("Geometry")
        .data_aspect(1.0)
        .height(300.0)
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
        });
}
