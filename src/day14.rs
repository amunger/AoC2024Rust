use crate::file_input;

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

fn print_room(room: &Vec<Vec<i32>>) {
    for i in 0..room.len() {
        for j in 0..room[i].len() {
            print!("{}", room[i][j]);
        }
        println!();
    }
}

pub fn solve() {
    let robots = get_robots();
    let mut q1_bots = 0;
    let mut q2_bots = 0;
    let mut q3_bots = 0;
    let mut q4_bots = 0;

    let mut room: Vec<Vec<i32>> = vec![vec![0; WIDTH as usize]; HEIGHT as usize];    

    for i in 0..robots.len() {
        let robot = &robots[i];
        let point = get_position_after_times(robot, 100);

        room[point.y as usize][point.x as usize] += 1;

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

    //print_room(&room);

    println!("Q1: {}, Q2: {}, Q3: {}, Q4: {}", q1_bots, q2_bots, q3_bots, q4_bots);
    println!("{}", q1_bots * q2_bots * q3_bots * q4_bots);
}

struct Point {
    x: i32,
    y: i32,
}

struct Robot {
    init_pos: Point,
    direction: Point,
}