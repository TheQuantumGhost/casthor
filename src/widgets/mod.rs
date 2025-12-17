#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MessageList {
    #[serde(skip)]
    textbox: String,
    list: Vec<String>,
}

pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui);
}

impl View for MessageList {
    fn ui(&mut self, ui: &mut egui::Ui) {
        //let mut scroll = ui.ctx().style().spacing.scroll;
        //scroll.ui(ui);

        //ui.ctx().all_styles_mut(|s| s.spacing.scroll = scroll);

        //ui.separator();

        let text_bar = egui::TextEdit::singleline(&mut self.textbox)
            .hint_text("query...")
            .show(ui);
        if text_bar.response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            self.list.push(self.textbox.clone());
            self.textbox.clear();
        }
        ui.separator();

        egui::ScrollArea::vertical()
            .auto_shrink(false)
            .show(ui, |ui| {
                for entry in self.list.iter().rev() {
                    ui.label(entry);
                }
            });
    }
}
