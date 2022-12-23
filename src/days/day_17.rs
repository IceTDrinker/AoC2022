use super::load_file;

/// --- Day 17: Pyroclastic Flow ---
/// Your handheld device has located an alternative exit from the cave for you and the elephants.
/// The ground is rumbling almost continuously now, but the strange valves bought you some time.
/// It's definitely getting warmer in here, though.
///
/// The tunnels eventually open into a very tall, narrow chamber. Large, oddly-shaped rocks are
/// falling into the chamber from above, presumably due to all the rumbling. If you can't work out
/// where the rocks will fall next, you might be crushed!
///
/// The five types of rocks have the following peculiar shapes, where # is rock and . is empty
/// space:
///
/// ####
///
/// .#.
/// ###
/// .#.
///
/// ..#
/// ..#
/// ###
///
/// #
/// #
/// #
/// #
///
/// ##
/// ##
/// The rocks fall in the order shown above: first the - shape, then the + shape, and so on. Once
/// the end of the list is reached, the same order repeats: the - shape falls first, sixth, 11th,
/// 16th, etc.
///
/// The rocks don't spin, but they do get pushed around by jets of hot gas coming out of the walls
/// themselves. A quick scan reveals the effect the jets of hot gas will have on the rocks as they
/// fall (your puzzle input).
///
/// For example, suppose this was the jet pattern in your cave:
///
/// >>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
/// In jet patterns, < means a push to the left, while > means a push to the right. The pattern
/// above means that the jets will push a falling rock right, then right, then right, then left,
/// then left, then right, and so on. If the end of the list is reached, it repeats.
///
/// The tall, vertical chamber is exactly seven units wide. Each rock appears so that its left edge
/// is two units away from the left wall and its bottom edge is three units above the highest rock
/// in the room (or the floor, if there isn't one).
///
/// After a rock appears, it alternates between being pushed by a jet of hot gas one unit (in the
/// direction indicated by the next symbol in the jet pattern) and then falling one unit down. If
/// any movement would cause any part of the rock to move into the walls, floor, or a stopped rock,
/// the movement instead does not occur. If a downward movement would have caused a falling rock to
/// move into the floor or an already-fallen rock, the falling rock stops where it is (having landed
/// on something) and a new rock immediately begins falling.
///
/// Drawing falling rocks with @ and stopped rocks with #, the jet pattern in the example above
/// manifests as follows:
///
/// The first rock begins falling:
/// |..@@@@.|
/// |.......|
/// |.......|
/// |.......|
/// +-------+
///
/// Jet of gas pushes rock right:
/// |...@@@@|
/// |.......|
/// |.......|
/// |.......|
/// +-------+
///
/// Rock falls 1 unit:
/// |...@@@@|
/// |.......|
/// |.......|
/// +-------+
///
/// Jet of gas pushes rock right, but nothing happens:
/// |...@@@@|
/// |.......|
/// |.......|
/// +-------+
///
/// Rock falls 1 unit:
/// |...@@@@|
/// |.......|
/// +-------+
///
/// Jet of gas pushes rock right, but nothing happens:
/// |...@@@@|
/// |.......|
/// +-------+
///
/// Rock falls 1 unit:
/// |...@@@@|
/// +-------+
///
/// Jet of gas pushes rock left:
/// |..@@@@.|
/// +-------+
///
/// Rock falls 1 unit, causing it to come to rest:
/// |..####.|
/// +-------+
///
/// A new rock begins falling:
/// |...@...|
/// |..@@@..|
/// |...@...|
/// |.......|
/// |.......|
/// |.......|
/// |..####.|
/// +-------+
///
/// Jet of gas pushes rock left:
/// |..@....|
/// |.@@@...|
/// |..@....|
/// |.......|
/// |.......|
/// |.......|
/// |..####.|
/// +-------+
///
/// Rock falls 1 unit:
/// |..@....|
/// |.@@@...|
/// |..@....|
/// |.......|
/// |.......|
/// |..####.|
/// +-------+
///
/// Jet of gas pushes rock right:
/// |...@...|
/// |..@@@..|
/// |...@...|
/// |.......|
/// |.......|
/// |..####.|
/// +-------+
///
/// Rock falls 1 unit:
/// |...@...|
/// |..@@@..|
/// |...@...|
/// |.......|
/// |..####.|
/// +-------+
///
/// Jet of gas pushes rock left:
/// |..@....|
/// |.@@@...|
/// |..@....|
/// |.......|
/// |..####.|
/// +-------+
///
/// Rock falls 1 unit:
/// |..@....|
/// |.@@@...|
/// |..@....|
/// |..####.|
/// +-------+
///
/// Jet of gas pushes rock right:
/// |...@...|
/// |..@@@..|
/// |...@...|
/// |..####.|
/// +-------+
///
/// Rock falls 1 unit, causing it to come to rest:
/// |...#...|
/// |..###..|
/// |...#...|
/// |..####.|
/// +-------+
///
/// A new rock begins falling:
/// |....@..|
/// |....@..|
/// |..@@@..|
/// |.......|
/// |.......|
/// |.......|
/// |...#...|
/// |..###..|
/// |...#...|
/// |..####.|
/// +-------+
/// The moment each of the next few rocks begins falling, you would see this:
///
/// |..@....|
/// |..@....|
/// |..@....|
/// |..@....|
/// |.......|
/// |.......|
/// |.......|
/// |..#....|
/// |..#....|
/// |####...|
/// |..###..|
/// |...#...|
/// |..####.|
/// +-------+
///
/// |..@@...|
/// |..@@...|
/// |.......|
/// |.......|
/// |.......|
/// |....#..|
/// |..#.#..|
/// |..#.#..|
/// |#####..|
/// |..###..|
/// |...#...|
/// |..####.|
/// +-------+
///
/// |..@@@@.|
/// |.......|
/// |.......|
/// |.......|
/// |....##.|
/// |....##.|
/// |....#..|
/// |..#.#..|
/// |..#.#..|
/// |#####..|
/// |..###..|
/// |...#...|
/// |..####.|
/// +-------+
///
/// |...@...|
/// |..@@@..|
/// |...@...|
/// |.......|
/// |.......|
/// |.......|
/// |.####..|
/// |....##.|
/// |....##.|
/// |....#..|
/// |..#.#..|
/// |..#.#..|
/// |#####..|
/// |..###..|
/// |...#...|
/// |..####.|
/// +-------+
///
/// |....@..|
/// |....@..|
/// |..@@@..|
/// |.......|
/// |.......|
/// |.......|
/// |..#....|
/// |.###...|
/// |..#....|
/// |.####..|
/// |....##.|
/// |....##.|
/// |....#..|
/// |..#.#..|
/// |..#.#..|
/// |#####..|
/// |..###..|
/// |...#...|
/// |..####.|
/// +-------+
///
/// |..@....|
/// |..@....|
/// |..@....|
/// |..@....|
/// |.......|
/// |.......|
/// |.......|
/// |.....#.|
/// |.....#.|
/// |..####.|
/// |.###...|
/// |..#....|
/// |.####..|
/// |....##.|
/// |....##.|
/// |....#..|
/// |..#.#..|
/// |..#.#..|
/// |#####..|
/// |..###..|
/// |...#...|
/// |..####.|
/// +-------+
///
/// |..@@...|
/// |..@@...|
/// |.......|
/// |.......|
/// |.......|
/// |....#..|
/// |....#..|
/// |....##.|
/// |....##.|
/// |..####.|
/// |.###...|
/// |..#....|
/// |.####..|
/// |....##.|
/// |....##.|
/// |....#..|
/// |..#.#..|
/// |..#.#..|
/// |#####..|
/// |..###..|
/// |...#...|
/// |..####.|
/// +-------+
///
/// |..@@@@.|
/// |.......|
/// |.......|
/// |.......|
/// |....#..|
/// |....#..|
/// |....##.|
/// |##..##.|
/// |######.|
/// |.###...|
/// |..#....|
/// |.####..|
/// |....##.|
/// |....##.|
/// |....#..|
/// |..#.#..|
/// |..#.#..|
/// |#####..|
/// |..###..|
/// |...#...|
/// |..####.|
/// +-------+
/// To prove to the elephants your simulation is accurate, they want to know how tall the tower will
/// get after 2022 rocks have stopped (but before the 2023rd rock begins falling). In this example,
/// the tower of rocks will be 3068 units tall.
///
/// How many units tall will the tower of rocks be after 2022 rocks have stopped falling?
pub fn day_17() {
    let data = load_file(17);

    let jet_iterator = data.trim().char_indices().into_iter().cycle();

    const HORIZONTAL_LINE_CONTENT: [[bool; 4]; 1] = [[true, true, true, true]];
    const CROSS_CONTENT: [[bool; 3]; 3] = [
        [false, true, false],
        [true, true, true],
        [false, true, false],
    ];
    // Content inverted vertically as y represents the bottow corner
    const INVERTED_L_CONTENT: [[bool; 3]; 3] = [
        [true, true, true],
        [false, false, true],
        [false, false, true],
    ];
    const VERTICAL_LINE_CONTENT: [[bool; 1]; 4] = [[true], [true], [true], [true]];
    const SQUARE_CONTENT: [[bool; 2]; 2] = [[true, true], [true, true]];

    const WORLD_WIDTH: usize = 7;

    pub trait Shape {
        fn new(x: usize, y: usize) -> Self
        where
            Self: Sized;

        fn x(&self) -> usize;
        fn y(&self) -> usize;

        fn w(&self) -> usize;
        fn h(&self) -> usize;

        fn right(&self) -> usize {
            self.x() + self.w()
        }

        fn top(&self) -> usize {
            self.y() + self.h()
        }

        fn set_y(&mut self, new_y: usize);
        fn set_x(&mut self, new_x: usize);

        fn content_iter(&self) -> Box<dyn Iterator<Item = (usize, usize, bool)>>;

        fn move_down(&mut self, world: &[Vec<bool>]) -> bool {
            if self.y() == 0 {
                return false;
            }

            let new_y = self.y() - 1;

            let content_iter = self.content_iter();

            for (diff_y, diff_x, is_present) in content_iter {
                if !is_present {
                    continue;
                }

                let dst_x = self.x() + diff_x;
                let dst_y = new_y + diff_y;

                // If something is present, we can't move
                if world[dst_y][dst_x] {
                    return false;
                }
            }

            self.set_y(new_y);
            true
        }

        fn move_left(&mut self, world: &[Vec<bool>]) {
            if self.x() == 0 {
                return;
            }

            let new_x = self.x() - 1;

            let content_iter = self.content_iter();

            for (diff_y, diff_x, is_present) in content_iter {
                if !is_present {
                    continue;
                }

                let dst_x = new_x + diff_x;
                let dst_y = self.y() + diff_y;

                // If something is present, we can't move
                if world[dst_y][dst_x] {
                    return;
                }
            }

            self.set_x(new_x);
        }

        fn move_right(&mut self, world: &[Vec<bool>]) {
            if self.right() >= WORLD_WIDTH {
                return;
            }

            let new_x = self.x() + 1;

            let content_iter = self.content_iter();

            for (diff_y, diff_x, is_present) in content_iter {
                if !is_present {
                    continue;
                }

                let dst_x = new_x + diff_x;
                let dst_y = self.y() + diff_y;

                // If something is present, we can't move
                if world[dst_y][dst_x] {
                    return;
                }
            }

            self.set_x(new_x);
        }

        fn draw(&self, world: &mut [Vec<bool>]) {
            let content_iter = self.content_iter();

            for (diff_y, diff_x, is_present) in content_iter {
                if !is_present {
                    continue;
                }

                let dst_x = self.x() + diff_x;
                let dst_y = self.y() + diff_y;

                world[dst_y][dst_x] = true;
            }
        }
    }

    struct HorizontalLine {
        x: usize,
        y: usize,
    }

    impl Shape for HorizontalLine {
        fn new(x: usize, y: usize) -> Self
        where
            Self: Sized,
        {
            Self { x, y }
        }

        fn content_iter(&self) -> Box<dyn Iterator<Item = (usize, usize, bool)>> {
            Box::new(
                HORIZONTAL_LINE_CONTENT
                    .into_iter()
                    .flatten()
                    .enumerate()
                    .map(|(idx, is_present)| {
                        let row_content_len = HORIZONTAL_LINE_CONTENT[0].len();
                        let y = idx / row_content_len;
                        let x = idx % row_content_len;
                        (y, x, is_present)
                    }),
            )
        }

        fn set_x(&mut self, new_x: usize) {
            self.x = new_x
        }

        fn set_y(&mut self, new_y: usize) {
            self.y = new_y
        }

        fn x(&self) -> usize {
            self.x
        }

        fn y(&self) -> usize {
            self.y
        }

        fn h(&self) -> usize {
            1
        }

        fn w(&self) -> usize {
            4
        }
    }

    struct Cross {
        x: usize,
        y: usize,
    }

    impl Shape for Cross {
        fn new(x: usize, y: usize) -> Self
        where
            Self: Sized,
        {
            Self { x, y }
        }

        fn content_iter(&self) -> Box<dyn Iterator<Item = (usize, usize, bool)>> {
            Box::new(
                CROSS_CONTENT
                    .into_iter()
                    .flatten()
                    .enumerate()
                    .map(|(idx, is_present)| {
                        let row_content_len = CROSS_CONTENT[0].len();
                        let y = idx / row_content_len;
                        let x = idx % row_content_len;
                        (y, x, is_present)
                    }),
            )
        }

        fn set_x(&mut self, new_x: usize) {
            self.x = new_x
        }

        fn set_y(&mut self, new_y: usize) {
            self.y = new_y
        }

        fn x(&self) -> usize {
            self.x
        }

        fn y(&self) -> usize {
            self.y
        }

        fn h(&self) -> usize {
            3
        }

        fn w(&self) -> usize {
            3
        }
    }

    struct InvertedL {
        x: usize,
        y: usize,
    }

    impl Shape for InvertedL {
        fn new(x: usize, y: usize) -> Self
        where
            Self: Sized,
        {
            Self { x, y }
        }

        fn content_iter(&self) -> Box<dyn Iterator<Item = (usize, usize, bool)>> {
            Box::new(INVERTED_L_CONTENT.into_iter().flatten().enumerate().map(
                |(idx, is_present)| {
                    let row_content_len = INVERTED_L_CONTENT[0].len();
                    let y = idx / row_content_len;
                    let x = idx % row_content_len;
                    (y, x, is_present)
                },
            ))
        }

        fn set_x(&mut self, new_x: usize) {
            self.x = new_x
        }

        fn set_y(&mut self, new_y: usize) {
            self.y = new_y
        }

        fn x(&self) -> usize {
            self.x
        }

        fn y(&self) -> usize {
            self.y
        }

        fn h(&self) -> usize {
            3
        }

        fn w(&self) -> usize {
            3
        }
    }

    struct VerticalLine {
        x: usize,
        y: usize,
    }

    impl Shape for VerticalLine {
        fn new(x: usize, y: usize) -> Self
        where
            Self: Sized,
        {
            Self { x, y }
        }

        fn content_iter(&self) -> Box<dyn Iterator<Item = (usize, usize, bool)>> {
            Box::new(VERTICAL_LINE_CONTENT.into_iter().flatten().enumerate().map(
                |(idx, is_present)| {
                    let row_content_len = VERTICAL_LINE_CONTENT[0].len();
                    let y = idx / row_content_len;
                    let x = idx % row_content_len;
                    (y, x, is_present)
                },
            ))
        }

        fn set_x(&mut self, new_x: usize) {
            self.x = new_x
        }

        fn set_y(&mut self, new_y: usize) {
            self.y = new_y
        }

        fn x(&self) -> usize {
            self.x
        }

        fn y(&self) -> usize {
            self.y
        }

        fn h(&self) -> usize {
            4
        }

        fn w(&self) -> usize {
            1
        }
    }

    struct Square {
        x: usize,
        y: usize,
    }

    impl Shape for Square {
        fn new(x: usize, y: usize) -> Self
        where
            Self: Sized,
        {
            Self { x, y }
        }

        fn content_iter(&self) -> Box<dyn Iterator<Item = (usize, usize, bool)>> {
            Box::new(
                SQUARE_CONTENT
                    .into_iter()
                    .flatten()
                    .enumerate()
                    .map(|(idx, is_present)| {
                        let row_content_len = SQUARE_CONTENT[0].len();
                        let y = idx / row_content_len;
                        let x = idx % row_content_len;
                        (y, x, is_present)
                    }),
            )
        }

        fn set_x(&mut self, new_x: usize) {
            self.x = new_x
        }

        fn set_y(&mut self, new_y: usize) {
            self.y = new_y
        }

        fn x(&self) -> usize {
            self.x
        }

        fn y(&self) -> usize {
            self.y
        }

        fn h(&self) -> usize {
            2
        }

        fn w(&self) -> usize {
            2
        }
    }

    type StateChangeMap =
        std::collections::HashMap<(usize, usize, usize, [usize; WORLD_WIDTH]), (usize, usize)>;

    fn solve<const TRACK_CYCLES: bool>(
        number_of_shapes: usize,
        mut jet_iterator: std::iter::Cycle<std::str::CharIndices>,
    ) -> (usize, StateChangeMap, (usize, usize, usize, usize)) {
        let mut lowest_empty_location = 0usize;
        let mut world: Vec<Vec<bool>> = vec![];

        // (shape_idx % 5, jet_idx, height_change), (shape_idx, height)
        let mut state_change: StateChangeMap = std::collections::HashMap::new();
        let mut cycle_len = 0;
        let mut cycle_start_idx = 0;
        let mut cycle_height = 0;
        let mut cycle_start_height = 0;

        'outer: for shape_index in 0..number_of_shapes {
            // x, y represent the bottom left corner of the shape
            let shape_x = 2usize;
            let shape_y = lowest_empty_location + 3;

            let mut current_shape: Box<dyn Shape> = match shape_index % 5 {
                0 => Box::new(HorizontalLine::new(shape_x, shape_y)),
                1 => Box::new(Cross::new(shape_x, shape_y)),
                2 => Box::new(InvertedL::new(shape_x, shape_y)),
                3 => Box::new(VerticalLine::new(shape_x, shape_y)),
                4 => Box::new(Square::new(shape_x, shape_y)),
                _ => unreachable!(),
            };
            let current_shape = current_shape.as_mut();

            let current_world_height = world.len();
            let top_y = current_shape.top();

            if top_y > current_world_height {
                for _ in current_world_height..top_y {
                    world.push(vec![false; WORLD_WIDTH]);
                }
            }

            // let mut cloned_world = world.clone();
            // current_shape.draw(&mut cloned_world);
            // cloned_world.reverse();

            // let newline = ['\n'];
            // let visual: String = cloned_world
            //     .iter()
            //     .flat_map(|x| x.iter().map(|&y| if y { '@' } else { '.' }).chain(newline))
            //     .collect();

            // println!("{visual}");

            loop {
                let (jet_idx, next_jet) = jet_iterator.next().unwrap();

                match next_jet {
                    '>' => current_shape.move_right(&world),
                    '<' => current_shape.move_left(&world),
                    _ => unreachable!(),
                }

                // let mut cloned_world = world.clone();
                // current_shape.draw(&mut cloned_world);
                // cloned_world.reverse();

                // let newline = ['\n'];
                // let visual: String = cloned_world
                //     .iter()
                //     .flat_map(|x| x.iter().map(|&y| if y { '@' } else { '.' }).chain(newline))
                //     .collect();

                // println!("{visual}");

                let moved = current_shape.move_down(&world);

                // let mut cloned_world = world.clone();
                // current_shape.draw(&mut cloned_world);
                // cloned_world.reverse();

                // let newline = ['\n'];
                // let visual: String = cloned_world
                //     .iter()
                //     .flat_map(|x| x.iter().map(|&y| if y { '@' } else { '.' }).chain(newline))
                //     .collect();

                // println!("{visual}");

                if !moved {
                    current_shape.draw(&mut world);
                    let previous_height = lowest_empty_location;
                    lowest_empty_location = lowest_empty_location.max(current_shape.top());

                    if TRACK_CYCLES {
                        let mut depth_or_limit: [usize; WORLD_WIDTH] =
                            [lowest_empty_location; WORLD_WIDTH];

                        // Ugly, but I just want a solution
                        'depth_record: for (depth_record_idx, depth_record) in
                            depth_or_limit.iter_mut().enumerate()
                        {
                            for (depth_idx, row) in
                                world[..lowest_empty_location].iter().rev().enumerate()
                            {
                                // Blocked
                                if row[depth_record_idx] {
                                    *depth_record = depth_idx;
                                    continue 'depth_record;
                                }
                            }
                        }

                        let k = (
                            shape_index % 5,
                            jet_idx,
                            lowest_empty_location - previous_height,
                            depth_or_limit,
                        );
                        let v = (shape_index, lowest_empty_location);

                        if let Some(previous) = state_change.insert(k, v) {
                            cycle_len = v.0 - previous.0;
                            cycle_start_idx = previous.0;
                            cycle_height = v.1 - previous.1;
                            cycle_start_height = previous.1;
                            // println!("New: {v:?}");
                            // println!("Previous: {previous:?}");
                            break 'outer;
                        }
                    }

                    break;
                }
            }
        }

        (
            lowest_empty_location,
            state_change,
            (cycle_len, cycle_start_idx, cycle_start_height, cycle_height),
        )
    }

    let (part_1_solution, _, _) = solve::<false>(2022, jet_iterator.clone());

    println!("Part 1: {part_1_solution}");

    let (_, state_change, (cycle_len, cycle_start_idx, cycle_start_height, cycle_height)) =
        solve::<true>(100_000_000, jet_iterator.clone());

    // println!("len: {cycle_len}, start_idx: {cycle_start_idx}, height: {cycle_height}");

    // let target = 2022;
    let target = 1_000_000_000_000;

    let start_rock_count = cycle_start_idx + 1;
    let number_of_rocks_without_start = target - start_rock_count;
    let number_of_cycles = number_of_rocks_without_start / cycle_len;
    let remainder_rock = number_of_rocks_without_start % cycle_len;

    // println!("remainder: {remainder_rock}");

    let height_without_remainder = cycle_start_height + number_of_cycles * cycle_height;

    // println!("Height without remainder: {height_without_remainder}");

    let remainder_position_in_cycle = remainder_rock + cycle_start_idx;

    let idx_height: std::collections::HashMap<usize, usize> = state_change.into_values().collect();

    let remainder_height_diff =
        idx_height.get(&remainder_position_in_cycle).unwrap() - cycle_start_height;

    let part_2_solution = height_without_remainder + remainder_height_diff;

    // println!("Height {height}");

    println!("Part 2: {part_2_solution}");
}
