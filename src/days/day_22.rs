use super::load_file;

/// --- Day 22: Monkey Map ---
/// The monkeys take you on a surprisingly easy trail through the jungle. They're even going in
/// roughly the right direction according to your handheld device's Grove Positioning System.
///
/// As you walk, the monkeys explain that the grove is protected by a force field. To pass through
/// the force field, you have to enter a password; doing so involves tracing a specific path on a
/// strangely-shaped board.
///
/// At least, you're pretty sure that's what you have to do; the elephants aren't exactly fluent in
/// monkey.
///
/// The monkeys give you notes that they took when they last saw the password entered (your puzzle
/// input).
///
/// For example:
///
///         ...#
///         .#..
///         #...
///         ....
/// ...#.......#
/// ........#...
/// ..#....#....
/// ..........#.
///         ...#....
///         .....#..
///         .#......
///         ......#.
///
/// 10R5L5R10L4R5L5
/// The first half of the monkeys' notes is a map of the board. It is comprised of a set of open
/// tiles (on which you can move, drawn .) and solid walls (tiles which you cannot enter, drawn #).
///
/// The second half is a description of the path you must follow. It consists of alternating numbers
/// and letters:
///
/// A number indicates the number of tiles to move in the direction you are facing. If you run into
/// a wall, you stop moving forward and continue with the next instruction. A letter indicates
/// whether to turn 90 degrees clockwise (R) or counterclockwise (L). Turning happens in-place; it
/// does not change your current tile. So, a path like 10R5 means "go forward 10 tiles, then turn
/// clockwise 90 degrees, then go forward 5 tiles".
///
/// You begin the path in the leftmost open tile of the top row of tiles. Initially, you are facing
/// to the right (from the perspective of how the map is drawn).
///
/// If a movement instruction would take you off of the map, you wrap around to the other side of
/// the board. In other words, if your next tile is off of the board, you should instead look in the
/// direction opposite of your current facing as far as you can until you find the opposite edge of
/// the board, then reappear there.
///
/// For example, if you are at A and facing to the right, the tile in front of you is marked B; if
/// you are at C and facing down, the tile in front of you is marked D:
///
///         ...#
///         .#..
///         #...
///         ....
/// ...#.D.....#
/// ........#...
/// B.#....#...A
/// .....C....#.
///         ...#....
///         .....#..
///         .#......
///         ......#.
/// It is possible for the next tile (after wrapping around) to be a wall; this still counts as
/// there being a wall in front of you, and so movement stops before you actually wrap to the other
/// side of the board.
///
/// By drawing the last facing you had with an arrow on each tile you visit, the full path taken by
/// the above example looks like this:
///
///         >>v#    
///         .#v.    
///         #.v.    
///         ..v.    
/// ...#...v..v#    
/// >>>v...>#.>>    
/// ..#v...#....    
/// ...>>>>v..#.    
///         ...#....
///         .....#..
///         .#......
///         ......#.
/// To finish providing the password to this strange input device, you need to determine numbers for
/// your final row, column, and facing as your final position appears from the perspective of the
/// original map. Rows start from 1 at the top and count downward; columns start from 1 at the left
/// and count rightward. (In the above example, row 1, column 1 refers to the empty space with no
/// tile on it in the top-left corner.) Facing is 0 for right (>), 1 for down (v), 2 for left (<),
/// and 3 for up (^). The final password is the sum of 1000 times the row, 4 times the column, and
/// the facing.
///
/// In the above example, the final row is 6, the final column is 8, and the final facing is 0. So,
/// the final password is 1000 * 6 + 4 * 8 + 0: 6032.
///
/// Follow the path given in the monkeys' notes. What is the final password?
pub fn day_22() {
    let data = load_file(22);

    let (map_str, instructions) = data.split_once("\n\n").unwrap();

    use std::collections::HashMap;

    let mut hashed_map: HashMap<(usize, usize), char> = Default::default();

    let mut line_count = 0;
    let mut column_count = 0;

    for (line_idx, map_line_str) in map_str.split('\n').enumerate() {
        line_count += 1;
        for (col_idx, c) in map_line_str.char_indices() {
            column_count = column_count.max(col_idx + 1);
            hashed_map.insert((line_idx, col_idx), c);
        }
    }

    let line_count = line_count;
    let column_count = column_count;
    let mut map = vec![vec![' '; column_count]; line_count];

    for ((line_idx, col_idx), terrain) in hashed_map {
        map[line_idx][col_idx] = terrain;
    }

    let mut line = 0;
    let mut column = 0;

    for col_idx in 0..column_count {
        if map[line][col_idx] == '.' {
            column = col_idx;
            break;
        }
    }

    // println!("pos: {line}, {column}");

    let instructions = instructions.trim();

    let mut char_iter = instructions.char_indices().peekable();
    let mut start_number_idx = 0;
    let mut end_number_idx;

    // 0 right, 1 down, 2 left, 3 up
    let mut current_direction: i32 = 0;

    let part1_res;

    loop {
        loop {
            if let Some((idx, c)) = char_iter.peek() {
                match c {
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        char_iter.next();
                    }
                    _ => {
                        end_number_idx = *idx;
                        break;
                    }
                }
            } else {
                end_number_idx = instructions.len();
                break;
            }
        }

        let moves: usize = instructions[start_number_idx..end_number_idx]
            .parse()
            .unwrap();

        // move

        // println!(
        //     "direction: {}, moves: {moves}",
        //     match current_direction {
        //         0 => "right",
        //         1 => "down",
        //         2 => "left",
        //         3 => "up",
        //         _ => unreachable!(),
        //     }
        // );

        let (diff_line, diff_col) = match current_direction {
            // Right
            0 => (0, 1),
            // Down
            1 => (1, 0),
            // Left
            2 => (0, -1),
            // Up
            3 => (-1, 0),
            _ => unreachable!(),
        };

        for _ in 0..moves {
            let (next_line, next_col) = (
                (line as i32 + diff_line).rem_euclid(line_count as i32) as usize,
                (column as i32 + diff_col).rem_euclid(column_count as i32) as usize,
            );

            (line, column) = match map[next_line][next_col] {
                '.' => (next_line, next_col),
                ' ' => {
                    let (mut jump_line, mut jump_col) = (next_line, next_col);
                    while map[jump_line][jump_col] == ' ' {
                        jump_line =
                            (jump_line as i32 + diff_line).rem_euclid(line_count as i32) as usize;
                        jump_col =
                            (jump_col as i32 + diff_col).rem_euclid(column_count as i32) as usize;
                    }
                    if map[jump_line][jump_col] == '#' {
                        break;
                    }
                    (jump_line, jump_col)
                }
                '#' => break,
                _ => unreachable!(),
            };
        }

        // println!("pos: {}, {}", line + 1, column + 1);

        if let Some((idx, rotation)) = char_iter.next() {
            match rotation {
                'L' => {
                    current_direction = (current_direction - 1).rem_euclid(4);
                    start_number_idx = idx + 1;
                }
                'R' => {
                    current_direction = (current_direction + 1).rem_euclid(4);
                    start_number_idx = idx + 1;
                }
                _ => unreachable!(),
            }
        } else {
            part1_res = 1000 * (line + 1) + 4 * (column + 1) + current_direction as usize;
            break;
        }
    }

    println!("Part 1: {part1_res}");
}
