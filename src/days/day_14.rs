use super::load_file;

/// --- Day 14: Regolith Reservoir ---
/// The distress signal leads you to a giant waterfall! Actually, hang on - the signal seems like
/// it's coming from the waterfall itself, and that doesn't make any sense. However, you do notice a
/// little path that leads behind the waterfall.
///
/// Correction: the distress signal leads you behind a giant waterfall! There seems to be a large
/// cave system here, and the signal definitely leads further inside.
///
/// As you begin to make your way deeper underground, you feel the ground rumble for a moment. Sand
/// begins pouring into the cave! If you don't quickly figure out where the sand is going, you could
/// quickly become trapped!
///
/// Fortunately, your familiarity with analyzing the path of falling material will come in handy
/// here. You scan a two-dimensional vertical slice of the cave above you (your puzzle input) and
/// discover that it is mostly air with structures made of rock.
///
/// Your scan traces the path of each solid rock structure and reports the x,y coordinates that form
/// the shape of the path, where x represents distance to the right and y represents distance down.
/// Each path appears as a single line of text in your scan. After the first point of each path,
/// each point indicates the end of a straight horizontal or vertical line to be drawn from the
/// previous point. For example:
///
/// 498,4 -> 498,6 -> 496,6
/// 503,4 -> 502,4 -> 502,9 -> 494,9
/// This scan means that there are two paths of rock; the first path consists of two straight lines,
/// and the second path consists of three straight lines. (Specifically, the first path consists of
/// a line of rock from 498,4 through 498,6 and another line of rock from 498,6 through 496,6.)
///
/// The sand is pouring into the cave from point 500,0.
///
/// Drawing rock as #, air as ., and the source of the sand as +, this becomes:
///
///
///   4     5  5
///   9     0  0
///   4     0  3
/// 0 ......+...
/// 1 ..........
/// 2 ..........
/// 3 ..........
/// 4 ....#...##
/// 5 ....#...#.
/// 6 ..###...#.
/// 7 ........#.
/// 8 ........#.
/// 9 #########.
/// Sand is produced one unit at a time, and the next unit of sand is not produced until the
/// previous unit of sand comes to rest. A unit of sand is large enough to fill one tile of air in
/// your scan.
///
/// A unit of sand always falls down one step if possible. If the tile immediately below is blocked
/// (by rock or sand), the unit of sand attempts to instead move diagonally one step down and to the
/// left. If that tile is blocked, the unit of sand attempts to instead move diagonally one step
/// down and to the right. Sand keeps moving as long as it is able to do so, at each step trying to
/// move down, then down-left, then down-right. If all three possible destinations are blocked, the
/// unit of sand comes to rest and no longer moves, at which point the next unit of sand is created
/// back at the source.
///
/// So, drawing sand that has come to rest as o, the first unit of sand simply falls straight down
/// and then stops:
///
/// ......+...
/// ..........
/// ..........
/// ..........
/// ....#...##
/// ....#...#.
/// ..###...#.
/// ........#.
/// ......o.#.
/// #########.
/// The second unit of sand then falls straight down, lands on the first one, and then comes to rest
/// to its left:
///
/// ......+...
/// ..........
/// ..........
/// ..........
/// ....#...##
/// ....#...#.
/// ..###...#.
/// ........#.
/// .....oo.#.
/// #########.
/// After a total of five units of sand have come to rest, they form this pattern:
///
/// ......+...
/// ..........
/// ..........
/// ..........
/// ....#...##
/// ....#...#.
/// ..###...#.
/// ......o.#.
/// ....oooo#.
/// #########.
/// After a total of 22 units of sand:
///
/// ......+...
/// ..........
/// ......o...
/// .....ooo..
/// ....#ooo##
/// ....#ooo#.
/// ..###ooo#.
/// ....oooo#.
/// ...ooooo#.
/// #########.
/// Finally, only two more units of sand can possibly come to rest:
///
/// ......+...
/// ..........
/// ......o...
/// .....ooo..
/// ....#ooo##
/// ...o#ooo#.
/// ..###ooo#.
/// ....oooo#.
/// .o.ooooo#.
/// #########.
/// Once all 24 units of sand shown above have come to rest, all further sand flows out the bottom,
/// falling into the endless void. Just for fun, the path any new sand takes before falling forever
/// is shown here with ~:
///
/// .......+...
/// .......~...
/// ......~o...
/// .....~ooo..
/// ....~#ooo##
/// ...~o#ooo#.
/// ..~###ooo#.
/// ..~..oooo#.
/// .~o.ooooo#.
/// ~#########.
/// ~..........
/// ~..........
/// ~..........
/// Using your scan, simulate the falling sand. How many units of sand come to rest before sand
/// starts flowing into the abyss below?
pub fn day_14() {
    let data = load_file(14);

    let data_as_lines = data.trim().split('\n');

    use std::str::FromStr;

    #[derive(Debug, Clone, Copy)]
    struct Pos {
        x: i32,
        y: i32,
    }

    impl Pos {
        pub fn get_idx(&self, w: i32) -> usize {
            (self.x + self.y * w).try_into().unwrap()
        }

        pub fn inside(&self, w: i32, h: i32) -> bool {
            self.x >= 0 && self.x < w && self.y >= 0 && self.y < h
        }
    }

    impl FromStr for Pos {
        type Err = Box<dyn std::error::Error>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (x, y) = s.trim().split_once(',').unwrap();

            Ok(Self {
                x: x.parse()?,
                y: y.parse()?,
            })
        }
    }

    #[derive(Debug, Clone)]
    struct RockPath {
        pub path: Vec<Pos>,
    }

    impl IntoIterator for RockPath {
        type Item = Pos;
        type IntoIter = <Vec<Pos> as IntoIterator>::IntoIter;
        fn into_iter(self) -> Self::IntoIter {
            self.path.into_iter()
        }
    }

    impl FromStr for RockPath {
        type Err = Box<dyn std::error::Error>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let path: Vec<Pos> = s.trim().split("->").map(|x| x.parse().unwrap()).collect();

            Ok(Self { path })
        }
    }

    let mut rock_paths: Vec<RockPath> = data_as_lines.map(|x| x.parse().unwrap()).collect();

    let ((min_x, max_x), (_, max_y)) = rock_paths.clone().into_iter().flatten().fold(
        ((i32::MAX, i32::MIN), (i32::MAX, i32::MIN)),
        |acc, pos| {
            let ((mut min_x, mut max_x), (mut min_y, mut max_y)) = acc;

            if pos.x < min_x {
                min_x = pos.x;
            } else if pos.x > max_x {
                max_x = pos.x;
            }

            if pos.y < min_y {
                min_y = pos.y;
            } else if pos.y > max_y {
                max_y = pos.y;
            }

            ((min_x, max_x), (min_y, max_y))
        },
    );

    let start = Pos {
        x: 500 - min_x,
        y: 0,
    };

    let w = max_x - min_x + 1;
    let h = max_y + 1;

    let mut cave = vec!['.'; (w * h).try_into().unwrap()];

    rock_paths
        .iter_mut()
        .for_each(|path| path.path.iter_mut().for_each(|pos| pos.x -= min_x));

    let rock_paths = rock_paths;

    for rock_path in rock_paths {
        let mut rock_path_iter = rock_path.into_iter();
        let mut start = rock_path_iter.next().unwrap();

        for next in rock_path_iter {
            let diff_x = next.x - start.x;
            let diff_y = next.y - start.y;

            if diff_x == 0 {
                let x = start.x;
                let mut y = start.y;
                let y_signum = diff_y.signum();
                for _ in 0..=diff_y.abs() {
                    let cave_idx: usize = (x + y * w).try_into().unwrap();
                    cave[cave_idx] = '#';
                    // let newline = ['\n'];
                    // let display: String = cave
                    //     .chunks(w.try_into().unwrap())
                    //     .flat_map(|x| x.iter().chain(newline.iter()))
                    //     .collect();

                    // println!("\n\n{display}\n\n");
                    y += y_signum;
                }
            } else if diff_y == 0 {
                let mut x = start.x;
                let y = start.y;
                let x_signum = diff_x.signum();
                for _ in 0..=diff_x.abs() {
                    let cave_idx: usize = (x + y * w).try_into().unwrap();
                    cave[cave_idx] = '#';
                    // let newline = ['\n'];
                    // let display: String = cave
                    //     .chunks(w.try_into().unwrap())
                    //     .flat_map(|x| x.iter().chain(newline.iter()))
                    //     .collect();

                    // println!("\n\n{display}\n\n");
                    x += x_signum;
                }
            } else {
                unreachable!()
            }

            start = next;
        }
    }

    // let newline = ['\n'];
    // let display: String = cave
    //     .chunks(w.try_into().unwrap())
    //     .flat_map(|x| x.iter().chain(newline.iter()))
    //     .collect();

    // println!("\n\n{display}\n\n");

    let mut sand_count = 0;

    'outer: loop {
        let mut sand_pos = start;

        loop {
            // try to fall down
            let next_position = Pos {
                x: sand_pos.x,
                y: sand_pos.y + 1,
            };

            if !next_position.inside(w, h) {
                break 'outer;
            }

            let next_position_idx = next_position.get_idx(w);

            match cave[next_position_idx] {
                '.' => {
                    sand_pos = next_position;
                }
                _ => {
                    let next_position = Pos {
                        x: sand_pos.x - 1,
                        y: sand_pos.y + 1,
                    };

                    if !next_position.inside(w, h) {
                        break 'outer;
                    }

                    let next_position_idx = next_position.get_idx(w);
                    match cave[next_position_idx] {
                        '.' => {
                            sand_pos = next_position;
                        }
                        _ => {
                            let next_position = Pos {
                                x: sand_pos.x + 1,
                                y: sand_pos.y + 1,
                            };

                            if !next_position.inside(w, h) {
                                break 'outer;
                            }

                            let next_position_idx = next_position.get_idx(w);

                            match cave[next_position_idx] {
                                '.' => {
                                    sand_pos = next_position;
                                }
                                _ => {
                                    cave[sand_pos.get_idx(w)] = 'o';
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }

        // let newline = ['\n'];
        // let display: String = cave
        //     .chunks(w.try_into().unwrap())
        //     .flat_map(|x| x.iter().chain(newline.iter()))
        //     .collect();

        // println!("\n\n{display}\n\n");
        sand_count += 1;
    }

    println!("Part 1: {sand_count}");
}
