use crate::file_input;
use std::io::Write;

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

fn get_robots() -> Vec<Robot> {
    let input: Vec<String> = file_input::read_input();
    let mut robots: Vec<Robot> = Vec::new();
    for line in input {
        // p=0,4 v=3,-3
        let re = regex::Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
        let caps = re.captures(&line).unwrap();

        let pos = vec![&caps[1], &caps[2]];
        let vel = vec![&caps[3], &caps[4]];

        let init_pos = Point {
            x: pos[0].parse().unwrap(),
            y: pos[1].parse().unwrap(),
        };

        let direction = Point {
            x: vel[0].parse().unwrap(),
            y: vel[1].parse().unwrap(),
        };

        let robot = Robot {
            init_pos: init_pos,
            direction: direction,
        };
        robots.push(robot);
    }
    robots
}

fn get_position_after_times(robot: &Robot, times: i32)-> Point {
    let x = (robot.init_pos.x + robot.direction.x * times) % WIDTH;
    let y = (robot.init_pos.y + robot.direction.y * times) % HEIGHT; 
    Point {
        x: if x < 0 { x + WIDTH } else { x },
        y: if y < 0 { y + HEIGHT } else { y },
    }
}

fn print_room(room: &Vec<Vec<i32>>, file_name: &str) {
    let mut file = std::fs::File::create(file_name).unwrap();
    for i in 0..room.len() {
        for j in 0..room[i].len() {
            let n = if room[i][j] > 0 { room[i][j].to_string() } else { ".".to_string() };
            write!(file, "{}", n).unwrap();
        }
        write!(file, "\n").unwrap();
    }
}

fn small_differece(a: i32, b: i32) -> bool {
    (a - b).abs() < 2
}

pub fn solve() {
    let robots = get_robots();

    for times in 0..10000 {
        let mut top_corner_bots = 0;
        let mut trunk_bots = 0;
        let mut q1_bots = 0;
        let mut q2_bots = 0;
        let mut q3_bots = 0;
        let mut q4_bots = 0;
    
        let mut room: Vec<Vec<i32>> = vec![vec![0; WIDTH as usize]; HEIGHT as usize];    
    
        for i in 0..robots.len() {
            let robot = &robots[i];
            let point = get_position_after_times(robot, times);
    
            room[point.y as usize][point.x as usize] += 1;
    
            if point.x + point.y < WIDTH / 4 {
                top_corner_bots += 1;
            } else if WIDTH - point.x + point.y < WIDTH / 4 {
                top_corner_bots += 1;   
            }

            if point.x > WIDTH / 2 - 10 && point.x < WIDTH / 2 + 10
            && point.y > HEIGHT - 10 {
                trunk_bots += 1;
            }

            if point.x < WIDTH/2 && point.y < HEIGHT/2 {
                q1_bots += 1;
            } else if point.x > WIDTH/2 && point.y < HEIGHT/2 {
                q2_bots += 1;
            } else if point.x < WIDTH/2 && point.y > HEIGHT/2 {
                q3_bots += 1;
            } else if point.x > WIDTH/2 && point.y > HEIGHT/2 {
                q4_bots += 1;
            }
        }

        let mut longest_line = 0;
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                if room[i as usize][j as usize] > 0 {
                    let mut line = 0;
                    for k in j..WIDTH {
                        if room[i as usize][k as usize] > 0 {
                            line += 1;
                        } else {
                            break;
                        }
                    }
                    if line > longest_line {
                        longest_line = line;
                    }
                }
            }
        }

        if longest_line > 10 {
            println!("corner bots: {}, Times: {}", top_corner_bots, times);
            let file_name = format!("{}-timesLongLine.txt", times);
            print_room(&room, &file_name);
        }

        if top_corner_bots < 11 {
            println!("corner bots: {}, Times: {}", top_corner_bots, times);
            let file_name = format!("{}-times.txt", times);
            print_room(&room, &file_name);
        }
    }

    
}

struct Point {
    x: i32,
    y: i32,
}

struct Robot {
    init_pos: Point,
    direction: Point,
}