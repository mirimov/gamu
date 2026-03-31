use crate::logic::*;
use speculate::*;

speculate! {
  describe "gamu" {
    before {
      let s = Game::start();
    }

    context "available buster" {
      it "basic game state" {
        assert_eq!(s.available_buster, 3);
      }

      it "no shot fired means available buster unchanged" {
        let shot = s.update(input());

        assert_eq!(shot.available_buster, 3);
      }

      it "basic_shot_fired_state" {
        let shot = s.update(input().press(Action::ShotFired));
        assert_eq!(shot.available_buster, 2);
      }

      it "lets you exhaust all buster shots" {
        let mut shot = s;
        for i in 1..3 {
          shot = shot.update(input().press(Action::ShotFired));
          assert_eq!(shot.available_buster, 3 - i);
        }

        let cant_shot = shot.update(input().press(Action::ShotFired));
        assert_eq!(cant_shot.available_buster, 0);
      }
    }

    context "shots flying" {

      it "no shots flying in game start" {
        assert_eq!(s.shots_flying, 0)
      }

      it "no shot fired no shots flying state" {
        let shot = s.update(input());
        assert_eq!(shot.shots_flying, 0);
      }

      it "basic shot flying state" {
        let shot = s.update(input().press(Action::ShotFired));
        assert_eq!(shot.shots_flying, 1);
      }

      it "max shots flying" {
        let mut state = s;
        for i in 1..=3 {
          state = state.update(input().press(Action::ShotFired));
          assert_eq!(state.shots_flying, i);
        }

        let no_more_shots_flying = state.update(input().press(Action::ShotFired));
        assert_eq!(no_more_shots_flying.shots_flying, 3);
      }

      it "shot gone means less shots flying" {
        let mut shot = s;
        for i in 1..=3 {
          shot = shot.update(input().press(Action::ShotFired));
          assert_eq!(shot.shots_flying, i);
        }


        let shot = shot.update(input().press(Action::ShotGone));
        assert_eq!(shot.shots_flying, 2);
      }
    }
  }
}
