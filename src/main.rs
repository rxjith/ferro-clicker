use eframe::egui;
use rdev::{simulate, Button, EventType};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() -> eframe::Result<()> {
    // Shared atomic state across threads
    let is_running = Arc::new(AtomicBool::new(false));
    let interval_ms = Arc::new(AtomicU64::new(100)); // Default 100ms delay

    // Spawn background clicker thread
    let running_clone = Arc::clone(&is_running);
    let interval_clone = Arc::clone(&interval_ms);
    thread::spawn(move || {
        loop {
            if running_clone.load(Ordering::Relaxed) {
                // Simulate Mouse Down and Mouse Up
                let _ = simulate(&EventType::ButtonPress(Button::Left));
                let _ = simulate(&EventType::ButtonRelease(Button::Left));

                let delay = interval_clone.load(Ordering::Relaxed);
                thread::sleep(Duration::from_millis(delay));
            } else {
                // Sleep longer when paused to save CPU
                thread::sleep(Duration::from_millis(50));
            }
        }
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([280.0, 160.0])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "FerroClicker",
        options,
        Box::new(|_cc| {
            Box::new(AutoClickerApp {
                is_running,
                interval_ms,
            })
        }),
    )
}

struct AutoClickerApp {
    is_running: Arc<AtomicBool>,
    interval_ms: Arc<AtomicU64>,
}

impl eframe::App for AutoClickerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("FerroClicker");
            ui.add_space(8.0);

            // Interval Slider
            let mut delay = self.interval_ms.load(Ordering::Relaxed);
            ui.horizontal(|ui| {
                ui.label("Interval (ms):");
                if ui.add(egui::Slider::new(&mut delay, 10..=1000)).changed() {
                    self.interval_ms.store(delay, Ordering::Relaxed);
                }
            });

            ui.add_space(12.0);

            // Start/Stop Toggle Button
            let currently_running = self.is_running.load(Ordering::Relaxed);
            let button_text = if currently_running { "STOP" } else { "START" };

            if ui.button(button_text).clicked() {
                self.is_running.store(!currently_running, Ordering::Relaxed);
            }

            ui.add_space(8.0);
            ui.label(format!(
                "Status: {}",
                if currently_running { "Running" } else { "Idle" }
            ));
        });
    }
}