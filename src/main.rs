use eframe::egui;
use poe_cipher::{Cipher, Poe};

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Poe Cipher App by Szymon Mielecki",
        options,
        Box::new(|_| Box::<MyApp>::default()),
    )
}

struct MyApp {
    input: String,
    output: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            input: ")[:9‡* 96808-76 [5)0?3?,8 *5 )[8)-".into(),
            output: "".into(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("Poe Cipher App");
                ui.text_edit_multiline(&mut self.input);
                ui.horizontal(|ui| {
                    if ui.button("Detect").clicked() {
                        self.output = Poe::detect(&self.input)
                    }
                    if ui.button("Decrypt").clicked() {
                        self.output = Poe::decrypt(&self.input)
                    }
                    if ui.button("Encrypt").clicked() {
                        self.output = Poe::encrypt(&self.input)
                    }
                });
                ui.label(&self.output);
            });
        });
    }
}
