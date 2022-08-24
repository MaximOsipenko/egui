#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui;
use std::sync::mpsc;
use std::thread::JoinHandle;
use egui::widgets::Spinner;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::new())),
    );
}

struct TestPanel {
    title: String,
    name: String,
    age: u32,
}

impl TestPanel {
    fn show(&mut self, ctx: &egui::Context) {
        let _prof_guard = tracy_client::span!("TestPanel::show");
        egui::Window::new(&self.title).show(ctx, |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);

                use eframe::egui::Widget;
                Spinner::new().ui(ui); // to enforce stable fps
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }

    fn new(name: &str, age: u32, id: usize) -> Self {
        let name = name.into();
        let title = format!("{}'s test panel {}", name, id);
        Self { title, name, age }
    }
}


type Worker = Box<dyn FnMut(&egui::Context) -> ()>;
fn new_worker(id: usize) -> Worker {
    let mut panels = [
        TestPanel::new("Bob", 42 + id as u32, id),
        TestPanel::new("Alice", 15 - id as u32, id),
        TestPanel::new("Cris", 10 * id as u32, id),
    ];

    Box::new(move |ctx| {
        for panel in &mut panels {
            panel.show(&ctx)
        }
    })
}


struct MyApp {
    workers: Vec<Worker>
}

impl MyApp {
    fn new() -> Self {
        let workers = Vec::with_capacity(3);

        Self {
            workers,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("add worker").clicked() {
                let id = self.workers.len();
                self.workers.push(new_worker(id))
            }
        });

        for func in &mut self.workers {
            (func)(ctx);
        }
    }
}
