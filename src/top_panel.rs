use crate::add_password_form::AddPasswordForm;
use magic_crypt::MagicCrypt256;

#[derive(Default)]
pub struct TopPanel {
    /// Form for adding password.
    add_password_form: AddPasswordForm,
    /// Boolean to show or hide AddPasswordForm.
    add_password_form_open: bool,
}

impl TopPanel {
    pub fn show(&mut self, ctx: &egui::Context, mc: &MagicCrypt256) -> bool {
        let mut logout = false;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Add password button
                if ui.button("Add").clicked() {
                    // Open password form
                    self.add_password_form_open = true;
                };

                // Logout button
                if ui.button("Logout").clicked() {
                    logout = true;
                };
            });
        });

        self.add_password_form
            .show_window(ctx, &mut self.add_password_form_open, mc);

        logout
    }
}
