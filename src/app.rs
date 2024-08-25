#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct MMRApp {
    // Example stuff:
    label: String,

    num_players: i32,
    // #[serde(skip)] // This how you opt-out of serialization of a field
    // value: f32,
}

impl Default for MMRApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            num_players: 4,
            // value: 2.7,
        }
    }
}

impl MMRApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for MMRApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                }

                ui.menu_button("Players", |ui| {
                    if ui.button("Add").clicked() {}
                    if ui.button("Delete").clicked() {}
                });
                ui.add_space(400.0);
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // ui.heading("quick multiplayer MMR");

            // ui.horizontal(|ui| {
            //     ui.label(" ");
            // });

            ui.label("Enter the players and their rankings for a match, then click Calculate to show their new MMR.");
            ui.label("If a player didn't participate in the match, leave their rank empty.");
            ui.label("Use the Players menu up top to add and delete players.");

            // ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            // if ui.button("Increment").clicked() {
            //     self.value += 1.0;
            // }

            ui.separator();

            egui::Grid::new("my_grid")
            .striped(true)
            .min_col_width(100.0)
            .num_columns(3)
            .show(ui, |ui| {
                for row in 0..=self.num_players {
                    if row == 0 {
                        ui.label("Player");
                        ui.label("Match rank");
                        ui.label("MMR");
                    } else {
                        let mut player = format!("name {row}");
                        let mut rank = row;
                        ui.add(egui::TextEdit::singleline(&mut player).hint_text("Name"));

                        ui.add(
                            egui::DragValue::new(&mut rank)
                            .speed(1.)
                            .range(1..=self.num_players)
                        );

                        let old_mmr = 1000;
                        ui.label(format!("mmr {old_mmr}"));
                    }

                    ui.end_row();
                }
            });

            if ui.button("Calculate").clicked() {
                calculate_match();
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                footer_links(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn footer_links(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(" | ");
        ui.hyperlink_to("Source", "https://github.com/astrojord/quick-mmr/");
    });
}

fn calculate_match() {}
