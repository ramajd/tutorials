#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Effect {
    pub id: usize,
    pub duration: usize,
    pub armor: isize,
    pub heal: isize,
    pub mana: isize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Spell {
    pub hp_self: isize,
    pub hp_target: isize,
    pub mp_self: isize,
    pub mp_target: isize,
    pub effect_self: Option<Effect>,
    pub effect_target: Option<Effect>,
}

impl Spell {
    pub fn mana_usage(&self) -> isize {
        if self.mp_self < 0 {
            -self.mp_self
        } else {
            0
        }
    }
}

pub const MAGIC: Spell = Spell {
    hp_self: 0,
    hp_target: -4,
    mp_self: -53,
    mp_target: 0,
    effect_self: None,
    effect_target: None,
};

pub const DRAIN: Spell = Spell {
    hp_self: 2,
    hp_target: -2,
    mp_self: -73,
    mp_target: 0,
    effect_self: None,
    effect_target: None,
};

pub const SHIELD: Spell = Spell {
    hp_self: 0,
    hp_target: 0,
    mp_self: -113,
    mp_target: 0,
    effect_self: Some(Effect {
        id: 1,
        duration: 6,
        armor: 7,
        heal: 0,
        mana: 0,
    }),
    effect_target: None,
};

pub const POISON: Spell = Spell {
    hp_self: 0,
    hp_target: 0,
    mp_self: -173,
    mp_target: 0,
    effect_target: Some(Effect {
        id: 2,
        duration: 6,
        armor: 0,
        heal: -3,
        mana: 0,
    }),
    effect_self: None,
};
pub const RECHARGE: Spell = Spell {
    hp_self: 0,
    hp_target: 0,
    mp_self: -229,
    mp_target: 0,
    effect_self: Some(Effect {
        id: 3,
        duration: 5,
        armor: 0,
        heal: 0,
        mana: 101,
    }),
    effect_target: None,
};

pub const SPELLS: [&Spell; 5] = [&MAGIC, &DRAIN, &SHIELD, &POISON, &RECHARGE];
