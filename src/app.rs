use egui_commonmark::{commonmark, commonmark_str, CommonMarkCache};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    about_open: bool,
    avatar_open: bool,

    /// Cache for [egui_commonmark]
    #[serde(skip)]
    markdown_cache: CommonMarkCache,
}

impl Default for App {
    fn default() -> Self {
        Self {
            markdown_cache: Default::default(),
            about_open: true,
            avatar_open: true,
        }
    }
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        // Defaults to dark mode.
        cc.egui_ctx.set_theme(egui::Theme::Dark);
        Default::default()
    }
}

impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::widgets::global_theme_preference_switch(ui);

                ui.separator();

                if ui.button("About").clicked() {
                    self.about_open = !self.about_open
                }

                if ui.button("Avatar").clicked() {
                    self.avatar_open = !self.avatar_open
                }
            });
        });

        let about_response = egui::Window::new("About")
            .open(&mut self.about_open)
            .scroll(true)
            .show(ctx, |ui| {
                // Displays url in tooltip when hovered
                ui.style_mut().url_in_tooltip = true;

                commonmark_str!(ui, &mut self.markdown_cache, "markdown/about.md");
            });

        // Sets the avatar to be left of the about
        let mut avatar_window = egui::Window::new("Avatar");
        if let Some(response) = about_response {
            avatar_window = avatar_window.default_pos(response.response.rect.left_top());
        }
        avatar_window
            .open(&mut self.avatar_open)
            .scroll(true)
            .show(ctx, |ui| {
                // Displays url in tooltip when hovered
                ui.style_mut().url_in_tooltip = true;

                commonmark!(
                    ui,
                    &mut self.markdown_cache,
                    "Avatar sourced from my [GitHub](https://github.com/tye-exe)."
                );
                ui.image("https://avatars.githubusercontent.com/tye-exe");
            });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            egui::warn_if_debug_build(ui);

            ui.hyperlink_to("Source Code", "https://github.com/tye-exe/tye-home");

            powered_by_egui_and_eframe(ui);
        });

        // Allows the centeral panel to change with the theme
        egui::CentralPanel::default().show(ctx, |_ui| {});
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
