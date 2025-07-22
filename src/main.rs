use eframe::egui;
mod geometry;
mod chronogram;

struct ExaConfApp {
    config_name: String,

    // Antenna parameters
    elevation_aperture_angle: f64,
    azimuth_aperture_angle: f64,

    // Geometry parameters
    carrier_speed: f64,
    carrier_altitude: f64,
    depression_angle: f64,

    // Transmission parameters
    pri: f64,
    tx_offset: f64,
    tx_duration: f64,

    // Receiver parameters
    nb_agility: u32,
    nb_channel: u32,
    fech: f64,
    rx_offset: f64,
    rx_duration: f64,
    rx_noise_offset: f64,
    rx_noise_duration: f64,
    rx_reinj_offset: f64,
    rx_reinj_duration: f64,

    // SAR sensitivity parameters
    power: f64,
    lose_power: f64,
    gain_antenna: f64,
    noise_factor: f64,
    central_frequency: f64,
    bandwidth: f64,

    // Level parameters
    retrodiff: f64,
    rx_gain: f64,

    // Interference parameters
    height_ambiguity: f64,
    accuracy_height_ambiguity: f64,
}

impl Default for ExaConfApp {
    fn default() -> Self {
        Self {
            config_name: String::from("Untitled"),
            elevation_aperture_angle: 18.0,
            azimuth_aperture_angle: 0.0,
            carrier_speed: 120.0,
            carrier_altitude: 180.0,
            depression_angle: 45.0,
            pri: 100.0,
            tx_offset: 0.0,
            tx_duration: 10.0,
            nb_agility: 1,
            nb_channel: 1,
            fech: 0.0,
            rx_offset: 2.0,
            rx_duration: 50.0,
            rx_noise_offset: 3.0,
            rx_noise_duration: 4.0,
            rx_reinj_offset: 0.0,
            rx_reinj_duration: 18.0,
            power: 0.0,
            lose_power: 0.0,
            gain_antenna: 0.0,
            noise_factor: 0.0,
            central_frequency: 0.0,
            bandwidth: 0.0,
            retrodiff: 0.0,
            rx_gain: 0.0,
            height_ambiguity: 0.0,
            accuracy_height_ambiguity: 0.0,
        }
    }
}

fn number_input_with_unit(ui: &mut egui::Ui, value: &mut f64, label: &str, unit: &str, tooltip: Option<&str>) {
    ui.horizontal(|ui| {
        ui.label(label);
        if let Some(t) = tooltip {
            ui.add(egui::Label::new("❓").sense(egui::Sense::click())).on_hover_text(t);
        }
        ui.add(egui::DragValue::new(value).speed(0.1));
        ui.label(unit);
    });
}

impl eframe::App for ExaConfApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::TopBottomPanel::top("top_panel")
            .resizable(true)
            .min_height(32.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Configuration Name:");
                    ui.text_edit_singleline(&mut self.config_name);
                });
            });

        egui::SidePanel::left("left_panel")
            .resizable(true)
            .default_width(200.0)
            .width_range(80.0..=300.0)
            .show(ctx, |ui| {
                // Bloc dépliable pour le premier ensemble de paramètres
                egui::CollapsingHeader::new("Antenna parameters").show(ui, |ui| {
                    number_input_with_unit(ui, &mut self.elevation_aperture_angle, "Elevation Aperture Angle:", "°", Some("This is the elevation aperture angle"));
                    number_input_with_unit(ui, &mut self.azimuth_aperture_angle, "Azimuth Aperture Angle:", "°", Some("This is the azimuth aperture angle"));
                });

                // Bloc dépliable pour le deuxième ensemble de paramètres
                egui::CollapsingHeader::new("Geometry parameters").show(ui, |ui| {
                    number_input_with_unit(ui, &mut self.carrier_altitude, "Carrier Altitude:", "m", Some("This is the carrier altitude"));
                    number_input_with_unit(ui, &mut self.depression_angle, "Depression Angle:", "°", Some("This is the depression angle"));
                });
            });

        egui::SidePanel::right("right_panel")
            .resizable(true)
            .default_width(150.0)
            .width_range(80.0..=200.0)
            .show(ctx, |ui| {
                egui::CollapsingHeader::new("Emission parameters").show(ui, |ui| {
                    number_input_with_unit(ui, &mut self.tx_offset, "Tx Offset:", "µs", Some("This is the Tx offset"));
                    number_input_with_unit(ui, &mut self.tx_duration, "Tx Duration:", "µs", Some("This is the Tx duration"));
                });
                egui::CollapsingHeader::new("Reception parameters").show(ui, |ui| {
                    number_input_with_unit(ui, &mut self.rx_offset, "Rx Offset:", "µs", Some("This is the Rx offset"));
                    number_input_with_unit(ui, &mut self.rx_duration, "Rx Duration:", "µs", Some("This is the Rx duration"));
                    number_input_with_unit(ui, &mut self.rx_noise_offset, "Rx Noise Offset:", "µs", Some("This is the Rx noise offset"));
                    number_input_with_unit(ui, &mut self.rx_noise_duration, "Rx Noise Duration:", "µs", Some("This is the Rx noise duration"));
                    number_input_with_unit(ui, &mut self.rx_reinj_offset, "Rx Reinj Offset:", "µs", Some("This is the Rx reinjection offset"));
                    number_input_with_unit(ui, &mut self.rx_reinj_duration, "Rx Reinj Duration:", "µs", Some("This is the Rx reinjection duration"));
                });
            });

        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .min_height(0.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Bottom Panel");
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Geometry Plot");
            let min_elevation_angle = self.depression_angle - self.elevation_aperture_angle / 2.0;
            let max_elevation_angle = self.depression_angle + self.elevation_aperture_angle / 2.0;
            geometry::plot(ui,
                (0.0, self.carrier_altitude),
                self.depression_angle,
                Some((min_elevation_angle, max_elevation_angle))
            );

            ui.heading("Chronogram");
            let windows = vec![
                chronogram::Window {
                    name: "Tx".to_string(),
                    start_time: self.tx_offset,
                    duration: self.tx_duration,
                    height: 1.0,
                    color: Some(egui::Color32::from_rgb(255, 0, 0)),
                },
                chronogram::Window {
                    name: "Rx".to_string(),
                    start_time: self.rx_offset,
                    duration: self.rx_duration,
                    height: 1.0,
                    color: Some(egui::Color32::from_rgb(0, 255, 0)),
                },
                chronogram::Window {
                    name: "Noise".to_string(),
                    start_time: self.rx_noise_offset,
                    duration: self.rx_noise_duration,
                    height: 0.8,
                    color: Some(egui::Color32::from_rgb(0, 0, 255)),
                },
                chronogram::Window {
                    name: "Reinj".to_string(),
                    start_time: self.rx_reinj_offset,
                    duration: self.rx_reinj_duration,
                    height: 0.8,
                    color: Some(egui::Color32::from_rgb(255, 255, 0)),
                },
            ];
            chronogram::plot(ui, self.pri, windows);
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "ExaConf",
        options,
        Box::new(|_cc| Ok(Box::new(ExaConfApp::default()))),
    );
}
