use super::load_file;

/// --- Day 6: Tuning Trouble ---
/// The preparations are finally complete; you and the Elves leave camp on foot and begin to make
/// your way toward the star fruit grove.
///
/// As you move through the dense undergrowth, one of the Elves gives you a handheld device. He says
/// that it has many fancy features, but the most important one to set up right now is the
/// communication system.
///
/// However, because he's heard you have significant experience dealing with signal-based systems,
/// he convinced the other Elves that it would be okay to give you their one malfunctioning device -
/// surely you'll have no problem fixing it.
///
/// As if inspired by comedic timing, the device emits a few colorful sparks.
///
/// To be able to communicate with the Elves, the device needs to lock on to their signal. The
/// signal is a series of seemingly-random characters that the device receives one at a time.
///
/// To fix the communication system, you need to add a subroutine to the device that detects a
/// start-of-packet marker in the datastream. In the protocol being used by the Elves, the start of
/// a packet is indicated by a sequence of four characters that are all different.
///
/// The device will send your subroutine a datastream buffer (your puzzle input); your subroutine
/// needs to identify the first position where the four most recently received characters were all
/// different. Specifically, it needs to report the number of characters from the beginning of the
/// buffer to the end of the first such four-character marker.
///
/// For example, suppose you receive the following datastream buffer:
///
/// mjqjpqmgbljsphdztnvjfqwrcgsmlb
/// After the first three characters (mjq) have been received, there haven't been enough characters
/// received yet to find the marker. The first time a marker could occur is after the fourth
/// character is received, making the most recent four characters mjqj. Because j is repeated, this
/// isn't a marker.
///
/// The first time a marker appears is after the seventh character arrives. Once it does, the last
/// four characters received are jpqm, which are all different. In this case, your subroutine should
/// report the value 7, because the first start-of-packet marker is complete after 7 characters have
/// been processed.
///
/// Here are a few more examples:
///
/// bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 5
/// nppdvjthqldpwncqszvftbrmjlhg: first marker after character 6
/// nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 10
/// zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 11
/// How many characters need to be processed before the first start-of-packet marker is detected?
///
/// --- Part Two ---
/// Your device's communication system is correctly detecting packets, but still isn't working. It
/// looks like it also needs to look for messages.
///
/// A start-of-message marker is just like a start-of-packet marker, except it consists of 14
/// distinct characters rather than 4.
///
/// Here are the first positions of start-of-message markers for all of the above examples:
///
/// mjqjpqmgbljsphdztnvjfqwrcgsmlb: first marker after character 19
/// bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 23
/// nppdvjthqldpwncqszvftbrmjlhg: first marker after character 23
/// nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 29
/// zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 26
/// How many characters need to be processed before the first start-of-message marker is detected?
pub fn day_06() {
    let data = load_file(6);

    let mut start_of_packet_buffer: [char; 4] = Default::default();
    let mut start_of_packet_hash_map =
        std::collections::HashMap::<char, usize>::with_capacity(start_of_packet_buffer.len());

    let mut start_of_message_buffer: [char; 14] = Default::default();
    let mut start_of_message_hash_map =
        std::collections::HashMap::<char, usize>::with_capacity(start_of_message_buffer.len());

    let mut part1_solution: Option<usize> = None;
    let mut part2_solution: Option<usize> = None;

    fn find_first_all_different_chars(
        idx: usize,
        c: char,
        buffer: &mut [char],
        hash_map: &mut std::collections::HashMap<char, usize>,
    ) -> Option<usize> {
        let store_idx = idx % buffer.len();

        hash_map.entry(c).and_modify(|e| *e = *e + 1).or_insert(1);
        let oldest_char = buffer[store_idx];
        buffer[store_idx] = c;
        let oldest_char_entry = hash_map.entry(oldest_char).and_modify(|e| *e = *e - 1);
        let oldest_char_count = match oldest_char_entry {
            std::collections::hash_map::Entry::Occupied(e) => *e.get(),
            _ => 0,
        };

        if oldest_char_count == 0 {
            hash_map.remove(&oldest_char);
        }

        if idx >= buffer.len() - 1 && buffer.len() == hash_map.len() {
            return Some(idx + 1);
        }

        None
    }

    for (idx, c) in data.char_indices() {
        if part1_solution.is_none() {
            part1_solution = find_first_all_different_chars(
                idx,
                c,
                &mut start_of_packet_buffer,
                &mut start_of_packet_hash_map,
            );
        }
        if part2_solution.is_none() {
            part2_solution = find_first_all_different_chars(
                idx,
                c,
                &mut start_of_message_buffer,
                &mut start_of_message_hash_map,
            );
        }

        if part1_solution.is_some() && part2_solution.is_some() {
            break;
        }
    }

    println!("Part 1: {}", part1_solution.unwrap());
    println!("Part 2: {}", part2_solution.unwrap());
}
