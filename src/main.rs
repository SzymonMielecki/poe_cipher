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
    action: Actions,
    output: String,
    input: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            action: Actions::Decrypt,
            input: ")[:9‡* 96808-76 [5)0?3?,8 *5 )[8)-".into(),
            output: "".into(),
        }
    }
}

#[derive(PartialEq)]
enum Actions {
    Encrypt,
    Decrypt,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.heading("Poe Cipher App");
                if ui
                    .selectable_value(&mut self.action, Actions::Decrypt, "Decrypt")
                    .clicked()
                    || ui
                        .selectable_value(&mut self.action, Actions::Encrypt, "Encrypt")
                        .clicked()
                {
                    self.input = "".into()
                }
                ui.text_edit_multiline(&mut self.input);
                if ui.button("Calculate").clicked() {
                    match self.action {
                        Actions::Encrypt => self.output = Poe::encrypt(&self.input),
                        Actions::Decrypt => self.output = Poe::decrypt(&self.input),
                    }
                }
                ui.label(&self.output);
            });
        });
    }
}
