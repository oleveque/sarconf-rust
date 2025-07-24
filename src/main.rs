use eframe::egui;
mod geometry;
mod chronogram;

struct SARConfApp {
    config_name: String,

    // Antenna parameters
    elevation_aperture_angle: f64,
    azimuth_aperture_angle: f64,

    // Geometry parameters
    carrier_velocity: f64,
    carrier_height: f64,
    depression_angle: f64,

    // Transmission parameters
    pri: f64,
    tx_offset: f64,
    tx_duration: f64,
    nb_agility: u32,

    // Receiver parameters
    nb_channel: u32,
    fech: f64,
    rx_offset: f64,
    rx_duration: f64,
    rx_noise_offset: f64,
    rx_noise_duration: f64,
    rx_reinj_offset: f64,
    rx_reinj_duration: f64,

    // SAR sensitivity parameters
    peak_power: f64,
    loss_power: f64,
    gain_antenna: f64,
    noise_factor: f64,
    center_frequency: f64,
    bandwidth: f64,

    // Level parameters
    retrodiff: f64,
    rx_gain: f64,

    // Interference parameters
    height_ambiguity: f64,
    accuracy_height_ambiguity: f64,
}

impl Default for SARConfApp {
    fn default() -> Self {
        Self {
            config_name: String::from("Untitled"),
            elevation_aperture_angle: 18.0,
            azimuth_aperture_angle: 0.0,
            carrier_velocity: 120.0,
            carrier_height: 3000.0,
            depression_angle: 45.0,
            pri: 100.0,
            tx_offset: 0.0,
            tx_duration: 10.0,
            nb_agility: 1,
            nb_channel: 1,
            fech: 0.0,
            rx_offset: 24.0,
            rx_duration: 21.0,
            rx_noise_offset: 15.0,
            rx_noise_duration: 3.0,
            rx_reinj_offset: 20.0,
            rx_reinj_duration: 3.0,
            peak_power: 0.0,
            loss_power: 0.0,
            gain_antenna: 0.0,
            noise_factor: 0.0,
            center_frequency: 0.0,
            bandwidth: 0.0,
            retrodiff: 0.0,
            rx_gain: 0.0,
            height_ambiguity: 0.0,
            accuracy_height_ambiguity: 0.0,
        }
    }
}

fn input(ui: &mut egui::Ui, label: &str, tooltip: Option<&str>, widget: impl egui::Widget) {
    ui.horizontal(|ui| {
        ui.label(label);
        if let Some(text) = tooltip {
            let hover_text = egui::RichText::new(text)
                .color(egui::Color32::from_rgb(200, 200, 200))
                .monospace();
            ui.add(egui::Label::new("❓").sense(egui::Sense::click())).on_hover_text(hover_text);
        }
        ui.add(widget);
    });
}

impl eframe::App for SARConfApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel")
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    egui::global_theme_preference_switch(ui);
                    ui.separator();
                    ui.label("Configuration Name:");
                    ui.text_edit_singleline(&mut self.config_name);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let export_button = ui.button("Export");
                        egui::Popup::menu(&export_button)
                            .show(|ui| {
                                if ui.button("Export as JSON (.json)").clicked() {
                                    // Export as JSON logic here
                                }
                                if ui.button("Export as SAMEVA (.init)").clicked() {
                                    // Export as SAMEVA logic here
                                }
                                if ui.button("Export as GENIO (.io)").clicked() {
                                    // Export as GENIO logic here
                                }
                                if ui.button("Export as VST (.vst)").clicked() {
                                    // Export as VST logic here
                                }
                                if ui.button("Export as PAMELA (.wave)").clicked() {
                                    // Export as PAMELA logic here
                                }
                            });
                        if ui.button("Import").clicked() {
                            // Import logic here
                        }
                    });
                });
            });

        egui::SidePanel::left("left_panel")
            .show(ctx, |ui| {
                ui.heading("Transmitter Settings");
                egui::CollapsingHeader::new("Carrier")
                    .default_open(true)
                    .show(ui, |ui| {
                        input(ui,
                            "Height:",
                            Some("This is the carrier height"),
                            egui::DragValue::new(&mut self.carrier_height)
                                .fixed_decimals(3)
                                .range(0.0..=f64::NAN)
                                .suffix(" m"),
                        );
                        input(ui,
                            "Velocity:",
                            Some("This is the carrier velocity"),
                            egui::DragValue::new(&mut self.carrier_velocity)
                                .fixed_decimals(3)
                                .range(0.0..=f64::NAN)
                                .suffix(" m/s"),
                        );
                        input(ui,
                            "Depression:",
                            Some("This is the depression angle"),
                            egui::DragValue::new(&mut self.depression_angle)
                                .fixed_decimals(3)
                                .range(0.0..=90.0)
                                .suffix("°"),
                        );
                    });
                egui::CollapsingHeader::new("Antenna")
                    .default_open(true)
                    .show(ui, |ui| {
                        input(ui,
                            "Elevation:",
                            Some("This is the elevation aperture angle"),
                            egui::DragValue::new(&mut self.elevation_aperture_angle)
                                .fixed_decimals(3)
                                .range(0.0..=360.0)
                                .suffix("°"),
                        );
                        input(ui,
                            "Azimuth:",
                            Some("This is the azimuth aperture angle"),
                            egui::DragValue::new(&mut self.azimuth_aperture_angle)
                                .fixed_decimals(3)
                                .range(0.0..=360.0)
                                .suffix("°"),
                        );
                        input(ui,
                            "Gain (one-way):",
                            Some("This is the gain of the antenna"),
                            egui::DragValue::new(&mut self.gain_antenna)
                                .fixed_decimals(1)
                                .range(0.0..=f64::NAN)
                                .suffix(" dB"),
                        );
                    });
                egui::CollapsingHeader::new("System")
                    .default_open(true)
                    .show(ui, |ui| {
                        input(ui,
                            "Center Freq.:",
                            Some("This is the center frequency"),
                            egui::DragValue::new(&mut self.center_frequency)
                                .fixed_decimals(3)
                                .range(0.0..=f64::NAN)
                                .suffix(" GHz"),
                        );
                        input(ui,
                            "Bandwidth:",
                            Some("This is the bandwidth"),
                            egui::DragValue::new(&mut self.bandwidth)
                                .fixed_decimals(1)
                                .range(0.0..=f64::NAN)
                                .suffix(" MHz"),
                        );
                        input(ui,
                            "PRI:",
                            Some("This is the pulse repetition interval"),
                            egui::DragValue::new(&mut self.pri)
                                .fixed_decimals(1)
                                .range(1.0..=f64::NAN)
                                .suffix(" µs"),
                        );
                        ui.label(format!("(PRF: {:.1} Hz)", 1e6/self.pri));
                        input(ui,
                            "Pulse Offset:",
                            Some("This is the pulse offset"),
                            egui::DragValue::new(&mut self.tx_offset)
                                .range(0.0..=f64::NAN)
                                .suffix(" µs"),
                        );
                        input(ui,
                            "Pulse duration:",
                            Some("This is the pulse duration"),
                            egui::DragValue::new(&mut self.tx_duration)
                                .fixed_decimals(1)
                                .range(0.0..=f64::NAN)
                                .suffix(" µs"),
                        );
                        input(ui, 
                            "Nb Agility:",
                            Some("This is the number of agility"),
                            egui::DragValue::new(&mut self.nb_agility)
                                .range(1..=u32::MAX),
                        );
                    });
            });

        egui::SidePanel::right("right_panel")
            .show(ctx, |ui| {
                ui.heading("Receiver Settings");
                egui::CollapsingHeader::new("Reception parameters")
                    .default_open(true)
                    .show(ui, |ui| {
                        input(ui,
                            "Fech I/O:",
                            Some("This is the Fech I/O"),
                            egui::DragValue::new(&mut self.fech)
                                .range(0.0..=f64::NAN)
                                .suffix(" MHz"),
                        );
                        input(ui,
                            "Rx Offset:",
                            Some("This is the Rx offset"),
                            egui::DragValue::new(&mut self.rx_offset)
                                .range(0.0..=f64::NAN)
                                .suffix(" µs"),
                        );
                        input(ui,
                            "Rx Duration:",
                            Some("This is the Rx duration"),
                            egui::DragValue::new(&mut self.rx_duration)
                                .range(0.0..=f64::NAN)
                                .suffix(" µs"),
                        );
                        input(ui,
                            "Rx Noise Offset:",
                            Some("This is the Rx noise offset"),
                            egui::DragValue::new(&mut self.rx_noise_offset)
                                .range(0.0..=f64::NAN)
                                .suffix(" µs"),
                        );
                        input(ui,
                            "Rx Noise Duration:",
                            Some("This is the Rx noise duration"),
                            egui::DragValue::new(&mut self.rx_noise_duration)
                                .range(0.0..=f64::NAN)
                                .suffix(" µs"),
                        );
                        input(ui,
                            "Rx Reinj Offset:",
                            Some("This is the Rx reinjection offset"),
                            egui::DragValue::new(&mut self.rx_reinj_offset)
                                .range(0.0..=f64::NAN)
                                .suffix(" µs"),
                        );
                        input(ui,
                            "Rx Reinj Duration:",
                            Some("This is the Rx reinjection duration"),
                            egui::DragValue::new(&mut self.rx_reinj_duration)
                                .range(0.0..=f64::NAN)
                                .suffix(" µs"),
                        );
                        input(ui, 
                            "Nb Channel:",
                            Some("This is the number of channel"),
                            egui::DragValue::new(&mut self.nb_channel)
                                .range(1..=u32::MAX),
                        );
                    });
            });

        egui::TopBottomPanel::bottom("bottom_panel")
            .show(ctx, |ui| {
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

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Geometry");
            let min_elevation_angle = self.depression_angle - self.elevation_aperture_angle / 2.0;
            let max_elevation_angle = self.depression_angle + self.elevation_aperture_angle / 2.0;
            geometry::plot(ui,
                (0.0, self.carrier_height),
                self.depression_angle,
                Some((min_elevation_angle, max_elevation_angle))
            );
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "SARConf",
        options,
        Box::new(|_cc| Ok(Box::new(SARConfApp::default()))),
    );
}
