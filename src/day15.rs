use crate::file_input;
use std::io::Write;

#[derive(Copy, Clone)]
struct point {
    x: i32,
    y: i32,
}

struct cell {
    position: point,
    is_wall: bool,
    has_block: bool,
}

struct state {
    cells: Vec<Vec<cell>>,
    robot: point,
    moves: Vec<point>,
}

fn print_room(cells: &Vec<Vec<cell>>, robot: &point, file_name: &str, append: bool) {
    let mut file = if append {
        std::fs::OpenOptions::new().append(true).open(file_name).unwrap()
    } else {
        std::fs::File::create(file_name).unwrap()
    };

    for i in 0..cells.len() {
        for j in 0..cells[i].len() {
            let n = if i == robot.x as usize && j == robot.y as usize {
                    "@" 
                } else if cells[i][j].has_block { 
                    "O" 
                } else if cells[i][j].is_wall {
                    "#" 
                } else { 
                    "." 
                };
            write!(file, "{}", n).unwrap();
        }
        write!(file, "\n").unwrap();
    }
}

fn parse_input(input: &Vec<String>) -> state {
    let mut ix = 0;
    let mut cells = Vec::new();
    let mut robot = point { x: 0, y: 0 };
    while input[ix].starts_with("#") {
        let mut row = Vec::new();
        for jx in 0..input[ix].len() {
            let c = input[ix].chars().nth(jx).unwrap();
            //print!("{}", c);
            if c == '#' {
                row.push(cell {
                    position: point { x: ix as i32, y: jx as i32 },
                    is_wall: true,
                    has_block: false,
                });
            } else if c == 'O' {
                row.push(cell {
                    position: point { x: ix as i32, y: jx as i32 },
                    is_wall: false,
                    has_block: true,
                });
            } else {
                if c == '@' {
                    robot = point { x: ix as i32, y: jx as i32 };
                }
                row.push(cell {
                    position: point { x: ix as i32, y: jx as i32 },
                    is_wall: false,
                    has_block: false,
                });
            }
        }
        //print!("\n");
        cells.push(row);
        ix += 1;
    }

    let directions = vec![point { x: 0, y: 1 }, point { x: 0, y: -1 }, point { x: 1, y: 0 }, point { x: -1, y: 0 }];
    let mut moves = Vec::new();
    for i in ix..input.len(){
        for c in input[i].chars(){
            if c == '>' {
                moves.push(directions[0]);
            } else if c == '<' {
                moves.push(directions[1]);
            } else if c == 'v' {
                moves.push(directions[2]);
            } else if c == '^' {
                moves.push(directions[3]);
            }
        }
    }

    return state { cells: cells, robot: robot, moves: moves };
}

fn calculate_next_position(p: &point, move_: &point) -> point {
    return point { x: p.x + move_.x, y: p.y + move_.y };
}

fn move_block(p: &point, direction: &point, cells: &mut Vec<Vec<cell>>) -> bool {
    let next = calculate_next_position(p, direction);
    if cells[next.x as usize][next.y as usize].is_wall {
        return false;
    }
    if cells[next.x as usize][next.y as usize].has_block {
        return move_block(&next, direction, cells)
    }
    
    cells[next.x as usize][next.y as usize].has_block = true;
    return true;
}

fn move_robot(state: &mut state) {
    print_room(&state.cells, &state.robot, "room.txt", false);

    for move_ in &state.moves {
     
        let next = calculate_next_position(&state.robot, move_);
        if state.cells[next.x as usize][next.y as usize].is_wall {
            continue;
        }

        if state.cells[next.x as usize][next.y as usize].has_block {
            if move_block(&next, move_, &mut state.cells) {
                state.cells[next.x as usize][next.y as usize].has_block = false;
                state.robot = next;
            }
        } else {
            state.robot = next;
        }

        //print_room(&state.cells, &state.robot, "room.txt", true);
    }
}

fn calc_answer(cells: &Vec<Vec<cell>>) {
    let mut sum = 0;
    for i in 0..cells.len() {
        for j in 0..cells[i].len() {
            let cell = &cells[i][j];
            if cell.has_block {
                let y = i * 100;
                sum += j + y;
            }
        }
    }
    println!("sum: {}", sum);
}

pub fn solve(){
    let mut state = parse_input(&file_input::read_input());
    
    move_robot(&mut state);

    calc_answer(&state.cells);
}