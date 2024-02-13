use rand::prelude::{ thread_rng, Rng };

/// Description of a single wheel (with a particular panel facing up).
///
/// Wheel panels have a quantity (1-3, inclusive) of each symbol. Squares and
/// Diamonds have an extra boolean flag controlling whether they count toward
/// hero EXP.
#[derive(Copy, Clone, Debug)]
pub enum Wheel {
    Square(u8, bool), 
    Diamond(u8, bool), 
    Hammer(u8), 
}

impl Wheel {
    pub fn get_kind(&self) -> WheelKind {
        match self {
            Self::Square(..) => WheelKind::Square,
            Self::Diamond(..) => WheelKind::Diamond,
            Self::Hammer(..) => WheelKind::Hammer,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum WheelKind {
    Square,
    Diamond,
    Hammer,
}

impl std::fmt::Display for Wheel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: String
            = match self {
                Self::Square(n, b)
                    => "S".repeat((*n).into()) + if *b { "*" } else { "" },
                Self::Diamond(n, b)
                    => "D".repeat((*n).into()) + if *b { "*" } else { "" },
                Self::Hammer(n)
                    => "H".repeat((*n).into()),
            };
        s.fmt(f)
    }
}

pub type Rolls = [Wheel; 5];

impl Wheel {
    // W0: S, D, S, S+, D, H, DD+, H
    fn w0_sides(face: u8) -> Self {
        match face {
            0 => Self::Square(1, false),
            1 => Self::Diamond(1, false),
            2 => Self::Square(1, false),
            3 => Self::Square(1, true),
            4 => Self::Diamond(1, false),
            5 => Self::Hammer(1),
            6 => Self::Diamond(2, true),
            7 => Self::Hammer(1),
            _ => {
                eprintln!("Wheel::w0_sides");
                unreachable!()
            },
        }
    }

    // W1: S+, D, SS, D+, S, H, DD, HH
    fn w1_sides(face: u8) -> Self {
        match face {
            0 => Self::Square(1, true),
            1 => Self::Diamond(1, false),
            2 => Self::Square(2, false),
            3 => Self::Diamond(1, true),
            4 => Self::Square(1, false),
            5 => Self::Hammer(1),
            6 => Self::Diamond(2, false),
            7 => Self::Hammer(2),
            _ => {
                eprintln!("Wheel::w1_sides");
                unreachable!()
            },
        }
    }

    // W2: S+, D, D+, S, D, HH, SS, HH
    fn w2_sides(face: u8) -> Self {
        match face {
            0 => Self::Square(1, true),
            1 => Self::Diamond(1, false),
            2 => Self::Diamond(1, true),
            3 => Self::Square(1, false),
            4 => Self::Diamond(1, false),
            5 => Self::Hammer(2),
            6 => Self::Diamond(2, false),
            7 => Self::Hammer(2),
            _ => {
                eprintln!("Wheel::w2_sides");
                unreachable!()
            },
        }
    }

    // W3: S, D, S+, D, HH, S, D+, HH
    fn w3_sides(face: u8) -> Self {
        match face {
            0 => Self::Square(1, false),
            1 => Self::Diamond(1, false),
            2 => Self::Square(1, true),
            3 => Self::Diamond(1, false),
            4 => Self::Hammer(2),
            5 => Self::Square(1, false),
            6 => Self::Diamond(1, true),
            7 => Self::Hammer(2),
            _ => {
                eprintln!("Wheel::w3_sides");
                unreachable!()
            },
        }
    }

    // W4: S, DD+, HHH, SS+, DD+, SS+, D, HH
    fn w4_sides(face: u8) -> Self {
        match face {
            0 => Self::Square(1, false),
            1 => Self::Diamond(2, true),
            2 => Self::Hammer(3),
            3 => Self::Square(2, true),
            4 => Self::Diamond(2, true),
            5 => Self::Square(2, true),
            6 => Self::Diamond(1, false),
            7 => Self::Hammer(2),
            _ => {
                eprintln!("Wheel::w4_sides");
                unreachable!()
            },
        }
    }

    /// Generate a series of wheel states.
    pub fn gen_rolls<R>(rng: &mut R) -> Rolls
    where R: Rng + ?Sized
    {
        let w0 = Self::w0_sides(rng.gen_range(0..8));
        let w1 = Self::w1_sides(rng.gen_range(0..8));
        let w2 = Self::w2_sides(rng.gen_range(0..8));
        let w3 = Self::w3_sides(rng.gen_range(0..8));
        let w4 = Self::w4_sides(rng.gen_range(0..8));
        [w0, w1, w2, w3, w4]
    }

    /// Re-generate wheel states for unlocked wheels.
    pub fn gen_rolls_locked<R>(
        rolls: &mut Rolls,
        locks: &[bool; 5],
        rng: &mut R,
    )
    where R: Rng + ?Sized
    {
        if !locks[0] { rolls[0] = Self::w0_sides(rng.gen_range(0..8)); }
        if !locks[1] { rolls[1] = Self::w1_sides(rng.gen_range(0..8)); }
        if !locks[2] { rolls[2] = Self::w2_sides(rng.gen_range(0..8)); }
        if !locks[3] { rolls[3] = Self::w3_sides(rng.gen_range(0..8)); }
        if !locks[4] { rolls[4] = Self::w4_sides(rng.gen_range(0..8)); }
    }

    /// Return `Some(n)` if `self` is `Square(n, _)`.
    pub fn square_energy(&self) -> Option<u8> {
        match self { Self::Square(n, _) => Some(*n), _ => None }
    }

    /// Return `Some(1)` if `self` is `Square(_, true)`.
    pub fn square_exp(&self) -> Option<u8> {
        match self { Self::Square(_, b) => b.then_some(1), _ => None }
    }

    /// Return `Some(n)` if `self` is `Diamond(n, _)`.
    pub fn diamond_energy(&self) -> Option<u8> {
        match self { Self::Diamond(n, _) => Some(*n), _ => None }
    }

    /// Return `Some(1)` if `self` is `Diamond(_, true)`.
    pub fn diamond_exp(&self) -> Option<u8> {
        match self { Self::Diamond(_, b) => b.then_some(1), _ => None }
    }

    /// Return `Some(n)` if `self` is `Hammer(n)`.
    pub fn hammer_energy(&self) -> Option<u8> {
        match self { Self::Hammer(n) => Some(*n), _ => None }
    }

    /// Calculate the total square, diamond, hammer, and EXP counts in a series
    /// of rolls.
    pub fn totals(rolls: &Rolls) -> RollTotals {
        let squares: u8 = rolls.iter().filter_map(Self::square_energy).sum();
        let diamonds: u8 = rolls.iter().filter_map(Self::diamond_energy).sum();
        let hammers: u8 = rolls.iter().filter_map(Self::hammer_energy).sum();
        let exp_l: u8 = rolls.iter().filter_map(Self::square_exp).sum();
        let exp_r: u8 = rolls.iter().filter_map(Self::diamond_exp).sum();
        RollTotals { squares, diamonds, hammers, exp_l, exp_r }
    }
}

/// Total counts from a series of rolls.
#[derive(Copy, Clone, Debug)]
pub struct RollTotals {
    /// Total number of squares.
    pub squares: u8,
    /// Total number of diamonds.
    pub diamonds: u8,
    /// Total number of hammers.
    pub hammers: u8,
    /// Total EXP for the left hero.
    pub exp_l: u8,
    /// Total EXP for the right hero.
    pub exp_r: u8,
}

impl RollTotals {
    pub fn max_kind(&self) -> WheelKind {
        [
            (self.squares, WheelKind::Square),
            (self.diamonds, WheelKind::Diamond),
            (self.hammers, WheelKind::Hammer),
        ]
        .into_iter()
        .max_by(|l, r| l.0.cmp(&r.0))
        .map(|(_, kind)| kind)
        .unwrap_or_else(|| {
            let mut rng = thread_rng();
            match rng.gen_range(0..=2_u8) {
                0 => WheelKind::Square,
                1 => WheelKind::Diamond,
                2 => WheelKind::Hammer,
                _ => unreachable!(),
            }
        })
    }
}

