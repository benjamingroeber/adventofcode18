fn main() {
    // part 1
    {
        let game = Game::new(459, 72_103);
        let result = game.play();
        let high_score = result.into_iter().max().unwrap_or(0);
        println!("Game 1 High Score: {}", high_score);
    }
    // part 2
    {
        let game = Game::new(459, 7_210_300);
        let result = game.play();

        let high_score = result.into_iter().max().unwrap_or(0);
        println!("Game 2 High Score: {}", high_score);
    }
}

type Score = usize;
type MarbleId = usize;

#[derive(Clone, Debug)]
struct Marble {
    value: usize,
    next: MarbleId,
    prev: MarbleId,
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Clockwise,
    Counterclockwise,
}

#[derive(Clone, Debug)]
struct Game {
    circle: Circle,
    players: Vec<Score>,
    max_rounds: usize,

    marble: usize,
    current_player: usize,
}

impl Game {
    fn new(players: usize, max_rounds: usize) -> Self {
        Game {
            circle: Circle::new(),
            max_rounds,
            players: vec![0; players],
            current_player: 0,
            marble: 0,
        }
    }

    fn play(mut self) -> Vec<Score> {
        for _round in 0..self.max_rounds {
            self.advance();
        }

        self.players
    }

    fn advance(&mut self) {
        self.marble += 1;
        self.current_player = (self.current_player + 1) % self.players.len();
        if self.marble % 23 == 0 {
            self.circle.step(Direction::Counterclockwise, 7);

            let removed = self.circle.remove();
            self.players[self.current_player] += self.marble + removed;
        } else {
            self.circle.step(Direction::Clockwise, 1);
            self.circle.insert(self.marble);
        }
    }
}

#[derive(Clone, Debug)]
struct Circle {
    marbles: Vec<Marble>,
    current: MarbleId,
}

impl Circle {
    fn new() -> Self {
        Circle {
            marbles: vec![Marble {
                value: 0,
                next: 0,
                prev: 0,
            }],
            current: 0,
        }
    }
    fn step(&mut self, direction: Direction, steps: usize) {
        match direction {
            Direction::Clockwise => {
                for _ in 0..steps {
                    self.current = self.marbles[self.current].next;
                }
            }
            Direction::Counterclockwise => {
                for _ in 0..steps {
                    self.current = self.marbles[self.current].prev;
                }
            }
        }
    }

    fn current(&self) -> &Marble {
        &self.marbles[self.current]
    }

    fn mut_next(&mut self) -> &mut Marble {
        let next;
        {
            let current = self.current();
            next = current.next;
        }
        &mut self.marbles[next]
    }

    fn mut_prev(&mut self) -> &mut Marble {
        let prev;
        {
            let current = self.current();
            prev = current.prev;
        }
        &mut self.marbles[prev]
    }

    fn insert(&mut self, value: usize) {
        let new;
        {
            let current = self.current();
            new = Marble {
                value,
                prev: self.current,
                next: current.next,
            };
        }

        self.marbles.push(new);
        let marble_id = self.marbles.len() - 1;
        self.current = marble_id;

        let prev = self.mut_prev();
        prev.next = marble_id;

        let next = self.mut_next();
        next.prev = marble_id;
    }

    fn remove(&mut self) -> usize {
        let prev_id;
        let next_id;
        let value;
        {
            let current = self.current();
            prev_id = current.prev;
            next_id = current.next;
            value = current.value;
        }
        {
            let next = self.mut_next();
            next.prev = prev_id;
        }
        {
            let prev = self.mut_prev();
            prev.next = next_id;
        }
        self.current = next_id;

        value
    }
}

#[test]
fn test_game() {
    let game = Game::new(9, 25);
    let result = game.play();

    let high_score = result.iter().max().unwrap();

    assert_eq!(32, *high_score);
}

#[test]
fn test_games() {
    let games = &[
        (10, 1618, 8317),
        (13, 7999, 146373),
        (17, 1104, 2764),
        (21, 6111, 54718),
        (30, 5807, 37305),
    ];

    for (players, limit, expected) in games {
        let game = Game::new(*players, *limit);
        let result = game.play();

        let high_score = result.iter().max().unwrap();
        assert_eq!(high_score, expected);
    }
}
