use eframe::egui;
use rdev::{listen, simulate, Button, Event, EventType, Key};
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(PartialEq, Clone, Copy)]
enum ClickMode {
    CurrentLocation,
    FixedLocation,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Hotkey {
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
}

impl Hotkey {
    fn to_rdev_key(self) -> Key {
        match self {
            Hotkey::F1 => Key::F1, Hotkey::F2 => Key::F2, Hotkey::F3 => Key::F3,
            Hotkey::F4 => Key::F4, Hotkey::F5 => Key::F5, Hotkey::F6 => Key::F6,
            Hotkey::F7 => Key::F7, Hotkey::F8 => Key::F8, Hotkey::F9 => Key::F9,
            Hotkey::F10 => Key::F10, Hotkey::F11 => Key::F11, Hotkey::F12 => Key::F12,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Hotkey::F1 => "F1", Hotkey::F2 => "F2", Hotkey::F3 => "F3",
            Hotkey::F4 => "F4", Hotkey::F5 => "F5", Hotkey::F6 => "F6",
            Hotkey::F7 => "F7", Hotkey::F8 => "F8", Hotkey::F9 => "F9",
            Hotkey::F10 => "F10", Hotkey::F11 => "F11", Hotkey::F12 => "F12",
        }
    }

    fn ALL() -> &'static [Hotkey] {
        &[
            Hotkey::F1, Hotkey::F2, Hotkey::F3, Hotkey::F4, Hotkey::F5, Hotkey::F6,
            Hotkey::F7, Hotkey::F8, Hotkey::F9, Hotkey::F10, Hotkey::F11, Hotkey::F12,
        ]
    }
}

fn main() -> eframe::Result<()> {
    let is_running = Arc::new(AtomicBool::new(false));
    let interval_ms = Arc::new(AtomicU64::new(100));
    let mode = Arc::new(AtomicU32::new(0)); 
    let fixed_x = Arc::new(AtomicI32::new(500));
    let fixed_y = Arc::new(AtomicI32::new(500));
    let active_hotkey = Arc::new(AtomicU32::new(5)); // Default F6

    // 1. Background Thread: Click Loop
    {
        let running = Arc::clone(&is_running);
        let interval = Arc::clone(&interval_ms);
        let click_mode = Arc::clone(&mode);
        let target_x = Arc::clone(&fixed_x);
        let target_y = Arc::clone(&fixed_y);

        thread::spawn(move || loop {
            if running.load(Ordering::Relaxed) {
                if click_mode.load(Ordering::Relaxed) == 1 {
                    let x = target_x.load(Ordering::Relaxed) as f64;
                    let y = target_y.load(Ordering::Relaxed) as f64;
                    let _ = simulate(&EventType::MouseMove { x, y });
                }

                let _ = simulate(&EventType::ButtonPress(Button::Left));
                let _ = simulate(&EventType::ButtonRelease(Button::Left));

                let delay = interval.load(Ordering::Relaxed);
                thread::sleep(Duration::from_millis(delay));
            } else {
                thread::sleep(Duration::from_millis(50));
            }
        });
    }

    // 2. Background Thread: Global Hotkey Listener
    {
        let running = Arc::clone(&is_running);
        let active_hk = Arc::clone(&active_hotkey);

        thread::spawn(move || {
            let callback = move |event: Event| {
                if let EventType::KeyPress(key) = event.event_type {
                    let current_hk_idx = active_hk.load(Ordering::Relaxed) as usize;
                    if let Some(target_hk) = Hotkey::ALL().get(current_hk_idx) {
                        if key == target_hk.to_rdev_key() {
                            let state = running.load(Ordering::Relaxed);
                            running.store(!state, Ordering::Relaxed);
                            println!("[Hotkey] Toggled state to: {}", !state);
                        }
                    }
                }
            };

            if let Err(error) = listen(callback) {
                eprintln!("[Hotkey Error] Failed to listen to global inputs: {:?}", error);
                eprintln!("[Hotkey Error] Ensure your user is in the 'input' group: sudo usermod -aG input $USER");
            }
        });
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([340.0, 320.0])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "FerroClicker",
        options,
        Box::new(|cc| {
            // Explicitly force dark theme across all platforms
            cc.egui_ctx.set_visuals(egui::Visuals::dark());

            Box::new(AutoClickerApp {
                is_running,
                interval_ms,
                mode,
                fixed_x,
                fixed_y,
                active_hotkey,
            })
        }),
    )
}

struct AutoClickerApp {
    is_running: Arc<AtomicBool>,
    interval_ms: Arc<AtomicU64>,
    mode: Arc<AtomicU32>,
    fixed_x: Arc<AtomicI32>,
    fixed_y: Arc<AtomicI32>,
    active_hotkey: Arc<AtomicU32>,
}

impl eframe::App for AutoClickerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.1);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("FerroClicker");
            ui.separator();

            // Timing Settings
            ui.add_space(4.0);
            ui.label(egui::RichText::new("Timing").strong());
            let mut delay = self.interval_ms.load(Ordering::Relaxed);
            ui.horizontal(|ui| {
                ui.label("Interval:");
                if ui.add(egui::Slider::new(&mut delay, 1..=1000).suffix(" ms")).changed() {
                    self.interval_ms.store(delay, Ordering::Relaxed);
                }
            });

            ui.add_space(8.0);
            ui.separator();

            // Location Settings
            ui.add_space(4.0);
            ui.label(egui::RichText::new("Click Location").strong());
            
            let current_mode_val = self.mode.load(Ordering::Relaxed);
            let mut selected_mode = if current_mode_val == 0 {
                ClickMode::CurrentLocation
            } else {
                ClickMode::FixedLocation
            };

            ui.radio_value(&mut selected_mode, ClickMode::CurrentLocation, "Current Cursor Location");
            ui.radio_value(&mut selected_mode, ClickMode::FixedLocation, "Fixed Coordinates");

            if selected_mode == ClickMode::FixedLocation {
                ui.indent("coords_indent", |ui| {
                    ui.horizontal(|ui| {
                        let mut x = self.fixed_x.load(Ordering::Relaxed);
                        let mut y = self.fixed_y.load(Ordering::Relaxed);

                        ui.label("X:");
                        if ui.add(egui::DragValue::new(&mut x).clamp_range(0..=7680)).changed() {
                            self.fixed_x.store(x, Ordering::Relaxed);
                        }

                        ui.label("Y:");
                        if ui.add(egui::DragValue::new(&mut y).clamp_range(0..=4320)).changed() {
                            self.fixed_y.store(y, Ordering::Relaxed);
                        }
                    });
                });
            }
            self.mode.store(selected_mode as u32, Ordering::Relaxed);

            ui.add_space(8.0);
            ui.separator();

            // Hotkey Configuration
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Toggle Hotkey:").strong());
                
                let mut hk_idx = self.active_hotkey.load(Ordering::Relaxed) as usize;
                let current_hk = Hotkey::ALL()[hk_idx];

                egui::ComboBox::from_id_source("hotkey_select")
                    .selected_text(current_hk.name())
                    .show_ui(ui, |ui| {
                        for (i, hk) in Hotkey::ALL().iter().enumerate() {
                            if ui.selectable_value(&mut hk_idx, i, hk.name()).clicked() {
                                self.active_hotkey.store(i as u32, Ordering::Relaxed);
                            }
                        }
                    });
            });

            ui.add_space(12.0);

            // Controls & Status
            let currently_running = self.is_running.load(Ordering::Relaxed);
            let hk_name = Hotkey::ALL()[self.active_hotkey.load(Ordering::Relaxed) as usize].name();

            let button_color = if currently_running {
                egui::Color32::from_rgb(180, 40, 40)
            } else {
                egui::Color32::from_rgb(40, 140, 40)
            };

            let button_text = if currently_running {
                format!("STOP ({})", hk_name)
            } else {
                format!("START ({})", hk_name)
            };

            ui.vertical_centered(|ui| {
                if ui.add_sized([180.0, 32.0], egui::Button::new(
                    egui::RichText::new(button_text).color(egui::Color32::WHITE).strong()
                ).fill(button_color)).clicked() {
                    self.is_running.store(!currently_running, Ordering::Relaxed);
                }

                ui.add_space(6.0);
                ui.label(format!(
                    "Status: {}",
                    if currently_running { "ACTIVE" } else { "PAUSED" }
                ));
            });
        });
    }
}