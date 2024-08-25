pub struct Player {
    name: String,
    mmr: i32,
}

impl Default for Player {
    fn default() -> Player {
        Player {
            name: String::from("Player name"),
            mmr: 1000,
        }
    }
}
