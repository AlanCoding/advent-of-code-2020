use super::*;

#[derive(Debug, Clone)]
struct Ship {
    // (x, y)
    location: (i64, i64),
    waypoint: (i64, i64),
}

impl Ship {
    fn new() -> Self {
        Ship {
            location: (0, 0),
            waypoint: (10, -1),
        }
    }

    fn rotate_right(&mut self, mut amount: i64) {
        while amount != 0 {
            amount -= 90;
            let tmp = self.waypoint.0;
            self.waypoint.0 = -self.waypoint.1;
            self.waypoint.1 = tmp;
        }
    }

    fn rotate_left(&mut self, mut amount: i64) {
        while amount != 0 {
            amount -= 90;
            let tmp = self.waypoint.0;
            self.waypoint.0 = self.waypoint.1;
            self.waypoint.1 = -tmp;
        }
    }

    fn move_forward(&mut self, amount: i64) {
        self.location.0 += self.waypoint.0 * amount;
        self.location.1 += self.waypoint.1 * amount;
    }

    fn apply_action(&mut self, a: Action) {
        match a {
            Action::N(x) => {
                self.waypoint.1 = self.waypoint.1 - x;
            }
            Action::E(x) => {
                self.waypoint.0 = self.waypoint.0 + x;
            }
            Action::W(x) => {
                self.waypoint.0 = self.waypoint.0 - x;
            }
            Action::S(x) => {
                self.waypoint.1 = self.waypoint.1 + x;
            }
            Action::L(x) => self.rotate_left(x),
            Action::R(x) => self.rotate_right(x),
            Action::F(x) => self.move_forward(x),
        }
    }
}

pub fn solve(data: impl Iterator<Item = Action>) -> i64 {
    let mut s = Ship::new();
    for action in data {
        s.apply_action(action);
    }
    s.location.0.abs() + s.location.1.abs()
}
