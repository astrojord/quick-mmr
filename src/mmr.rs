#[derive(Eq, PartialEq, Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Player {
    name: String,
    mmr_history: Vec<i32>,
}

#[derive(Debug)]
pub struct Match {
    num_players: i32,
    player_ranks: Vec<(Player, i32)>, // player, rank
}

impl Player {
    pub fn new(name: String, history: Option<Vec<i32>>) -> Self {
        let initial_mmr = match history {
            Some(x) => x,
            None => vec![1000],
        };

        Self {
            name,
            mmr_history: initial_mmr,
        }
    }
}

impl Match {
    pub fn new(n: i32) -> Self {
        Self {
            num_players: n,
            player_ranks: Vec::new(),
        }
    }

    pub fn add_player(&mut self, player: &Player, rank: i32) {
        self.player_ranks.push((player.clone(), rank));
    }

    // remove player?

    pub fn update_mmr(&mut self) -> Result<Vec<(Player, i32)>, &'static str> {
        if self.num_players != self.player_ranks.len().try_into().unwrap() {
            return Err("mismatched player count and rank vec length");
        }

        let k = (32 / (self.num_players - 1)) as f64;

        let inner_copy = self.player_ranks.clone();
        for (player, rank) in &mut self.player_ranks {
            let current_mmr = match &mut player.mmr_history.last() {
                Some(x) => x,
                None => &1000,
            };

            let mut mmr_delta: i32 = 0;

            // I'm sure there's a smarter way to do this loop
            for (opponent, opponent_rank) in &inner_copy {
                if opponent == player {
                    continue;
                }

                let opponent_mmr = match &mut opponent.mmr_history.last() {
                    Some(x) => x,
                    None => &1000,
                };

                let rank_diff = opponent_rank - *rank;
                let s = match rank_diff {
                    0 => 0.5,
                    1.. => 1.0,
                    _ => 0.0,
                };

                let exp = ((opponent_mmr - current_mmr) as f64) / 400.;
                let ea = 1. / (1. + 10.0_f64.powf(exp));

                // accumulate the mmr changes against each opponent
                mmr_delta += (k * (s - ea)).round() as i32;
            }

            player.mmr_history.push(current_mmr + mmr_delta);
        }

        Ok(self.player_ranks.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_player() {
        let p = Player::new(String::from("test"), None);
        assert_eq!(p.mmr_history, vec![1000]);
    }

    #[test]
    fn create_player_with_history() {
        let h = Some(vec![1000, 1020, 1010]);
        let p = Player::new(String::from("test"), h);
        assert_eq!(p.mmr_history, vec![1000, 1020, 1010]);
    }

    #[test]
    fn create_blank_match() {
        let m = Match::new(4);
        assert_eq!(m.num_players, 4);
        assert_eq!(m.player_ranks, vec![]);
    }

    #[test]
    fn add_player() {
        let mut m = Match::new(4);
        let h = Some(vec![1000, 1020, 1010]);
        let p = Player::new(String::from("test"), h);

        m.add_player(&p, 1_i32);
        assert_eq!(m.player_ranks.last(), Some((p, 1_i32)).as_ref());
    }

    #[test]
    fn run_match_without_data() {
        let mut m = Match::new(4);
        for i in 1..=2 {
            let p = Player::new(format!("test {i}"), None);
            m.add_player(&p, i);
        }

        assert!(m.update_mmr().is_err());
    }

    #[test]
    fn run_match_2player() {
        let mut m = Match::new(2);
        for i in 1..=2 {
            let p = Player::new(format!("test {i}"), None);
            m.add_player(&p, i);
        }

        let _ = m.update_mmr();

        assert_eq!(m.player_ranks[0].0.mmr_history, [1000, 1016]);
        assert_eq!(m.player_ranks[1].0.mmr_history, [1000, 984]);
    }

    #[test]
    fn run_match_3player() {
        let mut m = Match::new(3);
        for i in 1..=3 {
            let p = Player::new(format!("test {i}"), None);
            m.add_player(&p, i);
        }

        let _ = m.update_mmr();

        assert_eq!(m.player_ranks[0].0.mmr_history, [1000, 1016]);
        assert_eq!(m.player_ranks[1].0.mmr_history, [1000, 1000]);
        assert_eq!(m.player_ranks[2].0.mmr_history, [1000, 984]);
        //println!("{:?}", m.player_ranks);
    }

    #[test]
    fn run_match_4player_tie() {
        let mut m = Match::new(4);
        for i in 1..=3 {
            let p = Player::new(format!("test {i}"), None);
            m.add_player(&p, i);
        }

        let p2 = Player::new(String::from("test tie"), None);
        m.add_player(&p2, 2_i32);

        let _ = m.update_mmr();

        assert_eq!(m.player_ranks[0].0.mmr_history, [1000, 1015]);
        assert_eq!(m.player_ranks[1].0.mmr_history, [1000, 1000]);
        assert_eq!(m.player_ranks[2].0.mmr_history, [1000, 985]);
        assert_eq!(m.player_ranks[3].0.mmr_history, [1000, 1000]);
    }
}
