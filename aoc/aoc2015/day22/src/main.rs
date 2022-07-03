mod player;
mod spell;

use player::*;
use spell::*;

fn main() {
    let player = Player::new(50, 500, 0);
    let boss = Player::new(71, 0, 10);
    let mut min_mana = isize::max_value();
    simulate_battle(&player, &boss, &mut min_mana, 0, false);
    println!("optimal mana: {}", min_mana);

    let mut min_mp = isize::max_value();
    simulate_battle(&player, &boss, &mut min_mp, 0, true);
    println!("optimal mana (hard): {}", min_mp);
}

fn simulate_battle(
    player: &Player,
    boss: &Player,
    min_mana: &mut isize,
    used_mana: isize,
    hard: bool,
) {
    for spell in SPELLS.iter() {
        let mut player = player.clone();
        let mut boss = boss.clone();

        if hard {
            player.suffer(1);
            if player.is_dead() {
                continue;
            }
        }

        player.apply_effects();
        boss.apply_effects();
        if player.is_dead() {
            continue;
        }
        if boss.is_dead() {
            if used_mana < *min_mana {
                *min_mana = used_mana
            };
            continue;
        }

        if !player.can_cast(&boss, &spell) {
            continue;
        }
        if used_mana + spell.mana_usage() >= *min_mana {
            continue;
        }

        let mp_used = player.cast(&mut boss, spell);
        if player.is_dead() {
            continue;
        }
        if boss.is_dead() {
            if used_mana + mp_used < *min_mana {
                *min_mana = used_mana + mp_used
            };
            continue;
        }

        player.apply_effects();
        boss.apply_effects();
        if player.is_dead() {
            continue;
        }
        if boss.is_dead() {
            if used_mana + mp_used < *min_mana {
                *min_mana = used_mana + mp_used
            };
            continue;
        }

        boss.attack(&mut player);
        if player.is_dead() {
            continue;
        }
        if boss.is_dead() {
            if used_mana + mp_used < *min_mana {
                *min_mana = used_mana + mp_used
            };
            continue;
        }

        simulate_battle(&player, &boss, min_mana, used_mana + mp_used, hard);
    }
}
