use egui::{Color32, FontId, RichText};

use crate::mmr::{Match, Player};

pub struct MatchUI<'a> {
    num_players: &'a mut i32,
}
pub struct PlayerUI<'a> {
    players: &'a mut Vec<Player>,
}

impl<'a> MatchUI<'a> {
    pub fn new(num_players: &'a mut i32) -> Self {
        Self { num_players }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("match_panel")
            .resizable(false)
            .exact_width(400.0)
            .show(ctx, |ui| {
                self.ui(ui);
            });
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.label(
                RichText::new("Get MMR from Match Results")
                    .color(Color32::LIGHT_RED)
                    .font(FontId::proportional(14.0)),
            );
        });

        egui::Grid::new("match_grid")
            .striped(true)
            .min_col_width(50.0)
            .num_columns(3)
            .show(ui, |ui| {
                for row in 0..=*self.num_players {
                    if row == 0 {
                        ui.label(RichText::new("Player").strong());
                        ui.label(RichText::new("Rank").strong());
                        ui.label(RichText::new("MMR").strong());
                    } else {
                        let mut player = format!("name {row}");
                        let mut rank = row;
                        ui.add(egui::TextEdit::singleline(&mut player).hint_text("Name"));

                        ui.add(egui::DragValue::new(&mut rank).range(0..=*self.num_players));

                        let old_mmr = 1000;
                        ui.label(format!("mmr {old_mmr}"));
                    }

                    ui.end_row();
                }
            });

        ui.horizontal(|ui| {
            if ui.button("Add row").clicked() {
                *self.num_players += 1;
            }

            if ui.button("Calculate").clicked() {
                let players: Vec<(Player, i32)> = Vec::new(); // instantiate this vector with stuff from the ui

                if let Some(updated_players) = calculate_match(*self.num_players, players) {
                    // replace previous grid with grid from these players/mmrs
                    println!("{:?}", updated_players);
                }
            }
        });
    }
}

impl<'a> PlayerUI<'a> {
    pub fn new(players: &'a mut Vec<Player>) -> Self {
        Self { players }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("player_panel")
            .resizable(false)
            .exact_width(200.0)
            .show(ctx, |ui| {
                self.ui(ui);
            });
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.label(
                RichText::new("Players")
                    .color(Color32::LIGHT_RED)
                    .font(FontId::proportional(14.0)),
            );
        });

        self.players.sort_by(|a, b| {
            a.mmr_history
                .last()
                .unwrap()
                .cmp(b.mmr_history.last().unwrap())
        });

        egui::Grid::new("player_grid")
            .striped(true)
            .min_col_width(50.0)
            .num_columns(2)
            .show(ui, |ui| {
                for row in 0..=self.players.len() {
                    if row == 0 {
                        ui.label(RichText::new("Player").strong());
                        ui.label(RichText::new("MMR").strong());

                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new("Delete?").strong());
                        });
                    } else {
                        ui.label(&self.players[row - 1].name);
                        ui.label(format!("{:?}", &self.players[row - 1].mmr_history));

                        ui.vertical_centered(|ui| {
                            if ui.button(RichText::new("✖").color(Color32::RED)).clicked() {
                                self.players.remove(row - 1);
                            }
                        });
                    }

                    ui.end_row();
                }
            });
    }
}

fn calculate_match(n: i32, player_ranks: Vec<(Player, i32)>) -> Option<Vec<(Player, i32)>> {
    let mut m = Match::new(n);

    for tup in player_ranks {
        m.add_player(&tup.0, tup.1);
    }

    match m.update_mmr() {
        Ok(v) => Some(v),
        Err(..) => None,
    }
}
