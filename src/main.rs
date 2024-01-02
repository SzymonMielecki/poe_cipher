use eframe::egui;
use poe_cipher::{Cipher, Poe};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Box::<MyApp>::default()
        }),
    )
}

struct MyApp {
    plain_text: String,
    cipher_text: String,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            cipher_text: ")[:9‡* 96808-76 [5)0?3?,8 *5 )[8)-".into(),
            plain_text: "".into(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Poe Cipher App");
            ui.vertical(|ui| {
                let name_label = ui.label("Plain Text:");
                ui.text_edit_multiline(&mut self.plain_text)
                    .labelled_by(name_label.id);
            });
            if ui.button("Turn Plain Text To Cipher").clicked() {
                self.cipher_text = Poe::encrypt(&self.plain_text)
            }
            ui.vertical(|ui| {
                let name_label = ui.label("Cipher Text:");
                ui.text_edit_multiline(&mut self.cipher_text)
                    .labelled_by(name_label.id);
            });
            if ui.button("Turn Cipher Text To Plain").clicked() {
                self.plain_text = Poe::decrypt(&self.cipher_text)
            }
        });
    }
}
