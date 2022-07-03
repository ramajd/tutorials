use crate::spell::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    hp: isize,
    mana: isize,
    pub damage: isize,
    effects: Vec<Effect>,
}

impl Player {
    pub fn new(hp: isize, mana: isize, damage: isize) -> Player {
        Player {
            hp,
            mana,
            damage,
            effects: vec![],
        }
    }

    pub fn is_dead(&self) -> bool {
        self.hp <= 0
    }

    pub fn suffer(&mut self, dmg: isize) {
        if dmg != 0 {
            self.hp -= dmg;
        }
    }

    fn drain(&mut self, mn: isize) {
        if mn != 0 {
            self.mana -= mn;
        }
    }

    fn has_effect(&self, effect: &Effect) -> bool {
        self.effects.iter().any(|e| e.id == effect.id)
    }

    fn add_effect(&mut self, effect: Effect) {
        self.effects.push(effect);
    }

    pub fn apply_effects(&mut self) {
        let mut hpt = 0;
        let mut mpt = 0;
        for effect in &mut self.effects {
            hpt += effect.heal;
            mpt += effect.mana;
            effect.duration -= 1;
        }
        self.suffer(-hpt);
        self.drain(-mpt);
        self.effects.retain(|e| e.duration > 0);
    }

    fn armor(&self) -> isize {
        self.effects.iter().map(|e| e.armor).sum()
    }

    pub fn attack(&mut self, target: &mut Player) {
        let damage = self.damage - target.armor();
        target.suffer(if damage > 0 { damage } else { 1 });
    }

    pub fn can_cast(&self, target: &Player, spell: &Spell) -> bool {
        spell.mana_usage() <= self.mana
            && (spell.effect_self.is_none()
                || !self.has_effect(spell.effect_self.as_ref().unwrap()))
            && (spell.effect_target.is_none()
                || !target.has_effect(spell.effect_target.as_ref().unwrap()))
    }

    pub fn cast(&mut self, target: &mut Player, spell: &Spell) -> isize {
        if !self.can_cast(target, spell) {
            return 0;
        }
        self.suffer(-spell.hp_self);
        target.suffer(-spell.hp_target);
        self.drain(-spell.mp_self);
        target.drain(-spell.mp_target);
        if let Some(ref effect) = spell.effect_self {
            self.add_effect(effect.clone());
        }
        if let Some(ref effect) = spell.effect_target {
            target.add_effect(effect.clone());
        }
        spell.mana_usage()
    }
}
