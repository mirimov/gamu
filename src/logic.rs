use crate::numeric_struct;
use crate::types::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GamuInput(Int);

pub fn input() -> GamuInput {
  GamuInput(0)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Action {
  None = 0,
  ShotFired = 1,
  ShotGone = 1 << 1,
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
    "Invariant Violation: ShotCount {} is out of bounds!",
    val
  )
});

pub struct GameState {
  available_buster: BusterCounter,
  shots_flying: Int,
}

pub fn start() -> GameState {
  GameState {
    available_buster: 3.into(),
    shots_flying: 0,
  }
}

pub fn update(prev: GameState, input: GamuInput) -> GameState {
  GameState {
    available_buster: update_available_buster(prev.available_buster, input),
    shots_flying: update_shots_flying(prev.shots_flying, input),
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

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn basic_game_state() {
    let s = start();
    assert_eq!(s.available_buster, 3);
  }

  #[test]
  fn no_shot_fired_means_available_buster_unchanged() {
    let s = start();
    let shot = update(s, input());

    assert_eq!(shot.available_buster, 3);
  }

  #[test]
  fn basic_shot_fired_state() {
    let s = start();
    let shot = update(s, input().press(Action::ShotFired));
    assert_eq!(shot.available_buster, 2);
  }

  #[test]
  fn fire_all_shots() {
    let s = start();

    let mut shot = s;
    for i in 1..3 {
      shot = update(shot, input().press(Action::ShotFired));
      assert_eq!(shot.available_buster, 3 - i);
    }

    let cant_shot = update(shot, input().press(Action::ShotFired));
    assert_eq!(cant_shot.available_buster, 0);
  }

  #[test]
  fn no_shots_flying_in_start() {
    let s = start();
    assert_eq!(s.shots_flying, 0)
  }

  #[test]
  fn no_shot_fired_no_shots_flying_state() {
    let s = start();
    let shot = update(s, input());
    assert_eq!(shot.shots_flying, 0);
  }

  #[test]
  fn basic_shot_flying_state() {
    let s = start();
    let shot = update(s, input().press(Action::ShotFired));
    assert_eq!(shot.shots_flying, 1);
  }

  #[test]
  fn max_shots_flying() {
    let s = start();

    let mut state = s;
    for i in 1..=3 {
      state = update(state, input().press(Action::ShotFired));
      assert_eq!(state.shots_flying, i);
    }

    let no_more_shots_flying = update(state, input().press(Action::ShotFired));
    assert_eq!(no_more_shots_flying.shots_flying, 3);
  }

  #[test]
  fn shot_gone_means_less_shots_flying() {
    let s = setup_ran_out_of_shots();
    let shot = update(s, input().press(Action::ShotGone));
    assert_eq!(shot.shots_flying, 2);
  }

  fn setup_ran_out_of_shots() -> GameState {
    let s = start();

    let mut shot = s;
    for i in 1..=3 {
      shot = update(shot, input().press(Action::ShotFired));
      assert_eq!(shot.shots_flying, i);
    }

    return shot;
  }
}
