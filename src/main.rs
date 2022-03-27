use eframe::{
    egui::{CentralPanel, Visuals},
    epi::{App, Storage},
    run_native, NativeOptions,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Dailies {
    daily_info: Vec<DailyInfo>,
    to_add: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DailyInfo {
    done: bool,
    name: String,
}

impl Dailies {
    fn new() -> Self {
        Self {
            daily_info: Vec::new(),
            to_add: String::new(),
        }
    }
}

impl App for Dailies {
    fn name(&self) -> &str {
        "dailies"
    }

    fn setup(
        &mut self,
        ctx: &eframe::egui::Context,
        _frame: &eframe::epi::Frame,
        storage: Option<&dyn eframe::epi::Storage>,
    ) {
        if let Some(storage) = storage {
            if let Some(value) = storage.get_string("main") {
                let app_state = serde_json::from_str(value.as_str()).unwrap();
                *self = app_state;
            }
        }
        ctx.set_visuals(Visuals::dark());
    }

    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &eframe::epi::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            // Button to add new dailies
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.to_add);
                if ui.button("Add daily").clicked() {
                    self.daily_info.push(DailyInfo {
                        done: false,
                        name: self.to_add.clone(),
                    });
                    self.to_add.clear();
                }
            });
            // Button to reset dailies
            if ui.button("Reset for today").clicked() {
                for d in &mut self.daily_info {
                    d.done = false
                }
            }
            // Buttons to delete dailies
            let mut to_delete = None;
            for (i, d) in self.daily_info.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut d.done, d.name.clone());
                    if ui.button("delete").clicked() {
                        to_delete = Some(i);
                    }
                });
            }
            if let Some(i) = to_delete {
                self.daily_info.remove(i);
            }
        });
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        storage.set_string("main", serde_json::to_string(self).unwrap());
    }
}

fn main() {
    let app = Dailies::new();
    run_native(Box::new(app), NativeOptions::default());
}
