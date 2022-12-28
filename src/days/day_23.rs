use super::load_file;

/// --- Day 23: Unstable Diffusion ---
/// You enter a large crater of gray dirt where the grove is supposed to be. All around you, plants
/// you imagine were expected to be full of fruit are instead withered and broken. A large group of
/// Elves has formed in the middle of the grove.
///
/// "...but this volcano has been dormant for months. Without ash, the fruit can't grow!"
///
/// You look up to see a massive, snow-capped mountain towering above you.
///
/// "It's not like there are other active volcanoes here; we've looked everywhere."
///
/// "But our scanners show active magma flows; clearly it's going somewhere."
///
/// They finally notice you at the edge of the grove, your pack almost overflowing from the random
/// star fruit you've been collecting. Behind you, elephants and monkeys explore the grove, looking
/// concerned. Then, the Elves recognize the ash cloud slowly spreading above your recent detour.
///
/// "Why do you--" "How is--" "Did you just--"
///
/// Before any of them can form a complete question, another Elf speaks up: "Okay, new plan. We have
/// almost enough fruit already, and ash from the plume should spread here eventually. If we quickly
/// plant new seedlings now, we can still make it to the extraction point. Spread out!"
///
/// The Elves each reach into their pack and pull out a tiny plant. The plants rely on important
/// nutrients from the ash, so they can't be planted too close together.
///
/// There isn't enough time to let the Elves figure out where to plant the seedlings themselves; you
/// quickly scan the grove (your puzzle input) and note their positions.
///
/// For example:
///
/// ....#..
/// ..###.#
/// #...#.#
/// .#...##
/// #.###..
/// ##.#.##
/// .#..#..
/// The scan shows Elves # and empty ground .; outside your scan, more empty ground extends a long
/// way in every direction. The scan is oriented so that north is up; orthogonal directions are
/// written N (north), S (south), W (west), and E (east), while diagonal directions are written NE,
/// NW, SE, SW.
///
/// The Elves follow a time-consuming process to figure out where they should each go; you can speed
/// up this process considerably. The process consists of some number of rounds during which Elves
/// alternate between considering where to move and actually moving.
///
/// During the first half of each round, each Elf considers the eight positions adjacent to
/// themself. If no other Elves are in one of those eight positions, the Elf does not do anything
/// during this round. Otherwise, the Elf looks in each of four directions in the following order
/// and proposes moving one step in the first valid direction:
///
/// If there is no Elf in the N, NE, or NW adjacent positions, the Elf proposes moving north one
/// step. If there is no Elf in the S, SE, or SW adjacent positions, the Elf proposes moving south
/// one step. If there is no Elf in the W, NW, or SW adjacent positions, the Elf proposes moving
/// west one step. If there is no Elf in the E, NE, or SE adjacent positions, the Elf proposes
/// moving east one step. After each Elf has had a chance to propose a move, the second half of the
/// round can begin. Simultaneously, each Elf moves to their proposed destination tile if they were
/// the only Elf to propose moving to that position. If two or more Elves propose moving to the same
/// position, none of those Elves move.
///
/// Finally, at the end of the round, the first direction the Elves considered is moved to the end
/// of the list of directions. For example, during the second round, the Elves would try proposing a
/// move to the south first, then west, then east, then north. On the third round, the Elves would
/// first consider west, then east, then north, then south.
///
/// As a smaller example, consider just these five Elves:
///
/// .....
/// ..##.
/// ..#..
/// .....
/// ..##.
/// .....
/// The northernmost two Elves and southernmost two Elves all propose moving north, while the middle
/// Elf cannot move north and proposes moving south. The middle Elf proposes the same destination as
/// the southwest Elf, so neither of them move, but the other three do:
///
/// ..##.
/// .....
/// ..#..
/// ...#.
/// ..#..
/// .....
/// Next, the northernmost two Elves and the southernmost Elf all propose moving south. Of the
/// remaining middle two Elves, the west one cannot move south and proposes moving west, while the
/// east one cannot move south or west and proposes moving east. All five Elves succeed in moving to
/// their proposed positions:
///
/// .....
/// ..##.
/// .#...
/// ....#
/// .....
/// ..#..
/// Finally, the southernmost two Elves choose not to move at all. Of the remaining three Elves, the
/// west one proposes moving west, the east one proposes moving east, and the middle one proposes
/// moving north; all three succeed in moving:
///
/// ..#..
/// ....#
/// #....
/// ....#
/// .....
/// ..#..
/// At this point, no Elves need to move, and so the process ends.
///
/// The larger example above proceeds as follows:
///
/// == Initial State ==
/// ..............
/// ..............
/// .......#......
/// .....###.#....
/// ...#...#.#....
/// ....#...##....
/// ...#.###......
/// ...##.#.##....
/// ....#..#......
/// ..............
/// ..............
/// ..............
///
/// == End of Round 1 ==
/// ..............
/// .......#......
/// .....#...#....
/// ...#..#.#.....
/// .......#..#...
/// ....#.#.##....
/// ..#..#.#......
/// ..#.#.#.##....
/// ..............
/// ....#..#......
/// ..............
/// ..............
///
/// == End of Round 2 ==
/// ..............
/// .......#......
/// ....#.....#...
/// ...#..#.#.....
/// .......#...#..
/// ...#..#.#.....
/// .#...#.#.#....
/// ..............
/// ..#.#.#.##....
/// ....#..#......
/// ..............
/// ..............
///
/// == End of Round 3 ==
/// ..............
/// .......#......
/// .....#....#...
/// ..#..#...#....
/// .......#...#..
/// ...#..#.#.....
/// .#..#.....#...
/// .......##.....
/// ..##.#....#...
/// ...#..........
/// .......#......
/// ..............
///
/// == End of Round 4 ==
/// ..............
/// .......#......
/// ......#....#..
/// ..#...##......
/// ...#.....#.#..
/// .........#....
/// .#...###..#...
/// ..#......#....
/// ....##....#...
/// ....#.........
/// .......#......
/// ..............
///
/// == End of Round 5 ==
/// .......#......
/// ..............
/// ..#..#.....#..
/// .........#....
/// ......##...#..
/// .#.#.####.....
/// ...........#..
/// ....##..#.....
/// ..#...........
/// ..........#...
/// ....#..#......
/// ..............
/// After a few more rounds...
///
/// == End of Round 10 ==
/// .......#......
/// ...........#..
/// ..#.#..#......
/// ......#.......
/// ...#.....#..#.
/// .#......##....
/// .....##.......
/// ..#........#..
/// ....#.#..#....
/// ..............
/// ....#..#..#...
/// ..............
/// To make sure they're on the right track, the Elves like to check after round 10 that they're
/// making good progress toward covering enough ground. To do this, count the number of empty ground
/// tiles contained by the smallest rectangle that contains every Elf. (The edges of the rectangle
/// should be aligned to the N/S/E/W directions; the Elves do not have the patience to calculate
/// arbitrary rectangles.) In the above example, that rectangle is:
///
/// ......#.....
/// ..........#.
/// .#.#..#.....
/// .....#......
/// ..#.....#..#
/// #......##...
/// ....##......
/// .#........#.
/// ...#.#..#...
/// ............
/// ...#..#..#..
/// In this region, the number of empty ground tiles is 110.
///
/// Simulate the Elves' process and find the smallest rectangle that contains the Elves after 10
/// rounds. How many empty ground tiles does that rectangle contain?
///
/// --- Part Two ---
/// It seems you're on the right track. Finish simulating the process and figure out where the Elves
/// need to go. How many rounds did you save them?
///
/// In the example above, the first round where no Elf moved was round 20:
///
/// .......#......
/// ....#......#..
/// ..#.....#.....
/// ......#.......
/// ...#....#.#..#
/// #.............
/// ....#.....#...
/// ..#.....#.....
/// ....#.#....#..
/// .........#....
/// ....#......#..
/// .......#......
/// Figure out where the Elves need to go. What is the number of the first round where no Elf moves?
pub fn day_23() {
    let data = load_file(23);

    let data_as_lines = data.trim().split('\n');

    use std::collections::{HashMap, HashSet};

    // (line, col)
    const NORTH_WEST: (i64, i64) = (-1, -1);
    const NORTH: (i64, i64) = (-1, 0);
    const NORTH_EAST: (i64, i64) = (-1, 1);
    const EAST: (i64, i64) = (0, 1);
    const SOUTH_EAST: (i64, i64) = (1, 1);
    const SOUTH: (i64, i64) = (1, 0);
    const SOUTH_WEST: (i64, i64) = (1, -1);
    const WEST: (i64, i64) = (0, -1);

    type ZoneCheck = [(i64, i64); 3];

    // (check_zone, move)
    const MOVE_CYCLE: [(ZoneCheck, (i64, i64)); 4] = [
        ([NORTH_WEST, NORTH, NORTH_EAST], NORTH),
        ([SOUTH_WEST, SOUTH, SOUTH_EAST], SOUTH),
        ([NORTH_WEST, WEST, SOUTH_WEST], WEST),
        ([NORTH_EAST, EAST, SOUTH_EAST], EAST),
    ];

    let mut elves: HashSet<(i64, i64)> = Default::default();

    for (line_idx, line) in data_as_lines.enumerate() {
        for (col_idx, c) in line.trim().chars().enumerate() {
            if c == '#' {
                elves.insert((line_idx.try_into().unwrap(), col_idx.try_into().unwrap()));
            }
        }
    }

    // Destination, Vec of elves wanting to move there
    let mut planned_moves: HashMap<(i64, i64), Vec<(i64, i64)>> = Default::default();

    #[allow(dead_code)]
    fn debug_print(elves: &HashSet<(i64, i64)>) {
        let (min_line, max_line, min_col, max_col) =
            elves
                .iter()
                .fold((i64::MAX, i64::MIN, i64::MAX, i64::MIN), |acc, x| {
                    let (min_line, max_line, min_col, max_col) = acc;
                    let &(line, col) = x;
                    (
                        min_line.min(line),
                        max_line.max(line),
                        min_col.min(col),
                        max_col.max(col),
                    )
                });

        for line in min_line..=max_line {
            for col in min_col..=max_col {
                if elves.contains(&(line, col)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!()
    }

    // debug_print(&elves);

    let mut part_1_elves = elves.clone();

    for round_idx in 0..10 {
        let first_direction_idx = round_idx % MOVE_CYCLE.len();

        for &(line, col) in part_1_elves.iter() {
            let mut should_move = false;
            for (diff_line, diff_col) in [
                NORTH_WEST, NORTH, NORTH_EAST, EAST, SOUTH_EAST, SOUTH, SOUTH_WEST, WEST,
            ] {
                let dst = (line + diff_line, col + diff_col);
                if part_1_elves.get(&dst).is_some() {
                    should_move = true;
                    break;
                }
            }

            if !should_move {
                continue;
            }

            for move_diff in 0..4 {
                let move_idx = (first_direction_idx + move_diff) % MOVE_CYCLE.len();

                let (zone, direction) = MOVE_CYCLE[move_idx];
                let mut can_move = true;
                for (diff_line, diff_col) in zone {
                    let check_location = (line + diff_line, col + diff_col);
                    if part_1_elves.contains(&check_location) {
                        can_move = false;
                        break;
                    }
                }

                if can_move {
                    let move_location = (line + direction.0, col + direction.1);
                    if let Some(planned_move) = planned_moves.get_mut(&move_location) {
                        planned_move.push((line, col));
                    } else {
                        planned_moves.insert(move_location, vec![(line, col)]);
                    }
                    break;
                }
            }
        }

        for (planned_move, elves_moving) in planned_moves.drain() {
            if elves_moving.len() > 1 {
                continue;
            }

            part_1_elves.remove(&elves_moving[0]);
            part_1_elves.insert(planned_move);
        }
        // debug_print(&part_1_elves);
    }

    // debug_print(&part_1_elves);

    let (min_line, max_line, min_col, max_col) =
        part_1_elves
            .iter()
            .fold((i64::MAX, i64::MIN, i64::MAX, i64::MIN), |acc, x| {
                let (min_line, max_line, min_col, max_col) = acc;
                let &(line, col) = x;
                (
                    min_line.min(line),
                    max_line.max(line),
                    min_col.min(col),
                    max_col.max(col),
                )
            });

    // println!("{:?}", (min_line, max_line, min_col, max_col));

    let terrain_area = (max_line - min_line + 1) * (max_col - min_col + 1);

    // println!("{terrain_area}");

    let free_terrain = terrain_area - elves.len() as i64;

    println!("Part 1: {free_terrain}");

    let mut part_2_elves = elves;
    let mut part_2_res = 0;

    for round_idx in 0.. {
        let first_direction_idx = round_idx % MOVE_CYCLE.len();

        for &(line, col) in part_2_elves.iter() {
            let mut should_move = false;
            for (diff_line, diff_col) in [
                NORTH_WEST, NORTH, NORTH_EAST, EAST, SOUTH_EAST, SOUTH, SOUTH_WEST, WEST,
            ] {
                let dst = (line + diff_line, col + diff_col);
                if part_2_elves.get(&dst).is_some() {
                    should_move = true;
                    break;
                }
            }

            if !should_move {
                continue;
            }

            for move_diff in 0..4 {
                let move_idx = (first_direction_idx + move_diff) % MOVE_CYCLE.len();

                let (zone, direction) = MOVE_CYCLE[move_idx];
                let mut can_move = true;
                for (diff_line, diff_col) in zone {
                    let check_location = (line + diff_line, col + diff_col);
                    if part_2_elves.contains(&check_location) {
                        can_move = false;
                        break;
                    }
                }

                if can_move {
                    let move_location = (line + direction.0, col + direction.1);
                    if let Some(planned_move) = planned_moves.get_mut(&move_location) {
                        planned_move.push((line, col));
                    } else {
                        planned_moves.insert(move_location, vec![(line, col)]);
                    }
                    break;
                }
            }
        }

        if planned_moves.is_empty() {
            part_2_res = round_idx + 1;
            break;
        }

        for (planned_move, elves_moving) in planned_moves.drain() {
            if elves_moving.len() > 1 {
                continue;
            }

            part_2_elves.remove(&elves_moving[0]);
            part_2_elves.insert(planned_move);
        }
        // debug_print(&part_2_elves);
    }

    println!("Part 2: {part_2_res}");
}
