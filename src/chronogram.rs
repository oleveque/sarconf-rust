use eframe::egui::{Color32, Ui, Vec2b};
use egui_plot::{Legend, Line, Plot, Corner};

pub struct Window {
    pub name: String,
    pub start_time: f64,
    pub duration: f64,
    pub height: f64,
    pub dashed: bool,
    pub color: Option<Color32>,
}

impl Default for Window {
    fn default() -> Self {
        Self {
            name: "Untitled".to_string(),
            start_time: 0.0,
            duration: 1.0,
            height: 1.0,
            dashed: false,
            color: None,
        }
    }
}

impl Window {
    pub fn start(&self) -> f64 {
        self.start_time
    }

    pub fn end(&self) -> f64 {
        self.start_time + self.duration
    }
}

pub fn plot(ui: &mut Ui, pri: f64, windows: Vec<Window>) {
    let mut nb_of_ambiguities = 1;
    for window in &windows {
        let end_time = window.end();
        if end_time > nb_of_ambiguities as f64 * pri {
            nb_of_ambiguities = (end_time / pri).ceil() as usize + 1;
        }
    }

    let legend = Legend::default().position(Corner::RightBottom);

    Plot::new("Chronogram")
        .height(100.0)
        .show_y(false)
        .allow_boxed_zoom(false)
        .allow_drag(Vec2b::new(true, false))
        .allow_zoom(Vec2b::new(true, false))
        .allow_scroll(Vec2b::new(true, false))
        .show_axes(Vec2b::new(true, false))
        .show_grid(Vec2b::new(true, false))
        .default_y_bounds(0.0, 1.1)
        .x_axis_formatter(|x, _| format!("{:.1} µs", x.value))
        .include_x(nb_of_ambiguities as f64 * pri)
        .legend(legend)
        .show(ui, |plot_ui| {
            for window in windows {
                for i in 0..nb_of_ambiguities {
                    let mut w = Line::new(
                            if i > 0 {
                                format!("{} (Ambiguity {})", window.name, i)
                            } else {
                                window.name.clone()
                            },
                            vec![
                                [pri * i as f64 + window.start(), 0.0],
                                [pri * i as f64 + window.start(), window.height],
                                [pri * i as f64 + window.end(), window.height],
                                [pri * i as f64 + window.end(), 0.0],
                            ],
                        ).width(2.0).fill(0.0).fill_alpha(0.6 / (i as f32 + 1.0));

                    if window.dashed {
                        w = w.style(egui_plot::LineStyle::Dashed { length: 5.0 });
                    }

                    if let Some(c) = window.color {
                        w = w.color(c);
                    }

                    plot_ui.line(w);
                }
            }
        });
}