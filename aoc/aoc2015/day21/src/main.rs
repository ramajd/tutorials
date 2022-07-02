use std::cmp;

#[derive(Debug)]
struct Player {
    hp: i32,
    c: i32,
    d: i32,
    a: i32,
}

impl Player {
    fn new(hp: i32, c: i32, d: i32, a: i32) -> Self {
        Self { hp, c, d, a }
    }
}

// Cost  Damage  Armor
type Instrument = (i32, i32, i32);

fn main() {
    let mut weapons = Vec::new();
    weapons.push((8, 4, 0));
    weapons.push((10, 5, 0));
    weapons.push((25, 6, 0));
    weapons.push((40, 7, 0));
    weapons.push((74, 8, 0));

    let mut armors = Vec::new();
    armors.push((13, 0, 1));
    armors.push((31, 0, 2));
    armors.push((53, 0, 3));
    armors.push((75, 0, 4));
    armors.push((102, 0, 5));
    armors.push((0, 0, 0)); // not wearing armor

    let mut rings = Vec::new();
    rings.push((25, 1, 0));
    rings.push((50, 2, 0));
    rings.push((100, 3, 0));
    rings.push((20, 0, 1));
    rings.push((40, 0, 2));
    rings.push((80, 0, 3));
    rings.push((0, 0, 0)); // not wearing ring L
    rings.push((0, 0, 0)); // not wearing ring R

    let boss = Player::new(109, 0, 8, 2);
    let (min_win, max_loose) = calculate_minimum_cost(100, &weapons, &armors, &rings, &boss);
    println!("Min cost to win = {}", min_win);
    println!("Max cost and still loose = {}", max_loose);
}

fn fight(player: &Player, boss: &Player) -> bool {
    let mut player_hp = player.hp;
    let mut boss_hp = boss.hp;
    loop {
        boss_hp -= cmp::max(1, player.d - boss.a);
        if boss_hp <= 0 {
            return true;
        }
        player_hp -= cmp::max(1, boss.d - player.a);
        if player_hp <= 0 {
            return false;
        }
    }
}

fn calculate_minimum_cost(
    hp: i32,
    weapons: &Vec<Instrument>,
    armors: &Vec<Instrument>,
    rings: &Vec<Instrument>,
    boss: &Player,
) -> (i32, i32) {
    let mut min_to_win = std::i32::MAX;
    let mut max_to_loose = std::i32::MIN;

    for (wc, wd, _) in weapons {
        for (ac, _, aa) in armors {
            for (i1, (r1c, r1d, r1a)) in rings[..rings.len() - 1].iter().enumerate() {
                for (r2c, r2d, r2a) in rings[i1 + 1..].iter() {
                    let player =
                        Player::new(hp, wc + ac + r1c + r2c, wd + r1d + r2d, aa + r1a + r2a);
                    if fight(&player, boss) {
                        if min_to_win > player.c {
                            min_to_win = player.c;
                        }
                    } else {
                        if max_to_loose < player.c {
                            max_to_loose = player.c;
                        }
                    }
                }
            }
        }
    }
    (min_to_win, max_to_loose)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fight() {
        let boss = Player::new(12, 0, 7, 2);
        let player = Player::new(8, 0, 5, 5);
        assert_eq!(fight(&player, &boss), true);

        let boss = Player::new(109, 0, 8, 2);

        let player = Player::new(100, 0, 7, 3);
        println!("{:?}", player);
        assert_eq!(fight(&player, &boss), false);

        let player = Player::new(100, 0, 7, 4);
        assert_eq!(fight(&player, &boss), true);
    }
}
