use std::{ cmp::Ordering, io };
use rand::prelude::{ Rng, thread_rng, };
use crate::{
    print_flush,
    println_flush,
    engine::hero::{ Hero, HeroKind },
};

pub const INIT_CROWN: u8 = 10;
pub const MAX_CROWN: u8 = 12;
pub const INIT_BULWARK: u8 = 0;
pub const MAX_BULWARK: u8 = 5;

#[derive(Clone, Debug)]
pub struct Player {
    name: String,
    crown: u8,
    bulwark: u8,
    hero_l: Hero,
    hero_r: Hero,
}

#[derive(Copy, Clone, Debug)]
pub enum HeroPos { L, R }

impl HeroPos {
    pub fn other(&self) -> Self {
        match *self {
            Self::L => Self::R,
            Self::R => Self::L,
        }
    }
}

impl Player {
    pub(crate) fn get_choose() -> Self {
        let stdin = io::stdin();
        println_flush!("Choose your name: ");
        let name
            = loop {
                let mut name = String::new();
                print_flush!(">>> ");
                match stdin.read_line(&mut name) {
                    Ok(_) => { break name.trim().to_string(); },
                    Err(e) => {
                        println_flush!("error reading input: {}", e);
                        continue;
                    },
                }
            };
        println_flush!(
            "Choose your heroes:\n\
            [1] Warrior\n\
            [2] Mage\n\
            [3] Archer\n\
            [4] Engineer\n\
            [5] Assassin\n\
            [6] Priest"
        );
        println_flush!("Left hero:");
        let hero_l = Hero::get_choose();
        println_flush!("Right hero:");
        let hero_r
            = loop {
                let hero_r = Hero::get_choose();
                if hero_l.get_kind() == hero_r.get_kind() {
                    println_flush!("heroes must be different");
                    continue;
                } else {
                    break hero_r;
                }
            };
        Self::new(&name, hero_l, hero_r)
    }

    pub(crate) fn get_choose_cpu() -> Self {
        let mut rng = thread_rng();
        let hero_l
            = match rng.gen_range(0..6) {
                0 => Hero::new_warrior(),
                1 => Hero::new_mage(),
                2 => Hero::new_archer(),
                3 => Hero::new_engineer(),
                4 => Hero::new_assassin(),
                5 => Hero::new_priest(),
                _ => unreachable!(),
            };
        let hero_r
            = match hero_l.get_kind() {
                HeroKind::Warrior
                    => match rng.gen_range(0..5) {
                        0 => Hero::new_mage(),
                        1 => Hero::new_archer(),
                        2 => Hero::new_engineer(),
                        3 => Hero::new_assassin(),
                        4 => Hero::new_priest(),
                        _ => unreachable!(),
                    },
                HeroKind::Mage
                    => match rng.gen_range(0..5) {
                        0 => Hero::new_warrior(),
                        1 => Hero::new_archer(),
                        2 => Hero::new_engineer(),
                        3 => Hero::new_assassin(),
                        4 => Hero::new_priest(),
                        _ => unreachable!(),
                    },
                HeroKind::Archer
                    => match rng.gen_range(0..5) {
                        0 => Hero::new_warrior(),
                        1 => Hero::new_mage(),
                        2 => Hero::new_engineer(),
                        3 => Hero::new_assassin(),
                        4 => Hero::new_priest(),
                        _ => unreachable!(),
                    },
                HeroKind::Engineer
                | HeroKind::Assassin
                | HeroKind::Priest
                    => match rng.gen_range(0..3) {
                        0 => Hero::new_warrior(),
                        1 => Hero::new_mage(),
                        2 => Hero::new_archer(),
                        _ => unreachable!(),
                    },
            };
        Self::new("CPU", hero_l, hero_r)
    }

    pub fn new(name: &str, hero_l: Hero, hero_r: Hero) -> Self {
        Self {
            name: name.to_string(),
            crown: INIT_CROWN,
            bulwark: INIT_BULWARK,
            hero_l,
            hero_r,
        }
    }

    pub fn get_crown(&self) -> u8 { self.crown }

    pub fn get_bulwark(&self) -> u8 { self.bulwark }

    pub fn crown_inc(&mut self, inc: u8) {
        self.crown = self.crown.saturating_add(inc).min(MAX_CROWN);
    }

    pub fn crown_dec(&mut self, dec: u8) {
        self.crown = self.crown.saturating_sub(dec);
    }

    pub fn bulwark_inc(&mut self, inc: u8) {
        self.bulwark = self.bulwark.saturating_add(inc).min(MAX_BULWARK);
    }

    pub fn bulwark_dec(&mut self, dec: u8) {
        self.bulwark = self.bulwark.saturating_sub(dec);
    }

    pub fn get_name(&self) -> &str { &self.name }

    pub fn get_hero(&self, pos: HeroPos) -> &Hero {
        match pos {
            HeroPos::L => &self.hero_l,
            HeroPos::R => &self.hero_r,
        }
    }

    pub fn get_hero_act(&self, pos: HeroPos) -> Option<&Hero> {
        match pos {
            HeroPos::L => self.hero_l.get_act().then_some(&self.hero_l),
            HeroPos::R => self.hero_r.get_act().then_some(&self.hero_r),
        }
    }

    pub fn get_hero_mut(&mut self, pos: HeroPos) -> &mut Hero {
        match pos {
            HeroPos::L => &mut self.hero_l,
            HeroPos::R => &mut self.hero_r,
        }
    }

    pub fn get_hero_act_mut(&mut self, pos: HeroPos) -> Option<&mut Hero> {
        match pos {
            HeroPos::L => self.hero_l.get_act().then_some(&mut self.hero_l),
            HeroPos::R => self.hero_r.get_act().then_some(&mut self.hero_r),
        }
    }

    pub fn get_pos_of(&self, kind: HeroKind) -> Option<HeroPos> {
        if self.hero_l.get_kind() == kind {
            Some(HeroPos::L)
        } else if self.hero_r.get_kind() == kind {
            Some(HeroPos::R)
        } else {
            None
        }
    }

    pub fn get_pos_of_act(&self, kind: HeroKind) -> Option<HeroPos> {
        if self.hero_l.get_kind() == kind && self.hero_l.get_act() {
            Some(HeroPos::L)
        } else if self.hero_r.get_kind() == kind && self.hero_r.get_act() {
            Some(HeroPos::R)
        } else {
            None
        }
    }

    pub fn get_hero_of(&self, kind: HeroKind) -> Option<(&Hero, HeroPos)> {
        if self.hero_l.get_kind() == kind {
            Some((&self.hero_l, HeroPos::L))
        } else if self.hero_r.get_kind() == kind {
            Some((&self.hero_r, HeroPos::R))
        } else {
            None
        }
    }

    pub fn get_hero_of_act(&self, kind: HeroKind) -> Option<(&Hero, HeroPos)> {
        if self.hero_l.get_kind() == kind && self.hero_l.get_act() {
            Some((&self.hero_l, HeroPos::L))
        } else if self.hero_r.get_kind() == kind && self.hero_r.get_act() {
            Some((&self.hero_r, HeroPos::R))
        } else {
            None
        }
    }

    pub fn get_hero_of_mut(&mut self, kind: HeroKind)
        -> Option<(&mut Hero, HeroPos)>
    {
        if self.hero_l.get_kind() == kind {
            Some((&mut self.hero_l, HeroPos::L))
        } else if self.hero_r.get_kind() == kind {
            Some((&mut self.hero_r, HeroPos::R))
        } else {
            None
        }
    }

    pub fn get_hero_of_act_mut(&mut self, kind: HeroKind)
        -> Option<(&mut Hero, HeroPos)>
    {
        if self.hero_l.get_kind() == kind && self.hero_l.get_act() {
            Some((&mut self.hero_l, HeroPos::L))
        } else if self.hero_r.get_kind() == kind && self.hero_r.get_act() {
            Some((&mut self.hero_r, HeroPos::R))
        } else {
            None
        }
    }

    pub fn get_assassin_target(&self) -> (&Hero, HeroPos) {
        let energy_l: u8 = self.hero_l.get_energy();
        let kind_l: HeroKind = self.hero_l.get_kind();
        let act_l: bool = self.hero_l.get_act();
        let energy_r: u8 = self.hero_r.get_energy();
        let kind_r: HeroKind = self.hero_r.get_kind();
        let act_r: bool = self.hero_r.get_act();
        match (energy_l, kind_l, act_l, energy_r, kind_r, act_r) {
            (_, HeroKind::Assassin, true, _, _, _)
                => (&self.hero_r, HeroPos::R),
            (_, _, _, _, HeroKind::Assassin, true)
                => (&self.hero_l, HeroPos::L),
            (el, _, _, er, _, _) => {
                match el.cmp(&er) {
                    Ordering::Greater => (&self.hero_l, HeroPos::L),
                    Ordering::Less => (&self.hero_r, HeroPos::R),
                    Ordering::Equal => {
                        let mut rng = thread_rng();
                        if rng.gen::<bool>() {
                            (&self.hero_l, HeroPos::L)
                        } else {
                            (&self.hero_r, HeroPos::R)
                        }
                    },
                }
            },
        }
    }

    pub fn get_assassin_target_mut(&mut self) -> (&mut Hero, HeroPos) {
        let energy_l: u8 = self.hero_l.get_energy();
        let kind_l: HeroKind = self.hero_l.get_kind();
        let act_l: bool = self.hero_l.get_act();
        let energy_r: u8 = self.hero_r.get_energy();
        let kind_r: HeroKind = self.hero_r.get_kind();
        let act_r: bool = self.hero_r.get_act();
        match (energy_l, kind_l, act_l, energy_r, kind_r, act_r) {
            (_, HeroKind::Assassin, true, _, _, _)
                => (&mut self.hero_r, HeroPos::R),
            (_, _, _, _, HeroKind::Assassin, true)
                => (&mut self.hero_l, HeroPos::L),
            (el, _, _, er, _, _) => {
                match el.cmp(&er) {
                    Ordering::Greater => (&mut self.hero_l, HeroPos::L),
                    Ordering::Less => (&mut self.hero_r, HeroPos::R),
                    Ordering::Equal => {
                        let mut rng = thread_rng();
                        if rng.gen::<bool>() {
                            (&mut self.hero_l, HeroPos::L)
                        } else {
                            (&mut self.hero_r, HeroPos::R)
                        }
                    },
                }
            },
        }
    }
}

