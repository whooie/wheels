use std::io;
use crate::{ print_flush, println_flush };

pub const MAX_LEVEL: u8 = 2;
pub const MAX_EXP: u8 = 6;

#[derive(Copy, Clone, Debug)]
pub enum Hero {
    Warrior { level: u8, exp: u8, energy: u8, act: bool },
    Mage { level: u8, exp: u8, energy: u8, act: bool },
    Archer { level: u8, exp: u8, energy: u8, act: bool },
    Engineer { level: u8, exp: u8, energy: u8, act: bool },
    Assassin { level: u8, exp: u8, energy: u8, act: bool },
    Priest { level: u8, exp: u8, energy: u8, act: bool },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HeroKind {
    Warrior,
    Mage,
    Archer,
    Engineer,
    Assassin,
    Priest,
}

impl std::fmt::Display for HeroKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!("{:?}", self).fmt(f)
    }
}

impl Hero {
    pub(crate) fn get_choose() -> Self {
        let stdin = io::stdin();
        loop {
            let mut hero_num = String::new();
            print_flush!(">>> ");
            match stdin.read_line(&mut hero_num) {
                Ok(_) => { },
                Err(e) => {
                    println_flush!("error reading input: {}", e);
                    continue;
                },
            }
            let res
                = hero_num.trim()
                .parse::<u8>()
                .map_err(|_| format!("failed to parse input '{}'", hero_num))
                .and_then(|n| {
                    if !(1..=6).contains(&n) {
                        Err(format!("invalid input '{}': must be 1-6", n))
                    } else {
                        Ok(n)
                    }
                });
            match res {
                Ok(1) => { break Self::new_warrior(); },
                Ok(2) => { break Self::new_mage(); },
                Ok(3) => { break Self::new_archer(); },
                Ok(4) => { break Self::new_engineer(); },
                Ok(5) => { break Self::new_assassin(); },
                Ok(6) => { break Self::new_priest(); },
                Err(e) => {
                    println_flush!("{}", e);
                    continue;
                },
                _ => unreachable!(),
            }
        }
    }

    pub fn new_warrior() -> Self {
        Self::Warrior { level: 0, exp: 0, energy: 0, act: false }
    }

    pub fn new_mage() -> Self {
        Self::Mage { level: 0, exp: 0, energy: 0, act: false }
    }

    pub fn new_archer() -> Self {
        Self::Archer { level: 0, exp: 0, energy: 0, act: false }
    }

    pub fn new_engineer() -> Self {
        Self::Engineer { level: 0, exp: 0, energy: 0, act: false }
    }

    pub fn new_assassin() -> Self {
        Self::Assassin { level: 0, exp: 0, energy: 0, act: false }
    }

    pub fn new_priest() -> Self {
        Self::Priest { level: 0, exp: 0, energy: 0, act: false }
    }

    pub fn is_warrior(&self) -> bool { matches!(self, Self::Warrior { .. }) }

    pub fn is_mage(&self) -> bool { matches!(self, Self::Mage { .. }) }

    pub fn is_archer(&self) -> bool { matches!(self, Self::Archer { .. }) }

    pub fn is_engineer(&self) -> bool { matches!(self, Self::Engineer { .. }) }

    pub fn is_assassin(&self) -> bool { matches!(self, Self::Assassin { .. }) }

    pub fn is_priest(&self) -> bool { matches!(self, Self::Priest { .. }) }

    pub fn get_kind(&self) -> HeroKind {
        match self {
            Self::Warrior { .. } => HeroKind::Warrior,
            Self::Mage { .. } => HeroKind::Mage,
            Self::Archer { .. } => HeroKind::Archer,
            Self::Engineer { .. } => HeroKind::Engineer,
            Self::Assassin { .. } => HeroKind::Assassin,
            Self::Priest { .. } => HeroKind::Priest,
        }
    }

    pub fn get_level(&self) -> u8 {
        match *self {
            Self::Warrior { level, ..} => level,
            Self::Mage { level, ..} => level,
            Self::Archer { level, ..} => level,
            Self::Engineer { level, ..} => level,
            Self::Assassin { level, ..} => level,
            Self::Priest { level, ..} => level,
        }
    }

    pub fn get_level_mut(&mut self) -> &mut u8 {
        match self {
            Self::Warrior { level, .. } => level,
            Self::Mage { level, .. } => level,
            Self::Archer { level, .. } => level,
            Self::Engineer { level, .. } => level,
            Self::Assassin { level, .. } => level,
            Self::Priest { level, .. } => level,
        }
    }

    pub fn get_exp(&self) -> u8 {
        match *self {
            Self::Warrior { exp, .. } => exp,
            Self::Mage { exp, .. } => exp,
            Self::Archer { exp, .. } => exp,
            Self::Engineer { exp, .. } => exp,
            Self::Assassin { exp, .. } => exp,
            Self::Priest { exp, .. } => exp,
        }
    }

    pub fn get_exp_mut(&mut self) -> &mut u8 {
        match self {
            Self::Warrior { exp, .. } => exp,
            Self::Mage { exp, .. } => exp,
            Self::Archer { exp, .. } => exp,
            Self::Engineer { exp, .. } => exp,
            Self::Assassin { exp, .. } => exp,
            Self::Priest { exp, .. } => exp,
        }
    }

    pub fn get_energy(&self) -> u8 {
        match *self {
            Self::Warrior { energy, .. } => energy,
            Self::Mage { energy, .. } => energy,
            Self::Archer { energy, .. } => energy,
            Self::Engineer { energy, .. } => energy,
            Self::Assassin { energy, .. } => energy,
            Self::Priest { energy, .. } => energy,
        }
    }

    pub fn get_energy_mut(&mut self) -> &mut u8 {
        match self {
            Self::Warrior { energy, .. } => energy,
            Self::Mage { energy, .. } => energy,
            Self::Archer { energy, .. } => energy,
            Self::Engineer { energy, .. } => energy,
            Self::Assassin { energy, .. } => energy,
            Self::Priest { energy, .. } => energy,
        }
    }

    pub fn get_act(&self) -> bool {
        match *self {
            Self::Warrior { act, .. } => act,
            Self::Mage { act, .. } => act,
            Self::Archer { act, .. } => act,
            Self::Engineer { act, .. } => act,
            Self::Assassin { act, .. } => act,
            Self::Priest { act, .. } => act,
        }
    }

    pub fn get_act_mut(&mut self) -> &mut bool {
        match self {
            Self::Warrior { act, .. } => act,
            Self::Mage { act, .. } => act,
            Self::Archer { act, .. } => act,
            Self::Engineer { act, .. } => act,
            Self::Assassin { act, .. } => act,
            Self::Priest { act, .. } => act,
        }
    }

    pub fn set_act(&mut self, act: bool) {
        *self.get_act_mut() = act;
    }

    pub fn get_crown_dmg(&self) -> u8 {
        match *self {
            Self::Warrior { level, exp: _, energy: _, .. } => match level {
                0 => 3, 1 => 5, 2 => 7, _ => unreachable!(),
            },
            Self::Mage { level, exp: _, energy: _, .. } => match level {
                0 => 2, 1 => 3, 2 => 3, _ => unreachable!(),
            },
            Self::Archer { level, exp: _, energy: _, .. } => match level {
                0 => 3, 1 => 4, 2 => 6, _ => unreachable!(),
            },
            Self::Engineer { level, exp: _, energy: _, .. } => match level {
                0 => 1, 1 => 2, 2 => 4, _ => unreachable!(),
            },
            Self::Assassin { level, exp: _, energy: _, .. } => match level {
                0 => 1, 1 => 2, 2 => 2, _ => unreachable!(),
            },
            Self::Priest { level, exp: _, energy: _, .. } => match level {
                0 => 0, 1 => 0, 2 => 0, _ => unreachable!(),
            },
        }
    }

    pub fn get_bulwark_dmg(&self) -> u8 {
        match *self {
            Self::Warrior { level, exp: _, energy: _, .. } => match level {
                0 => 3, 1 => 5, 2 => 5, _ => unreachable!(),
            },
            Self::Mage { level, exp: _, energy: _, .. } => match level {
                0 => 2, 1 => 3, 2 => 5, _ => unreachable!(),
            },
            Self::Archer { level, exp: _, energy: _, .. } => match level {
                0 => 1, 1 => 2, 2 => 3, _ => unreachable!(),
            },
            Self::Engineer { level, exp: _, energy: _, .. } => match level {
                0 => 3, 1 => 5, 2 => 5, _ => unreachable!(),
            },
            Self::Assassin { level, exp: _, energy: _, .. } => match level {
                0 => 0, 1 => 0, 2 => 0, _ => unreachable!(),
            },
            Self::Priest { level, exp: _, energy: _, .. } => match level {
                0 => 0, 1 => 0, 2 => 0, _ => unreachable!(),
            },
        }
    }

    pub fn get_crown_heal(&self) -> u8 {
        match *self {
            Self::Warrior { level, exp: _, energy: _, .. } => match level {
                0 => 0, 1 => 0, 2 => 0, _ => unreachable!(),
            },
            Self::Mage { level, exp: _, energy: _, .. } => match level {
                0 => 0, 1 => 0, 2 => 0, _ => unreachable!(),
            },
            Self::Archer { level, exp: _, energy: _, .. } => match level {
                0 => 0, 1 => 0, 2 => 0, _ => unreachable!(),
            },
            Self::Engineer { level, exp: _, energy: _, .. } => match level {
                0 => 0, 1 => 0, 2 => 0, _ => unreachable!(),
            },
            Self::Assassin { level, exp: _, energy: _, .. } => match level {
                0 => 0, 1 => 0, 2 => 0, _ => unreachable!(),
            },
            Self::Priest { level, exp: _, energy: _, .. } => match level {
                0 => 1, 1 => 2, 2 => 2, _ => unreachable!(),
            },
        }
    }

    pub fn get_bulwark_heal(&self) -> u8 {
        match *self {
            Self::Warrior { level, exp: _, energy: _, .. } => match level {
                0 => 0, 1 => 0, 2 => 0, _ => unreachable!(),
            },
            Self::Mage { level, exp: _, energy: _, .. } => match level {
                0 => 0, 1 => 0, 2 => 0, _ => unreachable!(),
            },
            Self::Archer { level, exp: _, energy: _, .. } => match level {
                0 => 0, 1 => 0, 2 => 0, _ => unreachable!(),
            },
            Self::Engineer { level, exp: _, energy: _, .. } => match level {
                0 => 2, 1 => 2, 2 => 2, _ => unreachable!(),
            },
            Self::Assassin { level, exp: _, energy: _, .. } => match level {
                0 => 0, 1 => 0, 2 => 0, _ => unreachable!(),
            },
            Self::Priest { level, exp: _, energy: _, .. } => match level {
                0 => 0, 1 => 0, 2 => 0, _ => unreachable!(),
            },
        }
    }

    pub fn get_delay(&self) -> u8 {
        match *self {
            Self::Warrior { level, exp: _, energy: _, .. } => match level {
                0 => 0, 1 => 0, 2 => 0, _ => unreachable!(),
            },
            Self::Mage { level, exp: _, energy: _, .. } => match level {
                0 => 0, 1 => 0, 2 => 0, _ => unreachable!(),
            },
            Self::Archer { level, exp: _, energy: _, .. } => match level {
                0 => 0, 1 => 0, 2 => 0, _ => unreachable!(),
            },
            Self::Engineer { level, exp: _, energy: _, .. } => match level {
                0 => 0, 1 => 0, 2 => 0, _ => unreachable!(),
            },
            Self::Assassin { level, exp: _, energy: _, .. } => match level {
                0 => 1, 1 => 1, 2 => 2, _ => unreachable!(),
            },
            Self::Priest { level, exp: _, energy: _, .. } => match level {
                0 => 0, 1 => 0, 2 => 0, _ => unreachable!(),
            },
        }
    }

    pub fn get_energy_gen(&self) -> u8 {
        match *self {
            Self::Warrior { level, exp: _, energy: _, .. } => match level {
                0 => 0, 1 => 0, 2 => 0, _ => unreachable!(),
            },
            Self::Mage { level, exp: _, energy: _, .. } => match level {
                0 => 0, 1 => 0, 2 => 0, _ => unreachable!(),
            },
            Self::Archer { level, exp: _, energy: _, .. } => match level {
                0 => 0, 1 => 0, 2 => 0, _ => unreachable!(),
            },
            Self::Engineer { level, exp: _, energy: _, .. } => match level {
                0 => 0, 1 => 0, 2 => 0, _ => unreachable!(),
            },
            Self::Assassin { level, exp: _, energy: _, .. } => match level {
                0 => 0, 1 => 0, 2 => 0, _ => unreachable!(),
            },
            Self::Priest { level, exp: _, energy: _, .. } => match level {
                0 => 2, 1 => 2, 2 => 3, _ => unreachable!(),
            },
        }
    }

    pub fn get_rod_len(&self) -> u8 {
        match *self {
            Self::Warrior { level, exp: _, energy: _, .. } => match level {
                0 => 3, 1 => 3, 2 => 3, _ => unreachable!(),
            },
            Self::Mage { level, exp: _, energy: _, .. } => match level {
                0 => 5, 1 => 4, 2 => 4, _ => unreachable!(),
            },
            Self::Archer { level, exp: _, energy: _, .. } => match level {
                0 => 4, 1 => 3, 2 => 3, _ => unreachable!(),
            },
            Self::Engineer { level, exp: _, energy: _, .. } => match level {
                0 => 4, 1 => 4, 2 => 3, _ => unreachable!(),
            },
            Self::Assassin { level, exp: _, energy: _, .. } => match level {
                0 => 3, 1 => 3, 2 => 3, _ => unreachable!(),
            },
            Self::Priest { level, exp: _, energy: _, .. } => match level {
                0 => 4, 1 => 3, 2 => 3, _ => unreachable!(),
            },
        }
    }

    pub fn get_energy_left(&self) -> u8 {
        self.get_rod_len() - self.get_energy()
    }

    pub fn level_inc(&mut self) -> bool {
        let level: &mut u8 = self.get_level_mut();
        if *level < MAX_LEVEL { *level += 1; false } else { true }
    }

    pub fn exp_inc(&mut self, inc: u8) -> bool {
        let exp: &mut u8 = self.get_exp_mut();
        *exp += inc;
        let leveled_up: bool = *exp >= MAX_EXP;
        if leveled_up { *exp = 0; }
        leveled_up
    }

    pub fn energy_inc(&mut self, inc: u8) -> bool {
        let rod_len: u8 = self.get_rod_len();
        let energy: &mut u8 = self.get_energy_mut();
        *energy += inc;
        let will_act: bool = *energy >= rod_len;
        if will_act { *energy = 0; }
        if will_act { *self.get_act_mut() = true }
        will_act
    }

    pub fn energy_dec(&mut self, dec: u8) {
        let act = self.get_act();
        let rod_len = self.get_rod_len();
        let energy: &mut u8 = self.get_energy_mut();
        if act && *energy == 0 {
            *energy = rod_len.saturating_sub(dec);
            self.set_act(false);
        } else {
            *energy = energy.saturating_sub(dec);
        }
    }
}

