use crate::types::Int;

#[derive(Default)]
pub struct GameInput {
  shot_fired: bool,
  shot_gone: bool,
}

pub struct GameState {
  available_buster: Int,
  shots_flying: Int,
}

pub fn start() -> GameState {
  GameState {
    available_buster: 3,
    shots_flying: 0,
  }
}

pub fn update(prev: GameState, input: GameInput) -> GameState {
  GameState {
    available_buster: update_available_buster(prev.available_buster, input.shot_fired),
    shots_flying: update_shots_flying(prev.shots_flying, input.shot_fired, input.shot_gone),
  }
}

fn update_available_buster(available: Int, fired: bool) -> Int {
  let table = [(fired && available > 0, -1, 0)];
  sum_conditions(available, &table[..])
}

fn update_shots_flying(shots_flown: Int, fired: bool, gone: bool) -> Int {
  let table = [(fired && shots_flown < 3, 1, 0), (gone, -1, 0)];

  sum_conditions(shots_flown, &table[..])
}

fn sum_conditions(initial: i32, table: &[(bool, i32, i32)]) -> i32 {
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

  macro_rules! input {
    ($($field:ident : $val:expr),*) => {
      GameInput {
        $($field: $val,)*
        ..GameInput::default()
      }
    };
  }

  #[test]
  fn basic_game_state() {
    let s = start();
    assert_eq!(s.available_buster, 3);
  }

  #[test]
  fn no_shot_fired_available_buster_unchanged() {
    let s = start();
    let shot = update(s, input!(shot_fired: false));

    assert_eq!(shot.available_buster, 3);
  }

  #[test]
  fn basic_shot_fired_state() {
    let s = start();
    let shot = update(s, input!(shot_fired: true));
    assert_eq!(shot.available_buster, 2);
  }

  #[test]
  fn fire_all_shots() {
    let s = start();

    let mut shot = s;
    for i in 1..3 {
      shot = update(shot, input!(shot_fired: true));
      assert_eq!(shot.available_buster, 3 - i);
    }

    let cant_shot = update(shot, input!(shot_fired: true));
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
    let shot = update(s, input!(shot_fired: false));
    assert_eq!(shot.shots_flying, 0);
  }

  #[test]
  fn basic_shot_flying_state() {
    let s = start();
    let shot = update(s, input!(shot_fired: true));
    assert_eq!(shot.shots_flying, 1);
  }

  #[test]
  fn max_shots_flying() {
    let s = start();

    let mut state = s;
    for i in 1..=3 {
      state = update(state, input!(shot_fired: true));
      assert_eq!(state.shots_flying, i);
    }

    let no_more_shots_flying = update(state, input!(shot_fired: true));
    assert_eq!(no_more_shots_flying.shots_flying, 3);
  }

  #[test]
  fn shot_gone_means_less_shots_flying() {
    let s = setup_ran_out_of_shots();
    let shot = update(s, input!(shot_gone: true));
    assert_eq!(shot.shots_flying, 2);
  }

  fn setup_ran_out_of_shots() -> GameState {
    let s = start();

    let mut shot = s;
    for i in 1..=3 {
      shot = update(shot, input!(shot_fired: true));
      assert_eq!(shot.shots_flying, i);
    }

    return shot;
  }
}
