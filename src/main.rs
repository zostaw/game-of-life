use std::{thread::sleep, time};
use std::io::{stdin, Read};

const WIDTH: usize = 79;
const HEIGHT: usize = 60;
const TIME_DELAY_MILISECONDS: u64 = 50;

struct State {
    lines: [[bool; WIDTH]; HEIGHT],
}

impl State {
    fn new() -> State {
        State { lines: [[false; WIDTH]; HEIGHT] }
    }

    fn reload(& mut self) {
        let mut new_state = self.lines.clone();
        let lines = self.lines;

        for (line_id, line) in lines.iter().enumerate() {
            for (point_id, point) in line.iter().enumerate() {

                //println!("Line number: {}", line_id);
                let living_neighbours = count_neighbours(lines, line_id, point_id);
                if living_neighbours > 0 {
                    //println!("line_id: {}, point_id: {}", line_id, point_id);
                    //println!("Living neighbour: {}", living_neighbours);
                }

                match point {
                    true => {
                        match living_neighbours {
                            2 | 3 => (),
                            _ => new_state[line_id][point_id] = false,
                        };
                    },
                    false => {
                        match living_neighbours {
                            3 => new_state[line_id][point_id] = true,
                            _ => (),
                        }

                    }
                }

            }
        }

        self.lines = new_state;

        fn count_neighbours(lines:[[bool; WIDTH]; HEIGHT], line_id: usize, point_id: usize) -> i32 {
            let mut living_neighbours: i32 = 0;
            let max_height: usize = HEIGHT - 1;
            let max_width: usize = WIDTH - 1;

            //println!("Matrix size: {}, Line {}", lines.len(),line_id);
            //println!("Line content {:?}", lines[line_id]);

            let range_vertical = match line_id {
                0 => 0..=1,
                line_id if line_id == max_height => line_id-1..=line_id,
                _ => line_id-1..=line_id+1,
                
            };
            //println!("current line id: {} and range: {:?}", line_id, range_vertical);

            for line_checking_id in range_vertical {
                let line_checking = lines.get(line_checking_id).map(|line| Some(line)).unwrap();
                let range_horizontal = match point_id {
                    0 => 0..=1,
                    point_id if point_id == max_width => point_id-1..=point_id,
                    _ => point_id-1..=point_id+1,

                };
                //println!("current point id: {} and range: {:?}", point_id, range_horizontal);

                for id in range_horizontal {
                    if let Some(living_neighs) = line_checking
                        .unwrap()
                        .get(id)
                        .map(|bool_val| match bool_val {
                            true => 1,
                            false => 0,
                        }) {
                            living_neighbours += living_neighs;
                        };
                }
            }

            // the point itself is not it's own neighbour
            if lines[line_id][point_id] {
                living_neighbours -= 1;
            }

            return living_neighbours;
        }

    }
}
fn main() {
    let mut state = State::new();
    pattern5(&mut state);
    clearscreen::clear().expect("failed to clear screen");
    print_display(state.lines);
    stdin().read(&mut [0]).unwrap();
    for _ in 0..300 {
        state.reload();
        sleep(time::Duration::from_millis(TIME_DELAY_MILISECONDS));
        clearscreen::clear().expect("failed to clear screen");
        print_display(state.lines);
    }
}

fn print_display(lines: [[bool; WIDTH]; HEIGHT]) {

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
fn pattern1(state: &mut State) {
    // cross alternation
    state.lines[21][50] = true;
    state.lines[22][50] = true;
    state.lines[23][50] = true;
}
fn pattern2(state: &mut State) {
    // still life
    state.lines[19][51] = true;
    state.lines[20][51] = true;
    state.lines[19][50] = true;
    state.lines[21][50] = true;
    state.lines[22][49] = true;
    state.lines[23][48] = true;
    state.lines[22][47] = true;
    state.lines[23][47] = true;
}
fn pattern3(state: &mut State) {
    // cannon into stable cross
    state.lines[25][50] = true;
    state.lines[25][51] = true;
    state.lines[25][52] = true;
    state.lines[26][49] = true;
    state.lines[26][53] = true;
    state.lines[27][48] = true;
    state.lines[27][54] = true;
    state.lines[28][48] = true;
    state.lines[28][54] = true;
    state.lines[29][51] = true;
    state.lines[30][49] = true;
    state.lines[30][53] = true;
    state.lines[31][50] = true;
    state.lines[31][52] = true;
}
fn pattern4(state: &mut State) {
    // unpredictible
    state.lines[19][51] = true;
    state.lines[20][50] = true;
    state.lines[20][51] = true;
    state.lines[20][52] = true;
    state.lines[21][50] = true;
}
fn pattern5(state: &mut State) {
    // cannon 
    state.lines[5][30] = true;
    state.lines[5][31] = true;
    state.lines[6][30] = true;
    state.lines[6][31] = true;
    state.lines[15][30] = true;
    state.lines[15][31] = true;
    state.lines[15][32] = true;
    state.lines[16][29] = true;
    state.lines[16][33] = true;
    state.lines[17][28] = true;
    state.lines[17][34] = true;
    state.lines[18][28] = true;
    state.lines[18][34] = true;
    state.lines[19][31] = true;
    state.lines[20][29] = true;
    state.lines[20][33] = true;
    state.lines[21][30] = true;
    state.lines[21][31] = true;
    state.lines[21][32] = true;
    state.lines[22][31] = true;
    state.lines[25][28] = true;
    state.lines[25][29] = true;
    state.lines[25][30] = true;
    state.lines[26][28] = true;
    state.lines[26][29] = true;
    state.lines[26][30] = true;
    state.lines[27][27] = true;
    state.lines[27][31] = true;
    state.lines[29][26] = true;
    state.lines[29][27] = true;
    state.lines[29][31] = true;
    state.lines[29][32] = true;
    state.lines[39][28] = true;
    state.lines[39][29] = true;
    state.lines[40][28] = true;
    state.lines[40][29] = true;
}
