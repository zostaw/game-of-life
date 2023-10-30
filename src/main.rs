use std::io::{stdin, Read};
use std::{thread::sleep, time};
use crate::patterns::*;
mod patterns;

const WIDTH: usize = 79;
const HEIGHT: usize = 60;
const TIME_DELAY_MILISECONDS: u64 = 50;

fn main() {
    let mut state = State::new();
    pattern5(&mut state);

    // present starting position with awaiting Enter
    clearscreen::clear().expect("failed to clear screen");
    state.print_display();
    stdin().read(&mut [0]).unwrap();

    for _ in 0..300 {
        state.progress();
        sleep(time::Duration::from_millis(TIME_DELAY_MILISECONDS));
        clearscreen::clear().expect("failed to clear screen");
        state.print_display();
    }
}

pub struct State {
    lines: [[bool; WIDTH]; HEIGHT],
}

impl State {
    fn new() -> State {
        State {
            lines: [[false; WIDTH]; HEIGHT],
        }
    }

    fn print_display(&self) {
        let lines = self.lines;
        for line_id in 0..HEIGHT {
            for symbol_id in 0..WIDTH {
                if lines[line_id][symbol_id] == true {
                    print!("*");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
    }

    fn progress(&mut self) {
        let mut new_state = self.lines.clone();
        let lines = self.lines;

        for (line_id, line) in lines.iter().enumerate() {
            for (point_id, point) in line.iter().enumerate() {
                let living_neighbours = count_neighbours(lines, line_id, point_id);

                match point {
                    true => {
                        match living_neighbours {
                            2 | 3 => (),
                            _ => new_state[line_id][point_id] = false,
                        };
                    }
                    false => match living_neighbours {
                        3 => new_state[line_id][point_id] = true,
                        _ => (),
                    },
                }
            }
        }

        self.lines = new_state;

        fn count_neighbours(
            lines: [[bool; WIDTH]; HEIGHT],
            line_id: usize,
            point_id: usize,
        ) -> i32 {
            let mut living_neighbours: i32 = 0;
            let max_height: usize = HEIGHT - 1;
            let max_width: usize = WIDTH - 1;

            let range_vertical = match line_id {
                0 => 0..=1,
                line_id if line_id == max_height => (line_id-1)..=line_id,
                _ => (line_id-1)..=(line_id+1),
            };

            for id in range_vertical {
                let line_checking = lines.get(id).map(|line| Some(line)).unwrap();
                let range_horizontal = match point_id {
                    0 => 0..=1,
                    point_id if point_id == max_width => (point_id-1)..=point_id,
                    _ => (point_id-1)..=(point_id+1),
                };

                for id in range_horizontal {
                    if let Some(living_neighs) =
                        line_checking
                            .unwrap()
                            .get(id)
                            .map(|bool_val| match bool_val {
                                true => 1,
                                false => 0,
                            })
                    {
                        living_neighbours += living_neighs;
                    };
                }
            }

            // the point itself is not its neighbour
            if lines[line_id][point_id] {
                living_neighbours -= 1;
            }

            return living_neighbours;
        }
    }
}


