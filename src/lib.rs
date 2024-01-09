pub mod poe {
    use std::collections::HashMap;
    pub struct Poe;

    pub trait Cipher {
        fn encrypt(data: &str) -> String;
        fn decrypt(data: &str) -> String;
    }

    impl Cipher for Poe {
        fn encrypt(data: &str) -> String {
            let hm: HashMap<char, char> = "abcdefghijklmnopqrstuvwxyz"
                .chars()
                .into_iter()
                .zip("52-†81346,709*‡.$();?¶]¢:[".chars().into_iter())
                .collect();
            data.to_lowercase()
                .chars()
                .map(|x| hm.get(&x).unwrap_or(&' '))
                .collect()
        }
        fn decrypt(data: &str) -> String {
            let hm: HashMap<char, char> = "52-†81346,709*‡.$();?¶]¢:["
                .chars()
                .into_iter()
                .zip("abcdefghijklmnopqrstuvwxyz".chars().into_iter())
                .collect();
            data.to_lowercase()
                .chars()
                .map(|x| hm.get(&x).unwrap_or(&' '))
                .collect()
        }
    }
}

pub mod switcher {
    pub fn toggle_ui(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
        let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);

        let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

        if response.clicked() {
            *on = !*on;
            response.mark_changed();
        }

        response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, ""));

        if ui.is_rect_visible(rect) {
            let how_on = ui.ctx().animate_bool(response.id, *on);
            let visuals = ui.style().interact_selectable(&response, *on);
            let rect = rect.expand(visuals.expansion);
            let radius = 0.5 * rect.height();
            ui.painter()
                .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
            let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
            let center = egui::pos2(circle_x, rect.center().y);
            ui.painter()
                .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
        }

        response
    }

    fn toggle_ui_compact(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
        let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
        let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
        if response.clicked() {
            *on = !*on;
            response.mark_changed();
        }
        response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, ""));

        if ui.is_rect_visible(rect) {
            let how_on = ui.ctx().animate_bool(response.id, *on);
            let visuals = ui.style().interact_selectable(&response, *on);
            let rect = rect.expand(visuals.expansion);
            let radius = 0.5 * rect.height();
            ui.painter()
                .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
            let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
            let center = egui::pos2(circle_x, rect.center().y);
            ui.painter()
                .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
        }

        response
    }

    pub fn toggle(on: &mut bool) -> impl egui::Widget + '_ {
        move |ui: &mut egui::Ui| toggle_ui(ui, on)
    }

    pub fn url_to_file_source_code() -> String {
        format!("https://github.com/emilk/egui/blob/master/{}", file!())
    }
}
