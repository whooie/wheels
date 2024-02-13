use std::io;
use rand::{ prelude::thread_rng };
use crate::{
    print_flush,
    println_flush,
    engine::{
        hero::{ Hero, HeroKind, MAX_LEVEL, MAX_EXP },
        player::{ Player, HeroPos },
        wheel::{ Wheel, WheelKind, Rolls },
    },
};

pub const DISPW: usize = 80;
pub const TEXTW: usize = DISPW / 2 - 4;

#[derive(Copy, Clone, Debug)]
pub enum PlayerPos {
    P1,
    P2,
}

impl PlayerPos {
    pub fn other(&self) -> Self {
        match *self {
            Self::P1 => Self::P2,
            Self::P2 => Self::P1,
        }
    }
}

#[derive(Clone, Debug)]
pub enum PlayerId {
    P1(String),
    P2(String),
}

impl std::fmt::Display for PlayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::P1(name) => write!(f, "Player 1 ({})", name),
            Self::P2(name) => write!(f, "Player 2 ({})", name),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum LevelUpKind {
    Up(u8),
    Max,
}

#[derive(Copy, Clone, Debug)]
pub enum Damage {
    Crown(u8),
    Bulwark(u8),
}

#[derive(Debug)]
pub enum Action {
    ExpUp(PlayerId, HeroKind, u8),
    LevelUp(PlayerId, HeroKind, LevelUpKind),
    BulwarkUp(PlayerId, u8),
    EnergyAdd(PlayerId, HeroKind, u8),
    AssassinDamage(PlayerId, u8),
    AssassinDelay(PlayerId, PlayerId, HeroKind, u8),
    PriestHeal(PlayerId, u8),
    PriestEnergy(PlayerId, HeroKind, u8),
    EngineerBuild(PlayerId, u8),
    HeroDamage(PlayerId, HeroKind, Damage),
    Bomb(PlayerId, u8),
}

impl Action {
    pub fn msg(&self) -> String {
        match self {
            Self::ExpUp(player, hero, exp) => {
                format!("{}'s {:?} gains {} EXP", player, hero, exp)
            },
            Self::LevelUp(player, hero, level_up) => {
                match level_up {
                    LevelUpKind::Up(lvl) => {
                        format!("{}'s {:?} levels up to {}", player, hero, lvl)
                    },
                    LevelUpKind::Max => {
                        format!(
                            "{}'s {:?} is already at max level!", player, hero)
                    },
                }
            },
            Self::BulwarkUp(player, inc) => {
                format!("{}'s bulwark is increased by {}", player, inc)
            },
            Self::EnergyAdd(player, hero, energy) => {
                format!("{}'s {:?} gains {} energy", player, hero, energy)
            },
            Self::AssassinDamage(player, damage) => {
                format!(
                    "{}'s Assassin deals {} damage to Crown", player, damage)
            },
            Self::AssassinDelay(player, opponent, hero, delay) => {
                format!(
                    "{}'s Assassin removes {} energy from {}'s {:?}",
                    player, delay, opponent, hero,
                )
            },
            Self::PriestHeal(player, heal) => {
                format!("{}'s Priest heals Crown by {}", player, heal)
            },
            Self::PriestEnergy(player, hero, energy) => {
                format!(
                    "{}'s Priest gives {} energy to {:?}", player, energy, hero)
            },
            Self::EngineerBuild(player, build) => {
                format!("{}'s Engineer adds {} to Bulwark", player, build)
            },
            Self::HeroDamage(player, hero, damage) => {
                match damage {
                    Damage::Crown(dmg) => {
                        format!(
                            "{}'s {:?} deals {} damage to Crown",
                            player, hero, dmg,
                        )
                    },
                    Damage::Bulwark(dmg) => {
                        format!(
                            "{}'s {:?} deals {} damage to Bulwark",
                            player, hero, dmg,
                        )
                    },
                }
            },
            Self::Bomb(player, damage) => {
                format!("{}'s Bomb does {} damage to Crown", player, damage)
            },
        }
    }
}

pub type TurnLog = Vec<Action>;

#[derive(Copy, Clone, Debug)]
pub enum Bomb { Yes, No }

impl Bomb {
    pub fn flip(self) -> Self {
        match self {
            Self::Yes => Self::No,
            Self::No => Self::Yes,
        }
    }

    pub fn flip_mut(&mut self) {
        *self = self.flip();
    }

    pub fn or(self, other: Self) -> Self {
        match (self, other) {
            (Self::No, Self::No) => Self::No,
            _ => Self::Yes,
        }
    }

    pub fn or_mut(&mut self, other: Self) {
        *self = self.or(other);
    }

    pub fn or_opt(self, other: Option<Self>) -> Self {
        self.or(other.unwrap_or(Self::No))
    }

    pub fn or_opt_mut(&mut self, other: Option<Self>) {
        self.or_mut(other.unwrap_or(Self::No));
    }

    pub fn and(self, other: Self) -> Self {
        match (self, other) {
            (Self::Yes, Self::Yes) => Self::Yes,
            _ => Self::No,
        }
    }

    pub fn and_mut(&mut self, other: Self) {
        *self = self.and(other);
    }

    pub fn and_opt(self, other: Option<Self>) -> Self {
        self.and(other.unwrap_or(Self::Yes))
    }

    pub fn and_opt_mut(&mut self, other: Option<Self>) {
        self.and_mut(other.unwrap_or(Self::Yes));
    }
}

impl From<bool> for Bomb {
    fn from(b: bool) -> Self { if b { Self::Yes } else { Self::No } }
}

#[derive(Copy, Clone, Debug)]
pub struct Bombs {
    p1l: Bomb,
    p1r: Bomb,
    p2l: Bomb,
    p2r: Bomb,
}

impl Bombs {
    pub fn get(&self, player: PlayerPos, hero: HeroPos) -> Bomb {
        match (player, hero) {
            (PlayerPos::P1, HeroPos::L) => self.p1l,
            (PlayerPos::P1, HeroPos::R) => self.p1r,
            (PlayerPos::P2, HeroPos::L) => self.p2l,
            (PlayerPos::P2, HeroPos::R) => self.p2r,
        }
    }

    pub fn get_mut(&mut self, player: PlayerPos, hero: HeroPos) -> &mut Bomb {
        match (player, hero) {
            (PlayerPos::P1, HeroPos::L) => &mut self.p1l,
            (PlayerPos::P1, HeroPos::R) => &mut self.p1r,
            (PlayerPos::P2, HeroPos::L) => &mut self.p2l,
            (PlayerPos::P2, HeroPos::R) => &mut self.p2r,
        }
    }

    pub fn flip_mut(&mut self, player: PlayerPos, hero: HeroPos) {
        self.get_mut(player, hero).flip_mut();
    }

    pub fn or_mut(&mut self, b: Option<(PlayerPos, HeroPos, Bomb)>) {
        if let Some((player, hero, bomb)) = b {
            self.get_mut(player, hero).or_mut(bomb);
        }
    }

    pub fn and_mut(&mut self, b: Option<(PlayerPos, HeroPos, Bomb)>) {
        if let Some((player, hero, bomb)) = b {
            self.get_mut(player, hero).and_mut(bomb);
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Act { Yes, No }

impl Act {
    pub fn flip(self) -> Self {
        match self {
            Self::Yes => Self::No,
            Self::No => Self::Yes,
        }
    }

    pub fn flip_mut(&mut self) {
        *self = self.flip();
    }

    pub fn or(self, other: Self) -> Self {
        match (self, other) {
            (Self::No, Self::No) => Self::No,
            _ => Self::Yes,
        }
    }

    pub fn or_mut(&mut self, other: Self) {
        *self = self.or(other);
    }

    pub fn or_opt(self, other: Option<Self>) -> Self {
        self.or(other.unwrap_or(Self::No))
    }

    pub fn or_opt_mut(&mut self, other: Option<Self>) {
        self.or_mut(other.unwrap_or(Self::No));
    }

    pub fn and(self, other: Self) -> Self {
        match (self, other) {
            (Self::Yes, Self::Yes) => Self::Yes,
            _ => Self::No,
        }
    }

    pub fn and_mut(&mut self, other: Self) {
        *self = self.and(other);
    }

    pub fn and_opt(self, other: Option<Self>) -> Self {
        self.and(other.unwrap_or(Self::Yes))
    }

    pub fn and_opt_mut(&mut self, other: Option<Self>) {
        self.and_mut(other.unwrap_or(Self::Yes));
    }
}

impl From<bool> for Act {
    fn from(b: bool) -> Self { if b { Self::Yes } else { Self::No } }
}

#[derive(Copy, Clone, Debug)]
pub struct Acts {
    p1l: Act,
    p1r: Act,
    p2l: Act,
    p2r: Act,
}

impl Acts {
    pub fn get(&self, player: PlayerPos, hero: HeroPos) -> Act {
        match (player, hero) {
            (PlayerPos::P1, HeroPos::L) => self.p1l,
            (PlayerPos::P1, HeroPos::R) => self.p1r,
            (PlayerPos::P2, HeroPos::L) => self.p2l,
            (PlayerPos::P2, HeroPos::R) => self.p2r,
        }
    }

    pub fn get_mut(&mut self, player: PlayerPos, hero: HeroPos) -> &mut Act {
        match (player, hero) {
            (PlayerPos::P1, HeroPos::L) => &mut self.p1l,
            (PlayerPos::P1, HeroPos::R) => &mut self.p1r,
            (PlayerPos::P2, HeroPos::L) => &mut self.p2l,
            (PlayerPos::P2, HeroPos::R) => &mut self.p2r,
        }
    }

    pub fn flip_mut(&mut self, player: PlayerPos, hero: HeroPos) {
        self.get_mut(player, hero).flip_mut();
    }

    pub fn or_mut(&mut self, b: Option<(PlayerPos, HeroPos, Act)>) {
        if let Some((player, hero, bomb)) = b {
            self.get_mut(player, hero).or_mut(bomb);
        }
    }

    pub fn and_mut(&mut self, b: Option<(PlayerPos, HeroPos, Act)>) {
        if let Some((player, hero, bomb)) = b {
            self.get_mut(player, hero).and_mut(bomb);
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Winner {
    P1,
    P2,
    Draw,
}

#[derive(Clone, Debug)]
pub struct Game {
    p1: Player,
    p2: Player,
}

impl Game {
    pub fn get_choose() -> Self {
        let mut _buf = String::new();
        print_flush!("Press ENTER to start: ");
        io::stdin().read_line(&mut _buf).expect("error reading ENTER");
        println_flush!("Player 1:");
        let p1 = Player::get_choose();
        println_flush!("Player 2:");
        let p2 = Player::get_choose();
        Self::new(p1, p2)
    }

    pub fn get_choose_singleplayer() -> Self {
        let mut _buf = String::new();
        print_flush!("Press ENTER to start: ");
        io::stdin().read_line(&mut _buf).expect("error reading ENTER");
        let p1 = Player::get_choose();
        let p2 = Player::get_choose_cpu();
        Self::new(p1, p2)
    }

    pub fn new(p1: Player, p2: Player) -> Self { Self { p1, p2 } }

    fn get_player(&self, pos: PlayerPos) -> &Player {
        match pos {
            PlayerPos::P1 => &self.p1,
            PlayerPos::P2 => &self.p2,
        }
    }

    fn get_player_mut(&mut self, pos: PlayerPos) -> &mut Player {
        match pos {
            PlayerPos::P1 => &mut self.p1,
            PlayerPos::P2 => &mut self.p2,
        }
    }

    fn get_player_id(&self, pos: PlayerPos) -> PlayerId {
        match pos {
            PlayerPos::P1 => PlayerId::P1(self.p1.get_name().to_string()),
            PlayerPos::P2 => PlayerId::P2(self.p2.get_name().to_string()),
        }
    }

    fn get_hero(&self, player: PlayerPos, hero: HeroPos) -> &Hero {
        self.get_player(player).get_hero(hero)
    }

    fn get_hero_act(&self, player: PlayerPos, hero: HeroPos) -> Option<&Hero> {
        self.get_player(player).get_hero_act(hero)
    }

    fn get_hero_mut(&mut self, player: PlayerPos, hero: HeroPos)
        -> &mut Hero
    {
        self.get_player_mut(player).get_hero_mut(hero)
    }

    fn get_hero_act_mut(&mut self, player: PlayerPos, hero: HeroPos)
        -> Option<&mut Hero>
    {
        self.get_player_mut(player).get_hero_act_mut(hero)
    }

    fn get_pos_of(&self, player: PlayerPos, kind: HeroKind) -> Option<HeroPos> {
        self.get_player(player).get_pos_of(kind)
    }

    fn get_pos_of_act(&self, player: PlayerPos, kind: HeroKind)
        -> Option<HeroPos>
    {
        self.get_player(player).get_pos_of_act(kind)
    }

    fn get_hero_of(&self, player: PlayerPos, kind: HeroKind)
        -> Option<(&Hero, HeroPos)>
    {
        self.get_player(player).get_hero_of(kind)
    }

    fn get_hero_of_act(&self, player: PlayerPos, kind: HeroKind)
        -> Option<(&Hero, HeroPos)>
    {
        self.get_player(player).get_hero_of_act(kind)
    }

    fn get_hero_of_mut(&mut self, player: PlayerPos, kind: HeroKind)
        -> Option<(&mut Hero, HeroPos)>
    {
        self.get_player_mut(player).get_hero_of_mut(kind)
    }

    fn get_hero_of_act_mut(&mut self, player: PlayerPos, kind: HeroKind)
        -> Option<(&mut Hero, HeroPos)>
    {
        self.get_player_mut(player).get_hero_of_act_mut(kind)
    }

    fn do_exp_level(
        &mut self,
        player: PlayerPos,
        hero: HeroPos,
        inc: u8,
        log: &mut TurnLog,
    ) -> Bomb
    {
        let id = self.get_player_id(player);
        let hero: &mut Hero = self.get_hero_mut(player, hero);
        let hero_kind = hero.get_kind();
        if inc > 0 {
            log.push(Action::ExpUp(id.clone(), hero_kind, inc));
            if hero.exp_inc(inc) {
                if hero.level_inc() {
                    log.push(Action::LevelUp(
                        id, hero_kind, LevelUpKind::Up(hero.get_level())));
                    Bomb::No
                } else {
                    log.push(Action::LevelUp(id, hero_kind, LevelUpKind::Max));
                    Bomb::Yes
                }
            } else {
                Bomb::No
            }
        } else {
            Bomb::No
        }
    }

    fn do_exp_level_of(
        &mut self,
        player: PlayerPos,
        kind: HeroKind,
        inc: u8,
        log: &mut TurnLog,
    ) -> Option<Bomb>
    {
        let id = self.get_player_id(player);
        self.get_hero_of_mut(player, kind)
            .map(|(hero, _)| {
                if inc > 0 && hero.exp_inc(inc) {
                    if hero.level_inc() {
                        log.push(Action::LevelUp(
                            id, kind, LevelUpKind::Up(hero.get_level())));
                        Bomb::No
                    } else {
                        log.push(Action::LevelUp(
                            id, kind, LevelUpKind::Max));
                        Bomb::Yes
                    }
                } else {
                    Bomb::No
                }
            })
    }

    fn do_bulwark(
        &mut self,
        player: PlayerPos,
        points: u8,
        log: &mut TurnLog,
    ) {
        let id = self.get_player_id(player);
        let player: &mut Player = self.get_player_mut(player);
        if points > 2 {
            log.push(Action::BulwarkUp(id, points - 2));
            player.bulwark_inc(points - 2);
        }
    }

    fn do_energy(
        &mut self,
        player: PlayerPos,
        hero: HeroPos,
        points: u8,
        log: &mut TurnLog,
    ) -> Act
    {
        let id = self.get_player_id(player);
        let hero: &mut Hero = self.get_hero_mut(player, hero);
        let hero_kind = hero.get_kind();
        if points > 2 {
            log.push(Action::EnergyAdd(id, hero_kind, points - 2));
            if hero.energy_inc(points - 2) {
                Act::Yes
            } else {
                Act::No
            }
        } else {
            Act::No
        }
    }

    fn do_assassin(
        &mut self,
        player: PlayerPos,
        log: &mut TurnLog,
    ) -> Option<(PlayerPos, HeroPos, Bomb)>
    {
        let id = self.get_player_id(player);
        let id_opp = self.get_player_id(player.other());
        self.get_hero_of_act_mut(player, HeroKind::Assassin)
            .map(|(hero, pos)| {
                hero.set_act(false);
                let dmg = hero.get_crown_dmg();
                let delay = hero.get_delay();
                (dmg, delay, pos)
            })
            .map(|(dmg, delay, pos)| {
                let opp = self.get_player_mut(player.other());
                // do crown damage
                log.push(Action::AssassinDamage(id.clone(), dmg));
                opp.crown_dec(dmg);
                // do hero delay
                let (target, _) = opp.get_assassin_target_mut();
                let target_kind = target.get_kind();
                log.push(Action::AssassinDelay(
                    id.clone(), id_opp, target_kind, delay));
                target.energy_dec(delay);
                target.set_act(false);
                // +2 EXP from acting
                (player, pos, self.do_exp_level(player, pos, 2, log))
            })
    }

    fn do_priest(
        &mut self,
        player: PlayerPos,
        log: &mut TurnLog,
    ) -> Option<(PlayerPos, HeroPos, Bomb)>
    {
        let id = self.get_player_id(player);
        self.get_hero_of_act_mut(player, HeroKind::Priest)
            .map(|(hero, pos)| {
                hero.set_act(false);
                let heal = hero.get_crown_heal();
                let egen = hero.get_energy_gen();
                (heal, egen, pos)
            })
            .map(|(heal, egen, pos)| {
                let t_pos = pos.other();
                let t_act = self.get_hero(player, t_pos).get_act();
                (heal, egen, pos, t_pos, t_act)
            })
            .map(|(heal, egen, pos, t_pos, t_act)| {
                let plr = self.get_player_mut(player);
                // do crown heal
                log.push(Action::PriestHeal(id.clone(), heal));
                plr.crown_inc(heal);
                // do energy gen
                if !t_act {
                    let target = plr.get_hero_mut(t_pos);
                    let target_kind = target.get_kind();
                    log.push(Action::PriestEnergy(
                        id.clone(), target_kind, egen));
                    target.energy_inc(egen);
                }
                // +2 EXP from acting
                (player, pos, self.do_exp_level(player, pos, 2, log))
            })
    }

    fn do_priest_second(
        &mut self,
        player: PlayerPos,
        prev_acts: Acts,
        log: &mut TurnLog,
    ) {
        let id = self.get_player_id(player);
        self.get_pos_of(player, HeroKind::Priest)
            .and_then(|pos| {
                if prev_acts.get(player, pos) == Act::Yes {
                    let hero = self.get_hero(player, pos);
                    let egen = hero.get_energy_gen();
                    Some((egen, pos))
                } else {
                    None
                }
            })
            .and_then(|(egen, pos)| {
                let t_pos = pos.other();
                if prev_acts.get(player, t_pos) == Act::Yes {
                    let target = self.get_hero_mut(player, t_pos);
                    let target_kind = target.get_kind();
                    log.push(Action::PriestEnergy(
                        id.clone(), target_kind, egen));
                    target.energy_inc(egen);
                    Some(())
                } else {
                    None
                }
            });
    }

    fn do_engineer(
        &mut self,
        player: PlayerPos,
        log: &mut TurnLog,
    ) -> Option<(PlayerPos, HeroPos, Bomb)>
    {
        let id = self.get_player_id(player);
        self.get_hero_of_act_mut(player, HeroKind::Engineer)
            .map(|(hero, pos)| {
                hero.set_act(false);
                let dmg_crown = hero.get_crown_dmg();
                let dmg_blwk = hero.get_bulwark_dmg();
                (dmg_crown, dmg_blwk, pos)
            })
            .map(|(dmg_crown, dmg_blwk, pos)| {
                let opp = self.get_player_mut(player.other());
                // do damage
                if opp.get_bulwark() > 0 {
                    log.push(Action::HeroDamage(
                        id.clone(),
                        HeroKind::Engineer,
                        Damage::Bulwark(dmg_blwk)
                    ));
                    opp.bulwark_dec(dmg_blwk);
                } else {
                    log.push(Action::HeroDamage(
                        id.clone(),
                        HeroKind::Engineer,
                        Damage::Crown(dmg_crown)
                    ));
                    opp.crown_dec(dmg_crown);
                }
                // do bulwark build
                log.push(Action::EngineerBuild(id.clone(), 2));
                self.get_player_mut(player).bulwark_inc(2);
                // +2 EXP from acting
                (player, pos, self.do_exp_level(player, pos, 2, log))
            })
    }

    fn do_warrior(
        &mut self,
        player: PlayerPos,
        log: &mut TurnLog,
    ) -> Option<(PlayerPos, HeroPos, Bomb)>
    {
        let id = self.get_player_id(player);
        self.get_hero_of_act_mut(player, HeroKind::Warrior)
            .map(|(hero, pos)| {
                hero.set_act(false);
                let dmg_crown = hero.get_crown_dmg();
                let dmg_blwk = hero.get_bulwark_dmg();
                (dmg_crown, dmg_blwk, pos)
            })
            .map(|(dmg_crown, dmg_blwk, pos)| {
                let opp = self.get_player_mut(player.other());
                // do damage
                if opp.get_bulwark() > 0 {
                    log.push(Action::HeroDamage(
                        id.clone(),
                        HeroKind::Warrior,
                        Damage::Bulwark(dmg_blwk)
                    ));
                    opp.bulwark_dec(dmg_blwk);
                } else {
                    log.push(Action::HeroDamage(
                        id.clone(),
                        HeroKind::Warrior,
                        Damage::Crown(dmg_crown)
                    ));
                    opp.crown_dec(dmg_crown);
                }
                // +2 EXP from acting
                (player, pos, self.do_exp_level(player, pos, 2, log))
            })
    }

    fn do_mage(
        &mut self,
        player: PlayerPos,
        log: &mut TurnLog,
    ) -> Option<(PlayerPos, HeroPos, Bomb)>
    {
        let id = self.get_player_id(player);
        self.get_hero_of_act_mut(player, HeroKind::Mage)
            .map(|(hero, pos)| {
                hero.set_act(false);
                let dmg_crown = hero.get_crown_dmg();
                let dmg_blwk = hero.get_bulwark_dmg();
                (dmg_crown, dmg_blwk, pos)
            })
            .map(|(dmg_crown, dmg_blwk, pos)| {
                let opp = self.get_player_mut(player.other());
                // do first damage
                if opp.get_bulwark() > 0 {
                    log.push(Action::HeroDamage(
                        id.clone(), HeroKind::Mage, Damage::Bulwark(dmg_blwk)));
                    opp.bulwark_dec(dmg_blwk);
                } else {
                    log.push(Action::HeroDamage(
                        id.clone(), HeroKind::Mage, Damage::Crown(dmg_crown)));
                    opp.crown_dec(dmg_crown);
                }
                // do second damage
                log.push(Action::HeroDamage(
                    id.clone(), HeroKind::Mage, Damage::Crown(dmg_crown)));
                opp.crown_dec(dmg_crown);
                // +2 EXP from acting
                (player, pos, self.do_exp_level(player, pos, 2, log))
            })
    }

    fn do_archer(
        &mut self,
        player: PlayerPos,
        log: &mut TurnLog,
    ) -> Option<(PlayerPos, HeroPos, Bomb)>
    {
        let id = self.get_player_id(player);
        self.get_hero_of_act_mut(player, HeroKind::Archer)
            .map(|(hero, pos)| {
                hero.set_act(false);
                let dmg_crown = hero.get_crown_dmg();
                let dmg_blwk = hero.get_bulwark_dmg();
                (dmg_crown, dmg_blwk, pos)
            })
            .map(|(dmg_crown, dmg_blwk, pos)| {
                let opp = self.get_player_mut(player.other());
                // do damage
                if opp.get_bulwark() > 2 {
                    log.push(Action::HeroDamage(
                        id.clone(),
                        HeroKind::Archer,
                        Damage::Bulwark(dmg_blwk)
                    ));
                    opp.bulwark_dec(dmg_blwk);
                } else {
                    log.push(Action::HeroDamage(
                        id.clone(),
                        HeroKind::Archer,
                        Damage::Crown(dmg_crown)
                    ));
                    opp.crown_dec(dmg_crown);
                }
                // +2 EXP from acting
                (player, pos, self.do_exp_level(player, pos, 2, log))
            })
    }

    fn do_bomb(
        &mut self,
        player: PlayerPos,
        bomb: &mut Bomb,
        log: &mut TurnLog,
    ) {
        let id = self.get_player_id(player);
        match *bomb {
            Bomb::Yes => {
                let opp = self.get_player_mut(player.other());
                log.push(Action::Bomb(id, 2));
                opp.crown_dec(2);
                *bomb = Bomb::No;
            },
            Bomb::No => { },
        }
    }

    fn do_bombs(&mut self, bombs: &mut Bombs, log: &mut TurnLog) {
        use PlayerPos::*;
        use HeroPos::*;
        self.do_bomb(P1, bombs.get_mut(P1, L), log);
        self.do_bomb(P1, bombs.get_mut(P1, R), log);
        self.do_bomb(P2, bombs.get_mut(P2, L), log);
        self.do_bomb(P2, bombs.get_mut(P2, R), log);
    }

    pub fn do_turn(&mut self, rolls_p1: &Rolls, rolls_p2: &Rolls)
        -> (Option<Winner>, TurnLog)
    {
        use PlayerPos::*;
        use HeroPos::*;

        let mut log: TurnLog = Vec::new();
        let log_ref = &mut log;
        let totals_p1 = Wheel::totals(rolls_p1);
        let totals_p2 = Wheel::totals(rolls_p2);

        //  1 Panel XP, Level ups
        let p1l = self.do_exp_level(P1, L, totals_p1.exp_l, log_ref);
        let p1r = self.do_exp_level(P1, R, totals_p1.exp_r, log_ref);
        let p2l = self.do_exp_level(P2, L, totals_p2.exp_l, log_ref);
        let p2r = self.do_exp_level(P2, R, totals_p2.exp_r, log_ref);
        let mut bombs = Bombs { p1l, p1r, p2l, p2r };

        //  2 Hammer panels added
        self.do_bulwark(P1, totals_p1.hammers, log_ref);
        self.do_bulwark(P2, totals_p2.hammers, log_ref);

        //  3 Energy panels added
        let p1l = self.do_energy(P1, L, totals_p1.squares, log_ref);
        let p1r = self.do_energy(P1, R, totals_p1.diamonds, log_ref);
        let p2l = self.do_energy(P2, L, totals_p2.squares, log_ref);
        let p2r = self.do_energy(P2, R, totals_p2.diamonds, log_ref);
        let first_acts = Acts { p1l, p1r, p2l, p2r };

        //  4 Assassin Acts
        bombs.or_mut(self.do_assassin(P1, log_ref));
        bombs.or_mut(self.do_assassin(P2, log_ref));

        //  5 Priest heals + (If the second hero does not have enough energy
        //    from energy panels to act: Priest grants energy) + Action XP
        bombs.or_mut(self.do_priest(P1, log_ref));
        bombs.or_mut(self.do_priest(P2, log_ref));

        //  6 Engineer Acts
        bombs.or_mut(self.do_engineer(P1, log_ref));
        bombs.or_mut(self.do_engineer(P2, log_ref));

        //  7 Bombs
        self.do_bombs(&mut bombs, log_ref);

        //  8 Rest of heroes act
        bombs.or_mut(self.do_warrior(P1, log_ref));
        bombs.or_mut(self.do_warrior(P2, log_ref));
        bombs.or_mut(self.do_mage(P1, log_ref));
        bombs.or_mut(self.do_mage(P2, log_ref));
        bombs.or_mut(self.do_archer(P1, log_ref));
        bombs.or_mut(self.do_archer(P2, log_ref));

        //  9 (If the second hero had enough energy from energy panels to act:
        //    Priest grants energy)
        self.do_priest_second(P1, first_acts, log_ref);
        self.do_priest_second(P2, first_acts, log_ref);

        // 10 Hero acts from priest energy
        bombs.or_mut(self.do_assassin(P1, log_ref));
        bombs.or_mut(self.do_assassin(P2, log_ref));
        bombs.or_mut(self.do_engineer(P1, log_ref));
        bombs.or_mut(self.do_engineer(P2, log_ref));
        bombs.or_mut(self.do_warrior(P1, log_ref));
        bombs.or_mut(self.do_warrior(P2, log_ref));
        bombs.or_mut(self.do_mage(P1, log_ref));
        bombs.or_mut(self.do_mage(P2, log_ref));
        bombs.or_mut(self.do_archer(P1, log_ref));
        bombs.or_mut(self.do_archer(P2, log_ref));

        // 11 Bombs (if deployed after a priest caused hero to act and gain XP)
        self.do_bombs(&mut bombs, log_ref);

        // 12 0 HP Crown check (simultaneous)
        let p1_crown = self.get_player(P1).get_crown();
        let p2_crown = self.get_player(P2).get_crown();
        match (p1_crown, p2_crown) {
            (0, 0) => (Some(Winner::Draw), log),
            (0, _) => (Some(Winner::P2), log),
            (_, 0) => (Some(Winner::P1), log),
            _ => (None, log),
        }
    }

    fn display_rolls(rolls: &Rolls) {
        println_flush!("┌─────┐┌─────┐┌─────┐┌─────┐┌─────┐");
        println_flush!("│ {:^3} ││ {:^3} ││ {:^3} ││ {:^3} ││ {:^3} │",
            rolls[0], rolls[1], rolls[2], rolls[3], rolls[4]);
        println_flush!("└──1──┘└──2──┘└──3──┘└──4──┘└──5──┘");
    }

    fn parse_lock_numbers(input: &str)
        -> impl Iterator<Item = Result<usize, String>> + '_
    {
        input.split(',')
            .map(|k_str| {
                k_str.trim()
                    .parse::<usize>()
                    .map_err(|_| {
                        format!("failed to parse input '{}'", k_str.trim())
                    })
                    .and_then(|k| {
                        if !(1..=5).contains(&k) {
                            Err(format!("invalid input '{}': must be 1-5", k))
                        } else {
                            Ok(k)
                        }
                    })
            })
    }

    fn get_rolls_response(&self, rolls: &Rolls, locks: &mut [bool; 5]) {
        Self::display_rolls(rolls);
        let mut input: String;
        let mut lock_numbers: Result<Vec<usize>, String>;
        let stdin = io::stdin();
        loop {
            input = String::new();
            print_flush!(">>> ");
            match stdin.read_line(&mut input) {
                Ok(_) => { },
                Err(e) => {
                    println_flush!("error reading input: {}", e);
                    continue;
                },
            }
            lock_numbers = Self::parse_lock_numbers(&input).collect();
            match lock_numbers {
                Ok(nums) => {
                    nums.into_iter().for_each(|k| { locks[k - 1] = true; });
                    break;
                },
                Err(msg) => {
                    println_flush!("{}", msg);
                    continue;
                },
            }
        }
    }

    fn get_rolls_response_cpu(&self, rolls: &Rolls, locks: &mut [bool; 5]) {
        let plr = self.get_player(PlayerPos::P2);
        let energy_left_l = plr.get_hero(HeroPos::L).get_energy_left();
        let energy_left_r = plr.get_hero(HeroPos::R).get_energy_left();
        let totals = Wheel::totals(rolls);
        let target
            = if (1..=2).contains(&energy_left_l) {
                WheelKind::Square
            } else if (1..=2).contains(&energy_left_r) {
                WheelKind::Diamond
            } else {
                totals.max_kind()
            };
        rolls.iter()
            .zip(locks.iter_mut())
            .for_each(|(wheel, lock)| {
                if wheel.get_kind() == target {
                    *lock = true;
                }
            });
    }

    pub fn get_rolls(&self, turn_counter: usize) -> (Rolls, Rolls) {
        let mut rng = thread_rng();
        let mut p1_rolls = Wheel::gen_rolls(&mut rng);
        let mut p1_locks = [false; 5];
        let mut p2_rolls = Wheel::gen_rolls(&mut rng);
        let mut p2_locks = [false; 5];
        if turn_counter % 2 == 1 {
            println_flush!("Player 1:");
            self.get_rolls_response(&p1_rolls, &mut p1_locks);
            Wheel::gen_rolls_locked(&mut p1_rolls, &p1_locks, &mut rng);
            self.get_rolls_response(&p1_rolls, &mut p1_locks);
            Wheel::gen_rolls_locked(&mut p1_rolls, &p1_locks, &mut rng);
            Self::display_rolls(&p1_rolls);
        } else {
            println_flush!("Player 2:");
            self.get_rolls_response(&p2_rolls, &mut p2_locks);
            Wheel::gen_rolls_locked(&mut p2_rolls, &p2_locks, &mut rng);
            self.get_rolls_response(&p2_rolls, &mut p2_locks);
            Wheel::gen_rolls_locked(&mut p2_rolls, &p2_locks, &mut rng);
            Self::display_rolls(&p2_rolls);
        }
        (p1_rolls, p2_rolls)
    }

    pub fn get_rolls_single(&self) -> Rolls {
        let mut rng = thread_rng();
        let mut rolls = Wheel::gen_rolls(&mut rng);
        let mut locks = [false; 5];
        self.get_rolls_response(&rolls, &mut locks);
        Wheel::gen_rolls_locked(&mut rolls, &locks, &mut rng);
        self.get_rolls_response(&rolls, &mut locks);
        Wheel::gen_rolls_locked(&mut rolls, &locks, &mut rng);
        Self::display_rolls(&rolls);
        rolls
    }

    pub fn get_rolls_cpu(&self) -> Rolls {
        let mut rng = thread_rng();
        let mut rolls = Wheel::gen_rolls(&mut rng);
        let mut locks = [false; 5];
        self.get_rolls_response_cpu(&rolls, &mut locks);
        Wheel::gen_rolls_locked(&mut rolls, &locks, &mut rng);
        self.get_rolls_response_cpu(&rolls, &mut locks);
        Wheel::gen_rolls_locked(&mut rolls, &locks, &mut rng);
        rolls
    }

    fn display_turn(&self, turn: usize) {
        let turn_str = format!("Turn {}", turn);
        let turn_str_len = turn_str.len() + 2;
        let left: usize = (DISPW - turn_str_len) / 2;
        let right: usize = DISPW - left - turn_str_len;
        println_flush!("{} {} {}", "-".repeat(left), turn_str, "-".repeat(right));
    }

    fn display_board(&self) {
        let p1 = self.get_player(PlayerPos::P1);
        let p1l = p1.get_hero(HeroPos::L);
        let p1r = p1.get_hero(HeroPos::R);
        let p2 = self.get_player(PlayerPos::P2);
        let p2l = p2.get_hero(HeroPos::L);
        let p2r = p2.get_hero(HeroPos::R);

        println_flush!("┌{top}┐┌{top}┐", top="─".repeat(TEXTW + 2));
        println_flush!("│ Player 1: {:<w$} ││ Player 2: {:<w$} │",
            p1.get_name(), p2.get_name(), w=TEXTW - 10);
        println_flush!("│ Crown: {:<w$} ││ Crown: {:<w$} │",
            p1.get_crown(), p2.get_crown(), w=TEXTW - 7);
        println_flush!("│ Bulwark: {:<w$} ││ Bulwark: {:<w$} │",
            p1.get_bulwark(), p2.get_bulwark(), w=TEXTW - 9);

        println_flush!("│ Left: {:<w$} ││ Left: {:<w$} │",
            p1l.get_kind(), p2l.get_kind(), w=TEXTW - 6);
        println_flush!("│   Energy: {} / {:<w$} ││   Energy: {} / {:<w$} │",
            p1l.get_energy(), p1l.get_rod_len(),
            p2l.get_energy(), p2l.get_rod_len(),
            w=TEXTW - 14,
        );
        println_flush!("│   EXP: {} / {max:<w$} ││   EXP: {} / {max:<w$} │",
            p1l.get_exp(), p2l.get_exp(), max=MAX_EXP, w=TEXTW - 11);
        println_flush!("│   Level: {} / {max:<w$} ││   Level: {} / {max:<w$} │",
            p1l.get_level(), p2l.get_level(), max=MAX_LEVEL, w=TEXTW - 13);

        println_flush!("│ Right: {:<w$} ││ Right: {:<w$} │",
            p1r.get_kind(), p2r.get_kind(), w=TEXTW - 7);
        println_flush!("│   Energy: {} / {:<w$} ││   Energy: {} / {:<w$} │",
            p1r.get_energy(), p1r.get_rod_len(),
            p2r.get_energy(), p2r.get_rod_len(),
            w=TEXTW - 14,
        );
        println_flush!("│   EXP: {} / {max:<w$} ││   EXP: {} / {max:<w$} │",
            p1r.get_energy(), p2r.get_energy(), max=MAX_EXP, w=TEXTW - 11);
        println_flush!("│   Level: {} / {max:<w$} ││   Level: {} / {max:<w$} │",
            p1r.get_energy(), p2r.get_energy(), max=MAX_LEVEL, w=TEXTW - 13);
        println_flush!("└{bot}┘└{bot}┘", bot="─".repeat(TEXTW + 2));
    }

    fn display_log(&self, log: TurnLog) {
        for action in log.iter() {
            println_flush!("> {}", action.msg());
        }
    }

    pub fn run(&mut self) -> Winner {
        let mut turn_counter: usize = 0;
        let mut rolls: (Rolls, Rolls);
        loop {
            turn_counter += 1;
            self.display_turn(turn_counter);
            self.display_board();
            rolls = self.get_rolls(turn_counter);
            match self.do_turn(&rolls.0, &rolls.1) {
                (Some(winner), turn_log) => {
                    self.display_log(turn_log);
                    return winner;
                },
                (None, turn_log) => {
                    self.display_log(turn_log);
                    continue;
                },
            }
        }
    }

    pub fn run_singleplayer(&mut self) -> Winner {
        let mut turn_counter: usize = 0;
        let mut p1_rolls: Rolls;
        let mut p2_rolls: Rolls;
        loop {
            turn_counter += 1;
            self.display_turn(turn_counter);
            self.display_board();
            p1_rolls = self.get_rolls_single();
            p2_rolls = self.get_rolls_cpu();
            println_flush!("CPU's rolls:");
            Self::display_rolls(&p2_rolls);
            match self.do_turn(&p1_rolls, &p2_rolls) {
                (Some(winner), turn_log) => {
                    self.display_log(turn_log);
                    return winner;
                },
                (None, turn_log) => {
                    self.display_log(turn_log);
                    continue;
                },
            }
        }
    }
}

