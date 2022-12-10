use super::load_file;

/// --- Day 5: Supply Stacks ---
/// The expedition can depart as soon as the final supplies have been unloaded from the ships.
/// Supplies are stored in stacks of marked crates, but because the needed supplies are buried under
/// many other crates, the crates need to be rearranged.
///
/// The ship has a giant cargo crane capable of moving crates between stacks. To ensure none of the
/// crates get crushed or fall over, the crane operator will rearrange them in a series of
/// carefully-planned steps. After the crates are rearranged, the desired crates will be at the top
/// of each stack.
///
/// The Elves don't want to interrupt the crane operator during this delicate procedure, but they
/// forgot to ask her which crate will end up where, and they want to be ready to unload them as
/// soon as possible so they can embark.
///
/// They do, however, have a drawing of the starting stacks of crates and the rearrangement
/// procedure (your puzzle input). For example:
///
///     [D]    
/// [N] [C]    
/// [Z] [M] [P]
///  1   2   3
///
/// move 1 from 2 to 1
/// move 3 from 1 to 3
/// move 2 from 2 to 1
/// move 1 from 1 to 2
/// In this example, there are three stacks of crates. Stack 1 contains two crates: crate Z is on
/// the bottom, and crate N is on top. Stack 2 contains three crates; from bottom to top, they are
/// crates M, C, and D. Finally, stack 3 contains a single crate, P.
///
/// Then, the rearrangement procedure is given. In each step of the procedure, a quantity of crates
/// is moved from one stack to a different stack. In the first step of the above rearrangement
/// procedure, one crate is moved from stack 2 to stack 1, resulting in this configuration:
///
/// [D]        
/// [N] [C]    
/// [Z] [M] [P]
///  1   2   3
/// In the second step, three crates are moved from stack 1 to stack 3. Crates are moved one at a
/// time, so the first crate to be moved (D) ends up below the second and third crates:
///
///         [Z]
///         [N]
///     [C] [D]
///     [M] [P]
///  1   2   3
/// Then, both crates are moved from stack 2 to stack 1. Again, because crates are moved one at a
/// time, crate C ends up below crate M:
///
///         [Z]
///         [N]
/// [M]     [D]
/// [C]     [P]
///  1   2   3
/// Finally, one crate is moved from stack 1 to stack 2:
///
///         [Z]
///         [N]
///         [D]
/// [C] [M] [P]
///  1   2   3
/// The Elves just need to know which crate will end up on top of each stack; in this example, the
/// top crates are C in stack 1, M in stack 2, and Z in stack 3, so you should combine these
/// together and give the Elves the message CMZ.
///
/// After the rearrangement procedure completes, what crate ends up on top of each stack?
pub fn day_05() {
    let data = load_file(5);
    let (stock, instructions) = data.split_once("\n\n").unwrap();

    let mut stock_as_lines: Vec<&str> = stock.split("\n").into_iter().collect();
    stock_as_lines.reverse();
    let stock_as_lines = stock_as_lines;

    let stack_count = stock_as_lines.first().unwrap().split_whitespace().count();

    let mut stacks = vec![vec![String::new(); 0]; stack_count];

    for stack_content in stock_as_lines.iter().skip(1) {
        for (stack_idx, char) in stack_content.chars().skip(1).step_by(4).enumerate() {
            match char {
                ' ' => (),
                _ => stacks[stack_idx].push(char.to_string()),
            }
        }
    }

    for instruction in instructions.trim().split("\n").into_iter() {
        let mut split_instruction = instruction
            .split_whitespace()
            .into_iter()
            .skip(1)
            .step_by(2);

        let move_count: usize = split_instruction.next().unwrap().parse().unwrap();
        let src_stack_idx: usize = split_instruction.next().unwrap().parse().unwrap();
        let dst_stack_idx: usize = split_instruction.next().unwrap().parse().unwrap();

        // Index were given starting at 1
        let src_stack_idx = src_stack_idx - 1;
        let dst_stack_idx = dst_stack_idx - 1;

        for _ in 0..move_count {
            let value_to_move = stacks[src_stack_idx].pop().unwrap();
            stacks[dst_stack_idx].push(value_to_move);
        }
    }

    print!("Part 1: ");
    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    print!("\n");
}
