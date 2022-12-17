use super::load_file;

/// --- Day 13: Distress Signal ---
/// You climb the hill and again try contacting the Elves. However, you instead receive a signal you
/// weren't expecting: a distress signal.
///
/// Your handheld device must still not be working properly; the packets from the distress signal
/// got decoded out of order. You'll need to re-order the list of received packets (your puzzle
/// input) to decode the message.
///
/// Your list consists of pairs of packets; pairs are separated by a blank line. You need to
/// identify how many pairs of packets are in the right order.
///
/// For example:
///
/// [1,1,3,1,1]
/// [1,1,5,1,1]
///
/// [[1],[2,3,4]]
/// [[1],4]
///
/// [9]
/// [[8,7,6]]
///
/// [[4,4],4,4]
/// [[4,4],4,4,4]
///
/// [7,7,7,7]
/// [7,7,7]
///
/// []
/// [3]
///
/// [[[]]]
/// [[]]
///
/// [1,[2,[3,[4,[5,6,7]]]],8,9]
/// [1,[2,[3,[4,[5,6,0]]]],8,9]
/// Packet data consists of lists and integers. Each list starts with [, ends with ], and contains
/// zero or more comma-separated values (either integers or other lists). Each packet is always a
/// list and appears on its own line.
///
/// When comparing two values, the first value is called left and the second value is called right.
/// Then:
///
/// If both values are integers, the lower integer should come first. If the left integer is lower
/// than the right integer, the inputs are in the right order. If the left integer is higher than
/// the right integer, the inputs are not in the right order. Otherwise, the inputs are the same
/// integer; continue checking the next part of the input. If both values are lists, compare the
/// first value of each list, then the second value, and so on. If the left list runs out of items
/// first, the inputs are in the right order. If the right list runs out of items first, the inputs
/// are not in the right order. If the lists are the same length and no comparison makes a decision
/// about the order, continue checking the next part of the input. If exactly one value is an
/// integer, convert the integer to a list which contains that integer as its only value, then retry
/// the comparison. For example, if comparing [0,0,0] and 2, convert the right value to [2] (a list
/// containing 2); the result is then found by instead comparing [0,0,0] and [2]. Using these rules,
/// you can determine which of the pairs in the example are in the right order:
///
/// == Pair 1 ==
/// - Compare [1,1,3,1,1] vs [1,1,5,1,1]
///   - Compare 1 vs 1
///   - Compare 1 vs 1
///   - Compare 3 vs 5
///     - Left side is smaller, so inputs are in the right order
///
/// == Pair 2 ==
/// - Compare [[1],[2,3,4]] vs [[1],4]
///   - Compare [1] vs [1]
///     - Compare 1 vs 1
///   - Compare [2,3,4] vs 4
///     - Mixed types; convert right to [4] and retry comparison
///     - Compare [2,3,4] vs [4]
///       - Compare 2 vs 4
///         - Left side is smaller, so inputs are in the right order
///
/// == Pair 3 ==
/// - Compare [9] vs [[8,7,6]]
///   - Compare 9 vs [8,7,6]
///     - Mixed types; convert left to [9] and retry comparison
///     - Compare [9] vs [8,7,6]
///       - Compare 9 vs 8
///         - Right side is smaller, so inputs are not in the right order
///
/// == Pair 4 ==
/// - Compare [[4,4],4,4] vs [[4,4],4,4,4]
///   - Compare [4,4] vs [4,4]
///     - Compare 4 vs 4
///     - Compare 4 vs 4
///   - Compare 4 vs 4
///   - Compare 4 vs 4
///   - Left side ran out of items, so inputs are in the right order
///
/// == Pair 5 ==
/// - Compare [7,7,7,7] vs [7,7,7]
///   - Compare 7 vs 7
///   - Compare 7 vs 7
///   - Compare 7 vs 7
///   - Right side ran out of items, so inputs are not in the right order
///
/// == Pair 6 ==
/// - Compare [] vs [3]
///   - Left side ran out of items, so inputs are in the right order
///
/// == Pair 7 ==
/// - Compare [[[]]] vs [[]]
///   - Compare [[]] vs []
///     - Right side ran out of items, so inputs are not in the right order
///
/// == Pair 8 ==
/// - Compare [1,[2,[3,[4,[5,6,7]]]],8,9] vs [1,[2,[3,[4,[5,6,0]]]],8,9]
///   - Compare 1 vs 1
///   - Compare [2,[3,[4,[5,6,7]]]] vs [2,[3,[4,[5,6,0]]]]
///     - Compare 2 vs 2
///     - Compare [3,[4,[5,6,7]]] vs [3,[4,[5,6,0]]]
///       - Compare 3 vs 3
///       - Compare [4,[5,6,7]] vs [4,[5,6,0]]
///         - Compare 4 vs 4
///         - Compare [5,6,7] vs [5,6,0]
///           - Compare 5 vs 5
///           - Compare 6 vs 6
///           - Compare 7 vs 0
///             - Right side is smaller, so inputs are not in the right order
/// What are the indices of the pairs that are already in the right order? (The first pair has index
/// 1, the second pair has index 2, and so on.) In the above example, the pairs in the right order
/// are 1, 2, 4, and 6; the sum of these indices is 13.
///
/// Determine which pairs of packets are already in the right order. What is the sum of the indices
/// of those pairs?
///
/// --- Part Two ---
/// Now, you just need to put all of the packets in the right order. Disregard the blank lines in
/// your list of received packets.
///
/// The distress signal protocol also requires that you include two additional divider packets:
///
/// [[2]]
/// [[6]]
/// Using the same rules as before, organize all packets - the ones in your list of received packets
/// as well as the two divider packets - into the correct order.
///
/// For the example above, the result of putting the packets in the correct order is:
///
/// []
/// [[]]
/// [[[]]]
/// [1,1,3,1,1]
/// [1,1,5,1,1]
/// [[1],[2,3,4]]
/// [1,[2,[3,[4,[5,6,0]]]],8,9]
/// [1,[2,[3,[4,[5,6,7]]]],8,9]
/// [[1],4]
/// [[2]]
/// [3]
/// [[4,4],4,4]
/// [[4,4],4,4,4]
/// [[6]]
/// [7,7,7]
/// [7,7,7,7]
/// [[8,7,6]]
/// [9]
/// Afterward, locate the divider packets. To find the decoder key for this distress signal, you
/// need to determine the indices of the two divider packets and multiply them together. (The first
/// packet is at index 1, the second packet is at index 2, and so on.) In this example, the divider
/// packets are 10th and 14th, and so the decoder key is 140.
///
/// Organize all of the packets into the correct order. What is the decoder key for the distress
/// signal?
pub fn day_13() {
    let data = load_file(13);

    let packet_pairs_str = data.trim().split("\n\n");

    use std::str::FromStr;

    #[derive(Debug, Clone)]
    enum MixedItem {
        Integer(u32),
        IntergerList(Vec<u32>),
        NestedList(Vec<MixedItem>),
    }

    impl MixedItem {
        pub fn is_integer(&self) -> bool {
            matches!(self, MixedItem::Integer(_))
        }
    }

    impl FromStr for MixedItem {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if s == "[]" {
                return Ok(MixedItem::IntergerList(vec![]));
            }

            let integer = s.parse();
            if let Ok(integer) = integer {
                return Ok(MixedItem::Integer(integer));
            }

            // Remove encasing []
            let list_content = s.strip_prefix('[').unwrap().strip_suffix(']').unwrap();

            let mut bracket_depth = 0;
            let mut past_previous_comma_or_start_byte_idx = 0;

            let mut items_str: Vec<&str> = vec![];

            for (byte_idx, c) in list_content.char_indices() {
                match c {
                    '[' => bracket_depth += 1,
                    ']' => bracket_depth -= 1,
                    ',' => {
                        if bracket_depth == 0 {
                            let byte_range = past_previous_comma_or_start_byte_idx..byte_idx;
                            past_previous_comma_or_start_byte_idx = byte_idx + 1;
                            items_str.push(&list_content[byte_range]);
                        }
                    }
                    _ => (),
                }
                // Manage last part that could be skipeed
                if byte_idx + 1 == list_content.len() {
                    let byte_range = past_previous_comma_or_start_byte_idx..byte_idx + 1;
                    items_str.push(&list_content[byte_range]);
                }
            }

            let items_str = items_str;

            let nested_list_content: Vec<MixedItem> = items_str
                .into_iter()
                .map(|item_str| item_str.parse().unwrap())
                .collect();

            if nested_list_content.iter().all(|elt| elt.is_integer()) {
                let integer_list_content: Vec<u32> = nested_list_content
                    .into_iter()
                    .map(|elt| match elt {
                        MixedItem::Integer(inner) => inner,
                        _ => unreachable!(),
                    })
                    .collect();

                return Ok(MixedItem::IntergerList(integer_list_content));
            }

            Ok(MixedItem::NestedList(nested_list_content))
        }
    }

    use std::cmp::Ordering;

    fn cmp_mixed_item(lhs: &MixedItem, rhs: &MixedItem) -> Ordering {
        match (lhs, rhs) {
            (MixedItem::Integer(lhs), MixedItem::Integer(rhs)) => lhs.cmp(rhs),
            (MixedItem::IntergerList(lhs), MixedItem::IntergerList(rhs)) => {
                let size_cmp = lhs.len().cmp(&rhs.len());
                for (lhs, rhs) in lhs.iter().zip(rhs.iter()) {
                    if lhs < rhs {
                        return Ordering::Less;
                    }

                    if lhs > rhs {
                        return Ordering::Greater;
                    }
                }

                size_cmp
            }
            (MixedItem::Integer(lhs), MixedItem::IntergerList(_)) => {
                cmp_mixed_item(&MixedItem::IntergerList(vec![*lhs]), rhs)
            }
            (MixedItem::IntergerList(_), MixedItem::Integer(rhs)) => {
                cmp_mixed_item(lhs, &MixedItem::IntergerList(vec![*rhs]))
            }
            (MixedItem::Integer(lhs), MixedItem::NestedList(_)) => {
                cmp_mixed_item(&MixedItem::IntergerList(vec![*lhs]), rhs)
            }
            (MixedItem::NestedList(_), MixedItem::Integer(rhs)) => {
                cmp_mixed_item(lhs, &MixedItem::IntergerList(vec![*rhs]))
            }
            (MixedItem::NestedList(lhs), MixedItem::NestedList(rhs)) => {
                let size_cmp = lhs.len().cmp(&rhs.len());
                for (lhs, rhs) in lhs.iter().zip(rhs.iter()) {
                    let cmp = cmp_mixed_item(lhs, rhs);
                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                }

                size_cmp
            }
            (MixedItem::IntergerList(lhs), MixedItem::NestedList(rhs)) => {
                let size_cmp = lhs.len().cmp(&rhs.len());
                for (&lhs, rhs) in lhs.iter().zip(rhs.iter()) {
                    let cmp = cmp_mixed_item(&MixedItem::Integer(lhs), rhs);
                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                }

                size_cmp
            }
            (MixedItem::NestedList(lhs), MixedItem::IntergerList(rhs)) => {
                let size_cmp = lhs.len().cmp(&rhs.len());
                for (lhs, &rhs) in lhs.iter().zip(rhs.iter()) {
                    let cmp = cmp_mixed_item(lhs, &MixedItem::Integer(rhs));
                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                }

                size_cmp
            }
        }
    }

    let mut ordered_pair_index_sum = 0;

    let first_divider = MixedItem::from_str("[[2]]").unwrap();
    let second_divider = MixedItem::from_str("[[6]]").unwrap();

    let mut all_packets: Vec<MixedItem> = vec![first_divider.clone(), second_divider.clone()];

    for (pair_idx, packet_pair_str) in packet_pairs_str.enumerate() {
        let (first_packet, second_packet) = packet_pair_str.trim().split_once('\n').unwrap();

        let first_packet: MixedItem = first_packet.parse().unwrap();
        let second_packet: MixedItem = second_packet.parse().unwrap();

        all_packets.push(first_packet.clone());
        all_packets.push(second_packet.clone());

        let cmp = cmp_mixed_item(&first_packet, &second_packet);

        if cmp == Ordering::Less {
            ordered_pair_index_sum += pair_idx + 1;
        }
    }

    all_packets.sort_by(cmp_mixed_item);

    let decoder_key = all_packets.iter().enumerate().fold(1, |acc, (idx, x)| {
        if cmp_mixed_item(x, &first_divider) == Ordering::Equal
            || cmp_mixed_item(x, &second_divider) == Ordering::Equal
        {
            acc * (idx + 1)
        } else {
            acc
        }
    });

    println!("Part 1: {ordered_pair_index_sum}");
    println!("Part 2: {decoder_key}");
}
