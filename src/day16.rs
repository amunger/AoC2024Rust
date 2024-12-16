use core::str;
use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap};
use std::cmp::Ordering;

use crate::file_input;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy, Ord, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Eq, PartialEq)]
struct Visit {
    point: Point,
    cost: i32,
}

impl Ord for Visit {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse the order to make BinaryHeap a min-heap
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Visit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone)]
struct Parent {
    parent: Point,
    turn: bool,
}

struct Cell {
    blocked: bool,
    path: bool,
}

struct Map {
    cells: Vec<Vec<Cell>>,
    start: Point,
    end: Point,
}

fn parse_input(input: &Vec<String>) -> Map {
    let mut cells = Vec::new();
    let mut start = Point { x: 0, y: 0 };
    let mut end = Point { x: 0, y: 0 };
    for i in 0..input.len() {
        let mut row = Vec::new();
        for j in 0..input[i].len() {
            let c = input[i].chars().nth(j).unwrap();
            let cell = 
                if c == '#' {
                    Cell { blocked: true, path: false }
                } else if c == '.' {
                    Cell { blocked: false, path: false }
                } else if c == 'S' {
                    start = Point { x: i as i32, y: j as i32 };
                    Cell { blocked: false, path: true }
                } else if c == 'E' {
                    end = Point { x: i as i32, y: j as i32 };
                    Cell { blocked: false, path: true }
                } else {
                    panic!("Invalid character in input");
                };
            row.push(cell);
        }
        cells.push(row);
    }
    Map { cells, start, end }
}

fn print_map(map: &Map) {
    for i in 0..map.cells.len() {
        for j in 0..map.cells[i].len() {
            let c = 
                if map.cells[i][j].blocked {
                    "#"
                } else if map.cells[i][j].path {
                    if map.start.x == i as i32 && map.start.y == j as i32 {
                        "S"
                    } else if map.end.x == i as i32 && map.end.y == j as i32 {
                        "E"
                    } else {
                        "o"
                    }
                } else {
                    "."
                };
            print!("{}", c);
        }
        print!("\n");
    }
}

fn get_neighbors(p: &Point, map: &Map) -> Vec<Point>{
    let directions = vec![[-1, 0], [1, 0], [0, -1], [0, 1]];
    let mut neighbors = Vec::new();
    for d in &directions {
        let x = p.x + d[0];
        let y = p.y + d[1];
        if x >= 0 && y >= 0 && x < map.cells.len() as i32 && y < map.cells[0].len() as i32 && !map.cells[x as usize][y as usize].blocked {
            neighbors.push(Point { x, y });
        }
    };
    return neighbors;
}

fn node_point(p: &Point, direction: [i32; 2]) -> Point {
    Point { x: p.x + direction[0], y: p.y + direction[1] }
}

fn bfs_with_path(map: &mut Map) {
    let mut visited: HashMap<Point, i32> = HashMap::new();
    let mut queue = BinaryHeap::new();
    let mut parent_map: HashMap<Point, Option<Parent>> = HashMap::new();

    queue.push(Visit { point: map.start, cost: 0 });
    parent_map.insert(map.start, None);

    while let Some(visit) = queue.pop() {
        if let Some(parent) = parent_map.get(&visit.point) {
            if let Some(parent) = parent {
                visited.insert(parent.parent, visit.cost);
            }
        }

        //println!("Visiting: {:?} with cost {:?}", visit.point, visit.cost);
        let node = visit.point;
        let direction = if let Some(p) = parent_map.get(&node) {
            if let Some(p) = p {
                [node.x - p.parent.x, node.y - p.parent.y]
            } else {
                [0, 1]
            }
        } else {
            [0, 1]
        };

        if !parent_map.contains_key(&node) {
            parent_map.insert(node, Some(Parent { parent: node, turn: false }));
        }

        let straight = node_point(&node, direction);
        for neighbor in get_neighbors(&node, map) {
            if neighbor == map.end {
                let mut cost = if neighbor == straight { 1 } else { 1001 };

                // Reconstruct the path from start to target
                map.cells[node.x as usize][node.y as usize].path = true;

                let mut previous = &parent_map[&node];
                while let Some(n) = previous {
                    if n.turn {
                        cost += 1001;
                    } else {
                        cost += 1;
                    }
                    map.cells[n.parent.x as usize][n.parent.y as usize].path = true;
                    previous = &parent_map[&n.parent];
                }
                println!("Cost: {}", cost);
                return;
            }

            let step_cost = if neighbor == straight { 1 } else { 1001 };

            if !visited.contains_key(&neighbor) || visit.cost + step_cost < *visited.get(&neighbor).unwrap() {
                queue.push(Visit { point: neighbor, cost: visit.cost + step_cost });
                parent_map.insert(neighbor, Some(Parent { parent: node, turn: neighbor != straight }));
            }
        }
    }
}

pub fn solve(){
    let mut map = parse_input(&file_input::read_input());
    bfs_with_path(&mut map);
    //print_map(&map);
}

// 72432 too high