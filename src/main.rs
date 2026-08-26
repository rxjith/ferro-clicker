use eframe::egui;
use enigo::{Button as EnigoButton, Coordinate, Direction, Enigo, Mouse, Settings};
use evdev::{Device, Key as EvdevKey};
use std::fs;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

const MODE_CURRENT_LOCATION: u32 = 0;
const MODE_FIXED_LOCATION: u32 = 1;

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
    fn to_evdev_key(self) -> EvdevKey {
        match self {
            Hotkey::F1 => EvdevKey::KEY_F1,
            Hotkey::F2 => EvdevKey::KEY_F2,
            Hotkey::F3 => EvdevKey::KEY_F3,
            Hotkey::F4 => EvdevKey::KEY_F4,
            Hotkey::F5 => EvdevKey::KEY_F5,
            Hotkey::F6 => EvdevKey::KEY_F6,
            Hotkey::F7 => EvdevKey::KEY_F7,
            Hotkey::F8 => EvdevKey::KEY_F8,
            Hotkey::F9 => EvdevKey::KEY_F9,
            Hotkey::F10 => EvdevKey::KEY_F10,
            Hotkey::F11 => EvdevKey::KEY_F11,
            Hotkey::F12 => EvdevKey::KEY_F12,
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

    fn all() -> &'static [Hotkey] {
        &[
            Hotkey::F1, Hotkey::F2, Hotkey::F3, Hotkey::F4, Hotkey::F5, Hotkey::F6,
            Hotkey::F7, Hotkey::F8, Hotkey::F9, Hotkey::F10, Hotkey::F11, Hotkey::F12,
        ]
    }
}

fn check_input_permissions() -> bool {
    let input_dir = match fs::read_dir("/dev/input") {
        Ok(dir) => dir,
        Err(_) => return false,
    };

    let mut found_event_device = false;
    let mut can_read_device = false;

    for entry in input_dir.flatten() {
        let path = entry.path();
        if path.to_string_lossy().contains("event") {
            found_event_device = true;
            if fs::File::open(&path).is_ok() {
                can_read_device = true;
                break;
            }
        }
    }

    if !found_event_device {
        return true;
    }

    can_read_device
}

fn main() -> eframe::Result<()> {
    let is_running = Arc::new(AtomicBool::new(false));
    let interval_ms = Arc::new(AtomicU64::new(100));
    let mode = Arc::new(AtomicU32::new(MODE_CURRENT_LOCATION));
    let fixed_x = Arc::new(AtomicI32::new(500));
    let fixed_y = Arc::new(AtomicI32::new(500));
    let active_hotkey = Arc::new(AtomicU32::new(5));
    let is_picking_location = Arc::new(AtomicBool::new(false));

    // Background Thread: Clicker Engine
    {
        let running = Arc::clone(&is_running);
        let interval = Arc::clone(&interval_ms);
        let click_mode = Arc::clone(&mode);
        let target_x = Arc::clone(&fixed_x);
        let target_y = Arc::clone(&fixed_y);

        thread::spawn(move || {
            let mut enigo = match Enigo::new(&Settings::default()) {
                Ok(enigo) => enigo,
                Err(err) => {
                    eprintln!("[Enigo Init Error] Failed to initialize input handle: {:?}", err);
                    return;
                }
            };

            loop {
                if running.load(Ordering::SeqCst) {
                    let current_mode = click_mode.load(Ordering::SeqCst);

                    if current_mode == MODE_FIXED_LOCATION {
                        let x = target_x.load(Ordering::SeqCst);
                        let y = target_y.load(Ordering::SeqCst);

                        if let Err(err) = enigo.move_mouse(x, y, Coordinate::Abs) {
                            eprintln!(
                                "[Enigo Move Error] Could not move mouse to ({}, {}): {:?}",
                                x, y, err
                            );
                        }
                    }

                    if let Err(err) = enigo.button(EnigoButton::Left, Direction::Click) {
                        eprintln!("[Enigo Click Error] Click event failed: {:?}", err);
                    }

                    let delay = interval.load(Ordering::SeqCst).max(10);
                    thread::sleep(Duration::from_millis(delay));
                } else {
                    thread::sleep(Duration::from_millis(20));
                }
            }
        });
    }

    // Background Thread: Low-Level Event Listener
    {
        let running = Arc::clone(&is_running);
        let active_hk = Arc::clone(&active_hotkey);
        let picking = Arc::clone(&is_picking_location);
        let target_x = Arc::clone(&fixed_x);
        let target_y = Arc::clone(&fixed_y);
        let click_mode = Arc::clone(&mode);

        thread::spawn(move || {
            loop {
                let mut valid_devices: Vec<Device> = Vec::new();

                if let Ok(entries) = fs::read_dir("/dev/input") {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.to_string_lossy().contains("event") {
                            if let Ok(device) = Device::open(&path) {
                                valid_devices.push(device);
                            }
                        }
                    }
                }

                if valid_devices.is_empty() {
                    thread::sleep(Duration::from_secs(2));
                    continue;
                }

                let mut handles = Vec::new();

                for mut dev in valid_devices {
                    let running = Arc::clone(&running);
                    let active_hk = Arc::clone(&active_hk);
                    let picking = Arc::clone(&picking);
                    let target_x = Arc::clone(&target_x);
                    let target_y = Arc::clone(&target_y);
                    let click_mode = Arc::clone(&click_mode);

                    let handle = thread::spawn(move || loop {
                        match dev.fetch_events() {
                            Ok(events) => {
                                for ev in events {
                                    if ev.event_type() == evdev::EventType::KEY && ev.value() == 1 {
                                        let current_hk_idx = active_hk.load(Ordering::SeqCst) as usize;

                                        if let Some(target_hk) = Hotkey::all().get(current_hk_idx) {
                                            if ev.code() == target_hk.to_evdev_key().code() {
                                                let state = running.load(Ordering::SeqCst);
                                                running.store(!state, Ordering::SeqCst);
                                            }
                                        }

                                        if picking.load(Ordering::SeqCst) && ev.code() == EvdevKey::BTN_LEFT.code() {
                                            thread::sleep(Duration::from_millis(50));

                                            if let Ok(mut enigo) = Enigo::new(&Settings::default()) {
                                                match enigo.location() {
                                                    Ok((x, y)) => {
                                                        target_x.store(x, Ordering::SeqCst);
                                                        target_y.store(y, Ordering::SeqCst);
                                                        click_mode.store(MODE_FIXED_LOCATION, Ordering::SeqCst);

                                                        println!("[FerroClicker] Location captured: ({}, {})", x, y);
                                                    }
                                                    Err(err) => {
                                                        eprintln!(
                                                            "[Enigo Location Error] Failed to capture cursor position: {:?}",
                                                            err
                                                        );
                                                    }
                                                }
                                            } else {
                                                eprintln!("[Enigo Init Error] Failed to initialize coordinate picker.");
                                            }

                                            picking.store(false, Ordering::SeqCst);
                                        }
                                    }
                                }
                            }
                            Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                                thread::sleep(Duration::from_millis(10));
                            }
                            Err(_) => break,
                        }
                    });

                    handles.push(handle);
                }

                for handle in handles {
                    let _ = handle.join();
                }

                thread::sleep(Duration::from_secs(1));
            }
        });
    }

    let initial_permissions = check_input_permissions();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([340.0, 420.0])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "FerroClicker",
        options,
        Box::new(move |cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());

            Box::new(AutoClickerApp {
                is_running,
                interval_ms,
                mode,
                fixed_x,
                fixed_y,
                active_hotkey,
                is_picking_location,
                has_input_permissions: initial_permissions,
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
    is_picking_location: Arc<AtomicBool>,
    has_input_permissions: bool,
}

impl eframe::App for AutoClickerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.1);
        ctx.request_repaint_after(Duration::from_millis(50));

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("FerroClicker");
            ui.separator();

            if !self.has_input_permissions {
                egui::Frame::group(ui.style())
                    .fill(egui::Color32::from_rgb(60, 20, 20))
                    .stroke(egui::Stroke::new(1.0, egui::Color32::RED))
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            ui.label(
                                egui::RichText::new("⚠️ Missing /dev/input permissions!")
                                    .strong()
                                    .color(egui::Color32::RED),
                            );
                            ui.label(
                                egui::RichText::new(
                                    "Global hotkeys won't respond. Run:\nsudo usermod -aG input $USER",
                                )
                                .small()
                                .color(egui::Color32::LIGHT_GRAY),
                            );
                            ui.add_space(4.0);
                            if ui.button("🔄 Re-check Permissions").clicked() {
                                self.has_input_permissions = check_input_permissions();
                            }
                        });
                    });
                ui.add_space(8.0);
                ui.separator();
            }

            ui.add_space(4.0);

            ui.label(egui::RichText::new("Timing").strong());
            let mut delay = self.interval_ms.load(Ordering::SeqCst);
            ui.horizontal(|ui| {
                ui.label("Interval:");
                if ui.add(egui::Slider::new(&mut delay, 1..=1000).suffix(" ms")).changed() {
                    self.interval_ms.store(delay, Ordering::SeqCst);
                }
            });

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(4.0);

            ui.label(egui::RichText::new("Click Location").strong());
            let current_mode_val = self.mode.load(Ordering::SeqCst);
            let mut selected_mode = if current_mode_val == MODE_CURRENT_LOCATION {
                ClickMode::CurrentLocation
            } else {
                ClickMode::FixedLocation
            };

            let r1 = ui.radio_value(&mut selected_mode, ClickMode::CurrentLocation, "Current Cursor Location");
            let r2 = ui.radio_value(&mut selected_mode, ClickMode::FixedLocation, "Fixed Coordinates");

            if r1.changed() || r2.changed() {
                let new_mode_val = match selected_mode {
                    ClickMode::CurrentLocation => MODE_CURRENT_LOCATION,
                    ClickMode::FixedLocation => MODE_FIXED_LOCATION,
                };
                self.mode.store(new_mode_val, Ordering::SeqCst);
            }

            if selected_mode == ClickMode::FixedLocation {
                ui.indent("coords_indent", |ui| {
                    ui.horizontal(|ui| {
                        let mut x = self.fixed_x.load(Ordering::SeqCst);
                        let mut y = self.fixed_y.load(Ordering::SeqCst);

                        ui.label("X:");
                        if ui.add(egui::DragValue::new(&mut x).clamp_range(0..=7680)).changed() {
                            self.fixed_x.store(x, Ordering::SeqCst);
                        }

                        ui.label("Y:");
                        if ui.add(egui::DragValue::new(&mut y).clamp_range(0..=4320)).changed() {
                            self.fixed_y.store(y, Ordering::SeqCst);
                        }
                    });

                    ui.add_space(4.0);

                    let picking = self.is_picking_location.load(Ordering::SeqCst);
                    let picker_btn_text = if picking {
                        "Click anywhere to capture..."
                    } else {
                        "📍 Pick Location with Mouse"
                    };

                    let btn = egui::Button::new(
                        egui::RichText::new(picker_btn_text)
                            .small()
                            .color(if picking { egui::Color32::YELLOW } else { egui::Color32::WHITE }),
                    );

                    if ui.add(btn).clicked() {
                        self.is_picking_location.store(!picking, Ordering::SeqCst);
                    }
                });
            }

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(4.0);

            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Toggle Hotkey:").strong());

                let mut hk_idx = self.active_hotkey.load(Ordering::SeqCst) as usize;
                let current_hk = Hotkey::all()[hk_idx];

                egui::ComboBox::from_id_source("hotkey_select")
                    .selected_text(current_hk.name())
                    .show_ui(ui, |ui| {
                        for (i, hk) in Hotkey::all().iter().enumerate() {
                            if ui.selectable_value(&mut hk_idx, i, hk.name()).clicked() {
                                self.active_hotkey.store(i as u32, Ordering::SeqCst);
                            }
                        }
                    });
            });

            ui.add_space(12.0);

            let currently_running = self.is_running.load(Ordering::SeqCst);
            let hk_index = self.active_hotkey.load(Ordering::SeqCst) as usize;
            let hk_name = Hotkey::all()[hk_index].name();

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
                if ui
                    .add_sized(
                        [180.0, 32.0],
                        egui::Button::new(
                            egui::RichText::new(button_text)
                                .color(egui::Color32::WHITE)
                                .strong(),
                        )
                        .fill(button_color),
                    )
                    .clicked()
                {
                    self.is_running.store(!currently_running, Ordering::SeqCst);
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