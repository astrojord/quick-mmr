use crate::mmr::Player;
use crate::panels::{MatchUI, PlayerUI};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct MMRApp {
    pub num_players: i32,
    pub players: Vec<Player>,
    // #[serde(skip)] // This how you opt-out of serialization of a field
    // value: f32,
}

impl Default for MMRApp {
    fn default() -> Self {
        let mut v = Vec::new();
        for i in 1..=4 {
            let p = Player::new(format!("Player {i}"), None);
            v.push(p);
        }

        Self {
            num_players: 4,
            players: v,
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
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
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

                ui.add_space(450.0);
                egui::widgets::global_dark_light_mode_buttons(ui);
            });

            ui.separator();
            ui.label("Enter the players and their rankings for a match, then click Calculate to show their new MMR.");
            ui.label("If a player didn't participate in the match, leave their rank empty.");
            ui.label("Use the Players window to view, add, and delete players. Click a player to view MMR history.");
        });

        egui::TopBottomPanel::bottom("footer_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                footer_links(ui);
                egui::warn_if_debug_build(ui);
            });
        });

        // left panel for inputting match results
        let mut mui = MatchUI::new(&mut self.num_players);
        mui.show(ctx);

        // right panel to show persisted player data
        let mut pui = PlayerUI::new(&mut self.players);
        pui.show(ctx);
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
