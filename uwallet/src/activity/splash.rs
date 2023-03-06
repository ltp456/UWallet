#[derive(serde::Deserialize, serde::Serialize)]
pub struct SplashActivity {
    input: String,
    submit: bool,
}


impl SplashActivity {
    pub fn new() -> Self {
        SplashActivity {
            input: "abcd".to_string(),
            submit: false,
        }
    }
    pub fn on_create(&mut self, ctx: &egui::Context, _frame: &eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("UWallet");
            });

            ui.add_space(30 as f32);
            ui.separator();
            ui.add_space(3 as f32);
            ui.vertical_centered(|ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label("Password: ");
                    ui.add(egui::TextEdit::singleline(&mut self.input).hint_text("input your password"));
                });
            });
            ui.add_space(3 as f32);
            self.submit = false;
            if ui.button(egui::RichText::new("Submit").size(20 as f32).color(egui::Color32::WHITE)).clicked() {
                self.submit = true
            }
        });
    }

    pub fn get_res(&mut self) -> (String,bool) {
        let Self { input,submit } = self;
        (input.clone(), *submit)
    }
}