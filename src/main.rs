use eframe::egui;
mod geometry;
mod chronogram;

const C: f64 = 299792458.0; // Speed of light in m/s

struct SARConfApp {
    config_name: String,
    bsar_config: bool,

    // Antenna parameters
    elevation_aperture_angle: f64,
    azimuth_aperture_angle: f64,

    // Geometry parameters
    carrier_velocity: f64,
    carrier_height: f64,
    look_angle: f64,

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
            bsar_config: false,
            elevation_aperture_angle: 18.0,
            azimuth_aperture_angle: 0.0,
            carrier_velocity: 120.0,
            carrier_height: 3000.0,
            look_angle: 45.0,
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
                        egui::Grid::new("rx_carrier_grid")
                            .num_columns(2)
                            .striped(false)
                            .spacing([20.0, 5.0])
                            .show(ui, |ui| {
                                ui.label("Height:");
                                ui.add(
                                    egui::DragValue::new(&mut self.carrier_height)
                                        .fixed_decimals(3)
                                        .range(0.0..=f64::NAN)
                                        .suffix(" m")
                                );
                                ui.end_row();
                                ui.label("");
                                ui.label(format!("{:.3} ft", self.carrier_height/0.3048));
                                ui.end_row();
                                ui.label("Velocity:");
                                ui.add(
                                    egui::DragValue::new(&mut self.carrier_velocity)
                                        .fixed_decimals(3)
                                        .range(0.0..=f64::NAN)
                                        .suffix(" m/s")
                                );
                                ui.end_row();
                                ui.label("Look Angle:");
                                ui.add(
                                    egui::Slider::new(&mut self.look_angle, 0.0..=90.0)
                                        .fixed_decimals(3)
                                        .trailing_fill(true)
                                        .drag_value_speed(1.0)
                                        .suffix("°")
                                );
                                ui.end_row();
                            });
                    });
                egui::CollapsingHeader::new("Antenna")
                    .default_open(true)
                    .show(ui, |ui| {
                        egui::Grid::new("rx_antenna_grid")
                            .num_columns(2)
                            .striped(false)
                            .spacing([20.0, 5.0])
                            .show(ui, |ui| {
                                ui.label("Elevation:");
                                ui.add(
                                    egui::Slider::new(&mut self.elevation_aperture_angle, 0.0..=360.0)
                                        .fixed_decimals(3)
                                        .trailing_fill(true)
                                        .drag_value_speed(1.0)
                                        .suffix("°")
                                );
                                ui.end_row();
                                ui.label("Azimuth:");
                                ui.add(
                                    egui::Slider::new(&mut self.azimuth_aperture_angle, 0.0..=360.0)
                                        .fixed_decimals(3)
                                        .trailing_fill(true)
                                        .drag_value_speed(1.0)
                                        .suffix("°")
                                );
                                ui.end_row();
                                ui.label("Gain (one-way):");
                                ui.add(
                                    egui::DragValue::new(&mut self.gain_antenna)
                                        .fixed_decimals(1)
                                        .range(0.0..=f64::NAN)
                                        .suffix(" dB")
                                );
                                ui.end_row();
                            });
                    });
                egui::CollapsingHeader::new("System")
                    .default_open(true)
                    .show(ui, |ui| {
                        egui::Grid::new("rx_antenna_grid")
                            .num_columns(2)
                            .striped(false)
                            .spacing([20.0, 5.0])
                            .show(ui, |ui| {
                                ui.label("Center Freq.:");
                                ui.add(
                                    egui::DragValue::new(&mut self.center_frequency)
                                        .fixed_decimals(3)
                                        .range(0.0..=f64::NAN)
                                        .suffix(" GHz")
                                );
                                ui.end_row();
                                ui.label("Bandwidth:");
                                ui.add(
                                    egui::DragValue::new(&mut self.bandwidth)
                                        .fixed_decimals(1)
                                        .range(0.0..=f64::NAN)
                                        .suffix(" MHz")
                                );
                                ui.end_row();
                                ui.label("Agility:");
                                ui.add(
                                    egui::DragValue::new(&mut self.nb_agility)
                                        .range(1..=u32::MAX)
                                );
                                ui.end_row();
                                ui.label("PRI:");
                                ui.add(
                                    egui::DragValue::new(&mut self.pri)
                                        .fixed_decimals(1)
                                        .range(1.0..=f64::NAN)
                                        .suffix(" µs")
                                );
                                ui.end_row();
                                ui.label("PRF:");
                                ui.label(format!("{:.1} Hz", 1e6/self.pri));
                                ui.end_row();
                                ui.label("Pulse Offset:");
                                ui.add(
                                    egui::DragValue::new(&mut self.tx_offset)
                                        .fixed_decimals(1)
                                        .range(0.0..=f64::NAN)
                                        .suffix(" µs")
                                );
                                ui.end_row();
                                ui.label("Pulse duration:");
                                ui.add(
                                    egui::DragValue::new(&mut self.tx_duration)
                                        .fixed_decimals(1)
                                        .range(0.0..=f64::NAN)
                                        .suffix(" µs")
                                );
                                ui.end_row();
                                ui.label("Peak Power:");
                                ui.add(
                                    egui::DragValue::new(&mut self.peak_power)
                                        .fixed_decimals(1)
                                        .range(0.0..=f64::NAN)
                                        .suffix(" W")
                                );
                                ui.end_row();
                                ui.label("Loss Power:");
                                ui.add(
                                    egui::DragValue::new(&mut self.loss_power)
                                        .fixed_decimals(1)
                                        .range(0.0..=f64::NAN)
                                        .suffix(" dB")
                                );
                                ui.end_row();
                            });
                    });
            });

        egui::SidePanel::right("right_panel")
            .show(ctx, |ui| {
                ui.heading("Receiver Settings");
                ui.add(egui::Checkbox::new(&mut self.bsar_config, "bistatic configuration"));
                egui::CollapsingHeader::new("Carrier")
                    .default_open(true)
                    .show(ui, |ui| {
                        ui.add_enabled_ui(self.bsar_config, |ui| {
                            egui::Grid::new("rx_carrier_grid")
                                .num_columns(2)
                                .striped(false)
                                .spacing([20.0, 5.0])
                                .show(ui, |ui| {
                                    ui.label("Height:");
                                    ui.add(
                                        egui::DragValue::new(&mut self.carrier_height)
                                            .fixed_decimals(3)
                                            .range(0.0..=f64::NAN)
                                            .suffix(" m")
                                    );
                                    ui.end_row();
                                    ui.label("");
                                    ui.label(format!("{:.3} ft", self.carrier_height/0.3048));
                                    ui.end_row();
                                    ui.label("Velocity:");
                                    ui.add(
                                        egui::DragValue::new(&mut self.carrier_velocity)
                                            .fixed_decimals(3)
                                            .range(0.0..=f64::NAN)
                                            .suffix(" m/s")
                                    );
                                    ui.end_row();
                                    ui.label("Look Angle:");
                                    ui.add(
                                        egui::Slider::new(&mut self.look_angle, 0.0..=90.0)
                                            .fixed_decimals(3)
                                            .trailing_fill(true)
                                            .drag_value_speed(1.0)
                                            .suffix("°")
                                    );
                                    ui.end_row();
                                });
                        });
                    });
                egui::CollapsingHeader::new("System")
                    .default_open(true)
                    .show(ui, |ui| {
                        egui::Grid::new("rx_carrier_grid")
                            .num_columns(2)
                            .striped(false)
                            .spacing([20.0, 5.0])
                            .show(ui, |ui| {
                                ui.label("Fech I/O:");
                                ui.add(
                                    egui::DragValue::new(&mut self.fech)
                                        .range(0.0..=f64::NAN)
                                        .suffix(" MHz")
                                );
                                ui.end_row();
                                ui.label("RX Offset:");
                                ui.add(
                                    egui::DragValue::new(&mut self.rx_offset)
                                        .fixed_decimals(1)
                                        .range(0.0..=f64::NAN)
                                        .suffix(" µs")
                                );
                                ui.end_row();
                                ui.label("RX Duration:");
                                ui.add(
                                    egui::DragValue::new(&mut self.rx_duration)
                                        .range(0.0..=f64::NAN)
                                        .suffix(" µs")
                                );
                                ui.end_row();
                                ui.label("RX Noise Offset:");
                                ui.add(
                                    egui::DragValue::new(&mut self.rx_noise_offset)
                                        .range(0.0..=f64::NAN)
                                        .suffix(" µs")
                                );
                                ui.end_row();
                                ui.label("RX Noise Duration:");
                                ui.add(
                                    egui::DragValue::new(&mut self.rx_noise_duration)
                                        .range(0.0..=f64::NAN)
                                        .suffix(" µs")
                                );
                                ui.end_row();
                                ui.label("RX Reinj Offset:");
                                ui.add(
                                    egui::DragValue::new(&mut self.rx_reinj_offset)
                                        .range(0.0..=f64::NAN)
                                        .suffix(" µs")
                                );
                                ui.end_row();
                                ui.label("RX Reinj Duration:");
                                ui.add(
                                    egui::DragValue::new(&mut self.rx_reinj_duration)
                                        .range(0.0..=f64::NAN)
                                        .suffix(" µs")
                                );
                                ui.end_row();
                                ui.label("Nb Channel:");
                                ui.add(
                                    egui::DragValue::new(&mut self.nb_channel)
                                        .range(1..=u32::MAX)
                                );
                                ui.end_row();
                                ui.label("Noise Factor:");
                                ui.add(
                                    egui::DragValue::new(&mut self.noise_factor)
                                        .fixed_decimals(1)
                                        .range(0.0..=f64::NAN)
                                        .suffix(" dB")
                                );
                                ui.end_row();
                            });
                    });
            });

        egui::TopBottomPanel::bottom("bottom_panel")
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.heading("Chronogram");
                    if ui.button("📷").clicked() {
                        // TODO: Save the current chronogram as a PNG file
                    }
                });
                let mut windows = vec![
                    chronogram::Window {
                        name: "TX".to_string(),
                        start_time: self.tx_offset,
                        duration: self.tx_duration,
                        height: 1.0,
                        dashed: false,
                        color: Some(egui::Color32::RED),
                    },
                    chronogram::Window {
                        name: "Nadir".to_string(),
                        start_time: self.tx_offset + self.carrier_height / C * 2e6,
                        duration: self.tx_duration,
                        height: 0.2,
                        dashed: true,
                        color: Some(egui::Color32::WHITE),
                    },
                    chronogram::Window {
                        name: "RX".to_string(),
                        start_time: self.rx_offset,
                        duration: self.rx_duration,
                        height: 1.0,
                        dashed: false,
                        color: Some(egui::Color32::LIGHT_YELLOW),
                    },
                    chronogram::Window {
                        name: "Noise".to_string(),
                        start_time: self.rx_noise_offset,
                        duration: self.rx_noise_duration,
                        height: 0.8,
                        dashed: false,
                        color: Some(egui::Color32::GOLD),
                    },
                    chronogram::Window {
                        name: "Reinj".to_string(),
                        start_time: self.rx_reinj_offset,
                        duration: self.rx_reinj_duration,
                        height: 0.8,
                        dashed: false,
                        color: Some(egui::Color32::GOLD),
                    },
                ];
                if self.rx_duration - self.tx_offset - self.tx_duration > 0.0 {
                    windows.push(
                        chronogram::Window {
                            name: "RX (full resol)".to_string(),
                            start_time: self.rx_offset,
                            duration: self.rx_duration - self.tx_offset - self.tx_duration,
                            height: 1.0,
                            dashed: true,
                            color: Some(egui::Color32::YELLOW),
                        },
                    );
                }
                chronogram::plot(ui, self.pri, windows);

                egui::Grid::new("rx_antenna_grid")
                    .num_columns(2)
                    .striped(true)
                    .spacing([20.0, 5.0])
                    .show(ui, |ui| {
                        ui.label("Final PRF:");
                        ui.label(format!("{:.1} Hz", 1e6/self.pri/self.nb_agility as f64));
                        ui.end_row();                        
                    });

            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Geometry");
                if ui.button("📷").clicked() {
                    // TODO: Save the current geometry as a PNG file
                }
            });
            let min_aperture_elevation_angle_deg = self.look_angle - self.elevation_aperture_angle / 2.0;
            let max_aperture_elevation_angle_deg = self.look_angle + self.elevation_aperture_angle / 2.0;
            let min_numerization_distance_m = 0.5e-6 * C * self.rx_offset;
            let max_numerization_distance_m = 0.5e-6 * C * (self.rx_offset + self.rx_duration - self.tx_offset - self.tx_duration); // Full resolution
            geometry::plot(ui,
                (0.0, self.carrier_height),
                self.look_angle,
                Some((min_aperture_elevation_angle_deg, max_aperture_elevation_angle_deg)),
                Some((min_numerization_distance_m, max_numerization_distance_m)),
            );

            egui::Grid::new("rx_antenna_grid")
                .num_columns(2)
                .striped(true)
                .spacing([20.0, 5.0])
                .show(ui, |ui| {
                    ui.label("Incidence:");
                    ui.label(format!("{:.1}°", 90.0 - self.look_angle));
                    ui.end_row();
                    ui.label("Target distance:");
                    ui.label(format!("{:.1} m", self.carrier_height / self.look_angle.to_radians().cos()));
                    ui.end_row();
                });
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
