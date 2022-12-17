use super::load_file;

/// --- Day 12: Hill Climbing Algorithm ---
/// You try contacting the Elves using your handheld device, but the river you're following must be
/// too low to get a decent signal.
///
/// You ask the device for a heightmap of the surrounding area (your puzzle input). The heightmap
/// shows the local area from above broken into a grid; the elevation of each square of the grid is
/// given by a single lowercase letter, where a is the lowest elevation, b is the next-lowest, and
/// so on up to the highest elevation, z.
///
/// Also included on the heightmap are marks for your current position (S) and the location that
/// should get the best signal (E). Your current position (S) has elevation a, and the location that
/// should get the best signal (E) has elevation z.
///
/// You'd like to reach E, but to save energy, you should do it in as few steps as possible. During
/// each step, you can move exactly one square up, down, left, or right. To avoid needing to get out
/// your climbing gear, the elevation of the destination square can be at most one higher than the
/// elevation of your current square; that is, if your current elevation is m, you could step to
/// elevation n, but not to elevation o. (This also means that the elevation of the destination
/// square can be much lower than the elevation of your current square.)
///
/// For example:
///
/// Sabqponm
/// abcryxxl
/// accszExk
/// acctuvwj
/// abdefghi
/// Here, you start in the top-left corner; your goal is near the middle. You could start by moving
/// down or right, but eventually you'll need to head toward the e at the bottom. From there, you
/// can spiral around to the goal:
///
/// v..v<<<<
/// >v.vv<<^
/// .>vv>E^^
/// ..v>>>^^
/// ..>>>>>^
/// In the above diagram, the symbols indicate whether the path exits each square moving up (^),
/// down (v), left (<), or right (>). The location that should get the best signal is still E, and .
/// marks unvisited squares.
///
/// This path reaches the goal in 31 steps, the fewest possible.
///
/// What is the fewest steps required to move from your current position to the location that should
/// get the best signal?
pub fn day_12() {
    let data = load_file(12);

    let data_as_lines = data.trim().split('\n');
    let line_count = data_as_lines.clone().count();

    let mut elevation_map: Vec<i32> = vec![];

    #[derive(PartialEq, Eq, Hash, Clone, Copy, Default, Debug)]
    struct Pos {
        pub x: usize,
        pub y: usize,
    }

    const VERBOSE: bool = true;

    macro_rules! vprint {
        ($($x:tt)*) => { if VERBOSE { println!($($x)*); } }
    }

    let mut start_pos = Pos { x: 0, y: 0 };
    let mut goal_pos = Pos { x: 0, y: 0 };

    for (line_idx, line) in data_as_lines.enumerate() {
        for (column_idx, c) in line.char_indices() {
            match c {
                'S' => {
                    start_pos.x = column_idx;
                    start_pos.y = line_idx;
                    elevation_map.push(0);
                }
                'E' => {
                    goal_pos.x = column_idx;
                    goal_pos.y = line_idx;
                    elevation_map.push(25);
                }
                c => {
                    assert!(c.is_ascii_lowercase());
                    let elevation = c as u8 - b'a';
                    let elevation: i32 = elevation.into();
                    elevation_map.push(elevation);
                }
            }
        }
    }

    let column_count = elevation_map.len() / line_count;

    let get_neighbours = |current: &Pos| -> Vec<Pos> {
        let mut neighbours: Vec<Pos> = vec![];
        let curr_x = current.x;
        let curr_y = current.y;
        if curr_x > 0 {
            neighbours.push(Pos {
                x: curr_x - 1,
                y: curr_y,
            });
        }
        if curr_x < column_count - 1 {
            neighbours.push(Pos {
                x: curr_x + 1,
                y: curr_y,
            });
        }
        if curr_y > 0 {
            neighbours.push(Pos {
                x: curr_x,
                y: curr_y - 1,
            });
        }
        if curr_y < line_count - 1 {
            neighbours.push(Pos {
                x: curr_x,
                y: curr_y + 1,
            });
        }

        let can_go_to = |from: &Pos, to: &Pos| -> bool {
            let from_idx = from.x + from.y * column_count;
            let from_elevation = elevation_map[from_idx];

            let to_idx = to.x + to.y * column_count;
            let to_elevation = elevation_map[to_idx];

            let elevation_diff = to_elevation - from_elevation;

            elevation_diff <= 1
        };

        let neighbours = neighbours
            .iter()
            .filter(|&x| can_go_to(current, x))
            .map(|x| x.to_owned())
            .collect();

        neighbours
    };

    fn reconstruct_path(
        came_from: &mut std::collections::HashMap<Pos, Pos>,
        mut current: Pos,
    ) -> Vec<Pos> {
        let mut total_path = vec![current];

        while let Some(&from) = came_from.get(&current) {
            total_path.push(from);
            current = from;
        }

        total_path.reverse();
        total_path
    }

    fn h(from: Pos, to: Pos) -> usize {
        from.x.abs_diff(to.x) + from.y.abs_diff(to.y)
    }

    // Let's use smarter people algorithms
    let a_star = |start: Pos, goal: Pos| -> Option<Vec<Pos>> {
        let mut counter = 0usize;

        let mut visualization = vec!['.'; elevation_map.len()];

        let mut open_set = std::collections::HashSet::from([start]);
        let mut came_from = std::collections::HashMap::<Pos, Pos, _>::new();
        let mut distance_travelled = std::collections::HashMap::<Pos, usize, _>::new();
        distance_travelled.insert(start, 0);

        let mut estimated_distance = std::collections::HashMap::<Pos, usize, _>::new();
        estimated_distance.insert(start, h(start, goal));

        while !open_set.is_empty() {
            vprint!("Counter: {counter}");
            counter += 1;

            let current =
                estimated_distance
                    .iter()
                    .fold((Pos::default(), usize::MAX), |curr, x| {
                        if !open_set.contains(x.0) {
                            return curr;
                        }

                        if *x.1 < curr.1 {
                            (x.0.to_owned(), *x.1)
                        } else {
                            curr
                        }
                    });

            let current = current.0;

            vprint!("Processing: {current:?}");

            visualization[current.x + current.y * column_count] = '#';

            let newline = ['\n'];
            let display: String = visualization
                .chunks(column_count)
                .flat_map(|x| x.iter().chain(newline.iter()))
                .collect();

            vprint!("\n\n{display}\n\n");

            if current == goal {
                return Some(reconstruct_path(&mut came_from, current));
            }

            open_set.remove(&current);

            let neighbours = get_neighbours(&current);

            for neighbour in neighbours {
                let tentative_distance_travelled = *distance_travelled.get(&current).unwrap() + 1;

                let neigh_distance = distance_travelled.entry(neighbour).or_insert(usize::MAX);
                if tentative_distance_travelled < *neigh_distance {
                    came_from.insert(neighbour, current);
                    *neigh_distance = tentative_distance_travelled;

                    let estimated_total_distance =
                        tentative_distance_travelled + h(neighbour, goal);
                    estimated_distance.insert(neighbour, estimated_total_distance);

                    open_set.insert(neighbour);
                }
            }
        }

        None
    };

    let result = a_star(start_pos, goal_pos).unwrap();

    // -1 to remove starting node
    println!("Part 1: {}", result.len() - 1)
}
