use super::load_file;

/// --- Day 20: Grove Positioning System ---
/// It's finally time to meet back up with the Elves. When you try to contact them, however, you get
/// no reply. Perhaps you're out of range?
///
/// You know they're headed to the grove where the star fruit grows, so if you can figure out where
/// that is, you should be able to meet back up with them.
///
/// Fortunately, your handheld device has a file (your puzzle input) that contains the grove's
/// coordinates! Unfortunately, the file is encrypted - just in case the device were to fall into
/// the wrong hands.
///
/// Maybe you can decrypt it?
///
/// When you were still back at the camp, you overheard some Elves talking about coordinate file
/// encryption. The main operation involved in decrypting the file is called mixing.
///
/// The encrypted file is a list of numbers. To mix the file, move each number forward or backward
/// in the file a number of positions equal to the value of the number being moved. The list is
/// circular, so moving a number off one end of the list wraps back around to the other end as if
/// the ends were connected.
///
/// For example, to move the 1 in a sequence like 4, 5, 6, 1, 7, 8, 9, the 1 moves one position
/// forward: 4, 5, 6, 7, 1, 8, 9. To move the -2 in a sequence like 4, -2, 5, 6, 7, 8, 9, the -2
/// moves two positions backward, wrapping around: 4, 5, 6, 7, 8, -2, 9.
///
/// The numbers should be moved in the order they originally appear in the encrypted file. Numbers
/// moving around during the mixing process do not change the order in which the numbers are moved.
///
/// Consider this encrypted file:
///
/// 1
/// 2
/// -3
/// 3
/// -2
/// 0
/// 4
/// Mixing this file proceeds as follows:
///
/// Initial arrangement:
/// 1, 2, -3, 3, -2, 0, 4
///
/// 1 moves between 2 and -3:
/// 2, 1, -3, 3, -2, 0, 4
///
/// 2 moves between -3 and 3:
/// 1, -3, 2, 3, -2, 0, 4
///
/// -3 moves between -2 and 0:
/// 1, 2, 3, -2, -3, 0, 4
///
/// 3 moves between 0 and 4:
/// 1, 2, -2, -3, 0, 3, 4
///
/// -2 moves between 4 and 1:
/// 1, 2, -3, 0, 3, 4, -2
///
/// 0 does not move:
/// 1, 2, -3, 0, 3, 4, -2
///
/// 4 moves between -3 and 0:
/// 1, 2, -3, 4, 0, 3, -2
/// Then, the grove coordinates can be found by looking at the 1000th, 2000th, and 3000th numbers
/// after the value 0, wrapping around the list as necessary. In the above example, the 1000th
/// number after 0 is 4, the 2000th is -3, and the 3000th is 2; adding these together produces 3.
///
/// Mix your encrypted file exactly once. What is the sum of the three numbers that form the grove
/// coordinates?
pub fn day_20() {
    let data = load_file(20);

    let data_as_lines = data.trim().split('\n');

    let input_order_numbers: Vec<i64> = data_as_lines
        .clone()
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect();

    let buffer_len = input_order_numbers.len() as i64;

    use std::collections::VecDeque;

    let mut decrypted_numbers_indices: VecDeque<usize> = (0..input_order_numbers.len()).collect();

    // let display: Vec<i64> = decrypted_numbers_indices
    //     .iter()
    //     .map(|&idx| input_order_numbers[idx])
    //     .collect();

    // println!("{display:?}");

    for (input_idx, &number) in input_order_numbers.iter().enumerate() {
        let current_decryption_position = {
            let mut current_decryption_position = 0;
            for (current_pos_idx, &number_index) in decrypted_numbers_indices.iter().enumerate() {
                if number_index == input_idx {
                    current_decryption_position = current_pos_idx;
                    break;
                }
            }
            current_decryption_position as i64
        };

        let new_decryption_position = current_decryption_position + number;

        let new_decryption_position_modulus = new_decryption_position.rem_euclid(buffer_len - 1);

        let poped_val = decrypted_numbers_indices
            .remove(current_decryption_position as usize)
            .unwrap();
        decrypted_numbers_indices.insert(new_decryption_position_modulus as usize, poped_val);

        // let display: Vec<i64> = decrypted_numbers_indices
        //     .iter()
        //     .map(|&idx| input_order_numbers[idx])
        //     .collect();

        // println!("{display:?}");
    }

    let zero_idx = {
        let mut zero_idx = 0;
        for (idx, &number) in input_order_numbers.iter().enumerate() {
            if number == 0 {
                zero_idx = idx;
                break;
            }
        }
        zero_idx
    };

    let zero_position = {
        let mut zero_position = 0;
        for (position, &number_idx) in decrypted_numbers_indices.iter().enumerate() {
            if number_idx == zero_idx {
                zero_position = position;
                break;
            }
        }
        zero_position
    };

    let buffer_len_usize = input_order_numbers.len();

    let thousand_n_idx = (1000 + zero_position).rem_euclid(buffer_len_usize);
    let two_thousand_n_idx = (2000 + zero_position).rem_euclid(buffer_len_usize);
    let three_thousand_n_idx = (3000 + zero_position).rem_euclid(buffer_len_usize);

    let thousand_n = input_order_numbers[decrypted_numbers_indices[thousand_n_idx]];
    let two_thousand_n = input_order_numbers[decrypted_numbers_indices[two_thousand_n_idx]];
    let three_thousand_n = input_order_numbers[decrypted_numbers_indices[three_thousand_n_idx]];

    let part_1 = thousand_n + two_thousand_n + three_thousand_n;

    println!("Part 1: {part_1}");
}
