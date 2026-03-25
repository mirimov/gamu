use crate::numeric_struct;
use crate::types::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Action {
  None = 0,
  ShotFired = 1,
  ShotGone = 1 << 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GamuInput(Int);

pub fn input() -> GamuInput {
  GamuInput(0)
}

impl GamuInput {
  pub fn has(&self, action: Action) -> bool {
    (self.0 & (action as Int)) != 0
  }

  pub fn press(&self, action: Action) -> Self {
    Self(self.0 | (action as Int))
  }
}

numeric_struct!(BusterCounter, val => {
  debug_assert!(
    (0..=3).contains(&val),
    "Invariant Violation: BusterCounter {} is out of bounds!",
    val
  )
});

pub struct Game {
  pub available_buster: BusterCounter,
  pub shots_flying: Int,
}

impl Game {
  pub fn start() -> Game {
    Game {
      available_buster: 3.into(),
      shots_flying: 0,
    }
  }

  pub fn update(&self, input: GamuInput) -> Game {
    Game {
      available_buster: update_available_buster(self.available_buster, input),
      shots_flying: update_shots_flying(self.shots_flying, input),
    }
  }
}

fn update_available_buster(available: BusterCounter, input: GamuInput) -> BusterCounter {
  let fired = input.has(Action::ShotFired);
  let table = [(fired && available > 0.into(), -1, 0)];
  sum_conditions(available.into(), &table[..]).into()
}

fn update_shots_flying(shots_flown: Int, input: GamuInput) -> Int {
  let fired = input.has(Action::ShotFired);
  let gone = input.has(Action::ShotGone);
  let table = [(fired && shots_flown < 3, 1, 0), (gone, -1, 0)];

  sum_conditions(shots_flown, &table[..])
}

fn sum_conditions(initial: Int, table: &[(bool, Int, Int)]) -> Int {
  let mut result = initial;
  for (condition, counted, not_counted) in table {
    if *condition {
      result += counted;
    } else {
      result += not_counted;
    }
  }
  result
}
