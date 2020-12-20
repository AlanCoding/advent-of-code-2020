use super::*;

#[derive(Debug, Clone)]
struct Ship {
    direction: Direction,
    // (x, y)
    location: (i64, i64),
}

impl Ship {
    fn new() -> Self {
        Ship {
            direction: Direction::E,
            location: (0, 0),
        }
    }

    fn rotate_right(&mut self, mut amount: i64) {
        while amount != 0 {
            amount -= 90;
            match self.direction {
                Direction::N => self.direction = Direction::E,
                Direction::E => self.direction = Direction::S,
                Direction::W => self.direction = Direction::N,
                Direction::S => self.direction = Direction::W,
            }
        }
    }

    fn rotate_left(&mut self, mut amount: i64) {
        while amount != 0 {
            amount -= 90;
            match self.direction {
                Direction::N => self.direction = Direction::W,
                Direction::E => self.direction = Direction::N,
                Direction::W => self.direction = Direction::S,
                Direction::S => self.direction = Direction::E,
            }
        }
    }

    fn move_forward(&mut self, amount: i64) {
        match self.direction {
            Direction::N => self.apply_action(Action::N(amount)),
            Direction::E => self.apply_action(Action::E(amount)),
            Direction::W => self.apply_action(Action::W(amount)),
            Direction::S => self.apply_action(Action::S(amount)),
        }
    }

    fn apply_action(&mut self, a: Action) {
        match a {
            Action::N(x) => {
                self.location.1 = self.location.1 - x;
            }
            Action::E(x) => {
                self.location.0 = self.location.0 + x;
            }
            Action::W(x) => {
                self.location.0 = self.location.0 - x;
            }
            Action::S(x) => {
                self.location.1 = self.location.1 + x;
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
    s.location.0 + s.location.1
}
