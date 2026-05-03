use eframe::egui;
use egui_custom_widgets::{DigitwiseNumberEditor, DigitwiseNumberEditorAction};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Digitwise Number Editor Example",
        options,
        Box::new(|_cc| Ok(Box::<ExampleApp>::default())),
    )
}

#[derive(Default)]
struct ExampleApp {
    value: u64,
    last_action: Option<DigitwiseNumberEditorAction>,
}

impl eframe::App for ExampleApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("DigitwiseNumberEditor");
            ui.label("Click a digit, type a replacement, use arrow keys, or drag vertically.");
            ui.add_space(8.0);

            let output = DigitwiseNumberEditor::new("sample_count", &mut self.value)
                .digits(9)
                .max(999_999_999)
                .dim_leading_zeroes(true)
                .show(ui);

            if let Some(action) = output.action {
                self.last_action = Some(action);
            }

            ui.add_space(12.0);
            ui.monospace(format!("Current value: {}", self.value));
            ui.monospace(format!("Selected digit: {}", output.selected_digit));
            ui.monospace(format!(
                "Last action: {}",
                self.last_action
                    .map(|action| format!("{action:?}"))
                    .unwrap_or_else(|| "None".to_owned())
            ));
        });
    }
}
