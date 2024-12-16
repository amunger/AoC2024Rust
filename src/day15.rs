use crate::file_input;
use std::io::Write;

#[derive(Copy, Clone)]
struct point {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone, PartialEq)]
enum side {
    left,
    right,
}

struct cell {
    is_wall: bool,
    block: Option<side>,
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
            let n = 
                if i == robot.x as usize && j == robot.y as usize {
                    "@" 
                } else if let Some(block) = &cells[i][j].block {
                    if *block == side::left {
                        "["
                    } else {
                        "]"
                    }
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
                    is_wall: true,
                    block: None,
                });
                row.push(cell {
                    is_wall: true,
                    block: None,
                });
            } else if c == 'O' {
                row.push(cell {
                    is_wall: false,
                    block: Some(side::left),
                });
                row.push(cell {
                    is_wall: false,
                    block: Some(side::right),
                });
            } else {
                row.push(cell {
                    is_wall: false,
                    block: None,
                });
                if c == '@' {
                    robot = point { x: ix as i32, y: (jx*2) as i32 };
                }
                row.push(cell {
                    is_wall: false,
                    block: None,
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

fn calculate_next_positions(p: &point, move_: &point, cells: &Vec<Vec<cell>>) -> Vec<point> {
    let mut points = Vec::new();
    points.push(point { x: p.x + move_.x, y: p.y + move_.y });

    if move_.y == 0  {
        if let Some(block) = &cells[p.x as usize][p.y as usize].block {
            if let Some(block_next) = &cells[(p.x + move_.x) as usize][p.y as usize].block {
                if block == block_next {
                    return points;
                }
            }
            if block == &side::left {
                points.push(point { x: p.x + move_.x, y: p.y + 1 });
            } else {
                points.push(point { x: p.x + move_.x, y: p.y - 1 });
            }
        }
    }
    points
}

fn can_move(p: &point, direction: &point, cells: &Vec<Vec<cell>>) -> bool {
    let next_points = calculate_next_positions(p, direction, cells);
    for next_point in next_points {
        if cells[next_point.x as usize][next_point.y as usize].is_wall {
            return false;
        }
        if cells[next_point.x as usize][next_point.y as usize].block.is_some() {
            if !can_move(&next_point, direction, cells) {
                return false;
            }
        }
    }
    return true;
}

fn move_block_horizontally(p: &point, direction: &point, cells: &mut Vec<Vec<cell>>) {
    let next_positions = calculate_next_positions(p, direction, cells);
    for next in next_positions {
        if cells[next.x as usize][next.y as usize].block.is_some() {
            move_block(&next, direction, cells);
        }
        if let Some(block) = &cells[p.x as usize][p.y as usize].block {
            let side_char = if *block == side::left { "[" } else { "]" };
            //println!("move block {:?}: {:?},{:?} -> {:?},{:?}", side_char, p.x, p.y, next.x, next.y);
            cells[next.x as usize][next.y as usize].block = Some(*block);
        }
    }
}

fn move_block_vertically(p: &point, direction: &point, cells: &mut Vec<Vec<cell>>, move_other: bool) {
    let next = calculate_next_positions(p, direction, cells)[0];
    if cells[next.x as usize][next.y as usize].block.is_some() {
        //println!("move next block: {:?},{:?}", next.x, next.y);
        move_block(&next, direction, cells);
    }
    if let Some(block) = &cells[p.x as usize][p.y as usize].block {
        let side_char = if *block == side::left { "[" } else { "]" };
        let other_side_pos = if *block == side::left { point { x: p.x, y: p.y + 1 } } else { point { x: p.x, y: p.y - 1 } };
        //println!("move block {:?}: {:?},{:?} -> {:?},{:?}", side_char, p.x, p.y, next.x, next.y);
        cells[next.x as usize][next.y as usize].block = Some(*block);
        cells[p.x as usize][p.y as usize].block = None;

        if move_other {
            //println!("move block other side: {:?},{:?}", other_side_pos.x, other_side_pos.y);
            move_block_vertically(&other_side_pos, direction, cells, false);
        }
    }
}

fn move_block(p: &point, direction: &point, cells: &mut Vec<Vec<cell>>) {
    if direction.x == 0 {
        move_block_horizontally(p, direction, cells);
    } else {
        move_block_vertically(p, direction, cells, true);
    }
}

fn move_robot(state: &mut state) {
    print_room(&state.cells, &state.robot, "room.txt", false);

    for move_ in &state.moves {
     
        let next = calculate_next_positions(&state.robot, move_, &state.cells)[0];
        if state.cells[next.x as usize][next.y as usize].is_wall {
            continue;
        }

        if state.cells[next.x as usize][next.y as usize].block.is_some() {
            if can_move(&next, move_, &mut state.cells) {
                move_block(&next, move_, &mut state.cells);
                state.cells[next.x as usize][next.y as usize].block = None;
                state.robot = next;
            }
        } else {
            state.robot = next;
        }

    }
    print_room(&state.cells, &state.robot, "room.txt", true);
}

fn calc_answer(cells: &Vec<Vec<cell>>) {
    let mut sum = 0;
    for i in 0..cells.len() {
        for j in 0..cells[i].len() {
            if let Some(block) = &cells[i as usize][j as usize].block {
                if *block == side::left {
                    let y = i * 100;
                    sum += j + y;
                }
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