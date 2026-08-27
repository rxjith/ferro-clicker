use eframe::egui;
use enigo::{Button as EnigoButton, Direction, Enigo, Mouse, Settings};
use evdev::{Device, Key as EvdevKey};
use std::fs;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

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
    let startup_delay_sec = Arc::new(AtomicU64::new(3)); // Default 3 sec delay
    let countdown_remaining = Arc::new(AtomicU64::new(0));
    let active_hotkey = Arc::new(AtomicU32::new(5));

    // Background Thread: Clicker Engine with Delay Handler
    {
        let running = Arc::clone(&is_running);
        let interval = Arc::clone(&interval_ms);
        let start_delay = Arc::clone(&startup_delay_sec);
        let countdown = Arc::clone(&countdown_remaining);

        thread::spawn(move || {
            let mut enigo = match Enigo::new(&Settings::default()) {
                Ok(enigo) => enigo,
                Err(err) => {
                    eprintln!("[Enigo Init Error] Failed to initialize input handle: {:?}", err);
                    return;
                }
            };

            let mut was_running = false;

            loop {
                let currently_running = running.load(Ordering::SeqCst);

                // Detect transition from STOPPED -> RUNNING
                if currently_running && !was_running {
                    let delay_secs = start_delay.load(Ordering::SeqCst);

                    if delay_secs > 0 {
                        let start_time = Instant::now();
                        let total_duration = Duration::from_secs(delay_secs);

                        while start_time.elapsed() < total_duration {
                            // If user toggled it OFF during countdown, abort!
                            if !running.load(Ordering::SeqCst) {
                                countdown.store(0, Ordering::SeqCst);
                                break;
                            }

                            let remaining = total_duration
                                .checked_sub(start_time.elapsed())
                                .unwrap_or(Duration::ZERO);

                            // Ceiling division so it shows "3... 2... 1..." instead of starting at "2"
                            let remaining_secs = remaining.as_secs_f64().ceil() as u64;
                            countdown.store(remaining_secs, Ordering::SeqCst);

                            thread::sleep(Duration::from_millis(50));
                        }
                    }
                    countdown.store(0, Ordering::SeqCst);
                }

                was_running = running.load(Ordering::SeqCst);

                if was_running {
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
            .with_inner_size([300.0, 310.0])
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
                startup_delay_sec,
                countdown_remaining,
                active_hotkey,
                has_input_permissions: initial_permissions,
            })
        }),
    )
}

struct AutoClickerApp {
    is_running: Arc<AtomicBool>,
    interval_ms: Arc<AtomicU64>,
    startup_delay_sec: Arc<AtomicU64>,
    countdown_remaining: Arc<AtomicU64>,
    active_hotkey: Arc<AtomicU32>,
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

            // Click Interval Slider
            let mut delay = self.interval_ms.load(Ordering::SeqCst);
            ui.horizontal(|ui| {
                ui.label("Interval:");
                if ui.add(egui::Slider::new(&mut delay, 1..=1000).suffix(" ms")).changed() {
                    self.interval_ms.store(delay, Ordering::SeqCst);
                }
            });

            // Start Delay Slider
            let mut start_delay = self.startup_delay_sec.load(Ordering::SeqCst);
            ui.horizontal(|ui| {
                ui.label("Start Delay:");
                if ui.add(egui::Slider::new(&mut start_delay, 0..=10).suffix("s")).changed() {
                    self.startup_delay_sec.store(start_delay, Ordering::SeqCst);
                }
            });

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
            let remaining_cd = self.countdown_remaining.load(Ordering::SeqCst);
            let hk_index = self.active_hotkey.load(Ordering::SeqCst) as usize;
            let hk_name = Hotkey::all()[hk_index].name();

            let (button_color, button_text, status_text) = if currently_running {
                if remaining_cd > 0 {
                    (
                        egui::Color32::from_rgb(200, 120, 20),
                        format!("CANCEL ({})", hk_name),
                        format!("STARTING IN {}s...", remaining_cd),
                    )
                } else {
                    (
                        egui::Color32::from_rgb(180, 40, 40),
                        format!("STOP ({})", hk_name),
                        "ACTIVE".to_string(),
                    )
                }
            } else {
                (
                    egui::Color32::from_rgb(40, 140, 40),
                    format!("START ({})", hk_name),
                    "PAUSED".to_string(),
                )
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
                ui.label(format!("Status: {}", status_text));
            });
        });
    }
}