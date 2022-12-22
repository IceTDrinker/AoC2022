use super::load_file;

/// --- Day 16: Proboscidea Volcanium ---
/// The sensors have led you to the origin of the distress signal: yet another handheld device, just
/// like the one the Elves gave you. However, you don't see any Elves around; instead, the device is
/// surrounded by elephants! They must have gotten lost in these tunnels, and one of the elephants
/// apparently figured out how to turn on the distress signal.
///
/// The ground rumbles again, much stronger this time. What kind of cave is this, exactly? You scan
/// the cave with your handheld device; it reports mostly igneous rock, some ash, pockets of
/// pressurized gas, magma... this isn't just a cave, it's a volcano!
///
/// You need to get the elephants out of here, quickly. Your device estimates that you have 30
/// minutes before the volcano erupts, so you don't have time to go back out the way you came in.
///
/// You scan the cave for other options and discover a network of pipes and pressure-release valves.
/// You aren't sure how such a system got into a volcano, but you don't have time to complain; your
/// device produces a report (your puzzle input) of each valve's flow rate if it were opened (in
/// pressure per minute) and the tunnels you could use to move between the valves.
///
/// There's even a valve in the room you and the elephants are currently standing in labeled AA. You
/// estimate it will take you one minute to open a single valve and one minute to follow any tunnel
/// from one valve to another. What is the most pressure you could release?
///
/// For example, suppose you had the following scan output:
///
/// Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
/// Valve BB has flow rate=13; tunnels lead to valves CC, AA
/// Valve CC has flow rate=2; tunnels lead to valves DD, BB
/// Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
/// Valve EE has flow rate=3; tunnels lead to valves FF, DD
/// Valve FF has flow rate=0; tunnels lead to valves EE, GG
/// Valve GG has flow rate=0; tunnels lead to valves FF, HH
/// Valve HH has flow rate=22; tunnel leads to valve GG
/// Valve II has flow rate=0; tunnels lead to valves AA, JJ
/// Valve JJ has flow rate=21; tunnel leads to valve II
/// All of the valves begin closed. You start at valve AA, but it must be damaged or jammed or
/// something: its flow rate is 0, so there's no point in opening it. However, you could spend one
/// minute moving to valve BB and another minute opening it; doing so would release pressure during
/// the remaining 28 minutes at a flow rate of 13, a total eventual pressure release of 28 * 13 =
/// 364. Then, you could spend your third minute moving to valve CC and your fourth minute opening
/// it, providing an additional 26 minutes of eventual pressure release at a flow rate of 2, or 52
/// total pressure released by valve CC.
///
/// Making your way through the tunnels like this, you could probably open many or all of the valves
/// by the time 30 minutes have elapsed. However, you need to release as much pressure as possible,
/// so you'll need to be methodical. Instead, consider this approach:
///
/// == Minute 1 ==
/// No valves are open.
/// You move to valve DD.
///
/// == Minute 2 ==
/// No valves are open.
/// You open valve DD.
///
/// == Minute 3 ==
/// Valve DD is open, releasing 20 pressure.
/// You move to valve CC.
///
/// == Minute 4 ==
/// Valve DD is open, releasing 20 pressure.
/// You move to valve BB.
///
/// == Minute 5 ==
/// Valve DD is open, releasing 20 pressure.
/// You open valve BB.
///
/// == Minute 6 ==
/// Valves BB and DD are open, releasing 33 pressure.
/// You move to valve AA.
///
/// == Minute 7 ==
/// Valves BB and DD are open, releasing 33 pressure.
/// You move to valve II.
///
/// == Minute 8 ==
/// Valves BB and DD are open, releasing 33 pressure.
/// You move to valve JJ.
///
/// == Minute 9 ==
/// Valves BB and DD are open, releasing 33 pressure.
/// You open valve JJ.
///
/// == Minute 10 ==
/// Valves BB, DD, and JJ are open, releasing 54 pressure.
/// You move to valve II.
///
/// == Minute 11 ==
/// Valves BB, DD, and JJ are open, releasing 54 pressure.
/// You move to valve AA.
///
/// == Minute 12 ==
/// Valves BB, DD, and JJ are open, releasing 54 pressure.
/// You move to valve DD.
///
/// == Minute 13 ==
/// Valves BB, DD, and JJ are open, releasing 54 pressure.
/// You move to valve EE.
///
/// == Minute 14 ==
/// Valves BB, DD, and JJ are open, releasing 54 pressure.
/// You move to valve FF.
///
/// == Minute 15 ==
/// Valves BB, DD, and JJ are open, releasing 54 pressure.
/// You move to valve GG.
///
/// == Minute 16 ==
/// Valves BB, DD, and JJ are open, releasing 54 pressure.
/// You move to valve HH.
///
/// == Minute 17 ==
/// Valves BB, DD, and JJ are open, releasing 54 pressure.
/// You open valve HH.
///
/// == Minute 18 ==
/// Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
/// You move to valve GG.
///
/// == Minute 19 ==
/// Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
/// You move to valve FF.
///
/// == Minute 20 ==
/// Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
/// You move to valve EE.
///
/// == Minute 21 ==
/// Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
/// You open valve EE.
///
/// == Minute 22 ==
/// Valves BB, DD, EE, HH, and JJ are open, releasing 79 pressure.
/// You move to valve DD.
///
/// == Minute 23 ==
/// Valves BB, DD, EE, HH, and JJ are open, releasing 79 pressure.
/// You move to valve CC.
///
/// == Minute 24 ==
/// Valves BB, DD, EE, HH, and JJ are open, releasing 79 pressure.
/// You open valve CC.
///
/// == Minute 25 ==
/// Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.
///
/// == Minute 26 ==
/// Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.
///
/// == Minute 27 ==
/// Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.
///
/// == Minute 28 ==
/// Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.
///
/// == Minute 29 ==
/// Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.
///
/// == Minute 30 ==
/// Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.
/// This approach lets you release the most pressure possible in 30 minutes with this valve layout,
/// 1651.
///
/// Work out the steps to release the most pressure in 30 minutes. What is the most pressure you can
/// release?
///
/// --- Part Two ---
/// You're worried that even with an optimal approach, the pressure released won't be enough. What
/// if you got one of the elephants to help you?
///
/// It would take you 4 minutes to teach an elephant how to open the right valves in the right
/// order, leaving you with only 26 minutes to actually execute your plan. Would having two of you
/// working together be better, even if it means having less time? (Assume that you teach the
/// elephant before opening any valves yourself, giving you both the same full 26 minutes.)
///
/// In the example above, you could teach the elephant to help you as follows:
///
/// == Minute 1 ==
/// No valves are open.
/// You move to valve II.
/// The elephant moves to valve DD.
///
/// == Minute 2 ==
/// No valves are open.
/// You move to valve JJ.
/// The elephant opens valve DD.
///
/// == Minute 3 ==
/// Valve DD is open, releasing 20 pressure.
/// You open valve JJ.
/// The elephant moves to valve EE.
///
/// == Minute 4 ==
/// Valves DD and JJ are open, releasing 41 pressure.
/// You move to valve II.
/// The elephant moves to valve FF.
///
/// == Minute 5 ==
/// Valves DD and JJ are open, releasing 41 pressure.
/// You move to valve AA.
/// The elephant moves to valve GG.
///
/// == Minute 6 ==
/// Valves DD and JJ are open, releasing 41 pressure.
/// You move to valve BB.
/// The elephant moves to valve HH.
///
/// == Minute 7 ==
/// Valves DD and JJ are open, releasing 41 pressure.
/// You open valve BB.
/// The elephant opens valve HH.
///
/// == Minute 8 ==
/// Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
/// You move to valve CC.
/// The elephant moves to valve GG.
///
/// == Minute 9 ==
/// Valves BB, DD, HH, and JJ are open, releasing 76 pressure.
/// You open valve CC.
/// The elephant moves to valve FF.
///
/// == Minute 10 ==
/// Valves BB, CC, DD, HH, and JJ are open, releasing 78 pressure.
/// The elephant moves to valve EE.
///
/// == Minute 11 ==
/// Valves BB, CC, DD, HH, and JJ are open, releasing 78 pressure.
/// The elephant opens valve EE.
///
/// (At this point, all valves are open.)
///
/// == Minute 12 ==
/// Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.
///
/// ...
///
/// == Minute 20 ==
/// Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.
///
/// ...
///
/// == Minute 26 ==
/// Valves BB, CC, DD, EE, HH, and JJ are open, releasing 81 pressure.
/// With the elephant helping, after 26 minutes, the best you could do would release a total of 1707
/// pressure.
///
/// With you and an elephant working together for 26 minutes, what is the most pressure you could
/// release?
pub fn day_16() {
    let data = load_file(16);

    use std::collections::{HashMap, HashSet, VecDeque};

    #[derive(Debug, Clone)]
    struct Valve {
        name: String,
        pressure_release_per_minute: u64,
        tunnels_lead_to: HashMap<String, u64>,
    }

    impl Valve {
        pub fn new(
            name: &str,
            pressure_release_per_minute: u64,
            tunnels: HashMap<String, u64>,
        ) -> Self {
            Self {
                name: name.to_string(),
                pressure_release_per_minute,
                tunnels_lead_to: tunnels,
            }
        }
    }

    impl PartialEq for Valve {
        fn eq(&self, other: &Self) -> bool {
            self.name == other.name
        }
    }

    impl Eq for Valve {}

    impl std::str::FromStr for Valve {
        type Err = Box<dyn std::error::Error>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (name_flow, tunnels) = s.trim().split_once(';').unwrap();
            let name_flow = name_flow.strip_prefix("Valve").unwrap();
            let (name, flow) = name_flow.trim().split_once("has flow rate=").unwrap();
            let name = name.trim();
            let flow: u64 = flow.parse()?;
            let (_, tunnels) = tunnels.split_once("valve").unwrap();
            let tunnels: HashMap<String, u64> = tunnels
                .trim_start_matches('s')
                .split(',')
                .into_iter()
                .map(|x| (x.trim().to_string(), 1))
                .collect();

            Ok(Valve::new(name, flow, tunnels))
        }
    }

    let mut all_valves = HashMap::new();

    let mut idx_to_valve: Vec<String> = vec![];

    for valve_str in data.trim().split('\n') {
        let valve: Valve = valve_str.parse().unwrap();
        idx_to_valve.push(valve.name.clone());
        all_valves.insert(valve.name.clone(), valve);
    }

    let valve_to_idx: HashMap<String, usize> = idx_to_valve
        .iter()
        .enumerate()
        .map(|(idx, valve_name)| (valve_name.clone(), idx))
        .collect();

    let valve_count = valve_to_idx.len();

    let mut adjacency_matrix = vec![vec![u64::MAX; valve_count]; valve_count];

    for (src_valve_idx, src_valve_name) in idx_to_valve.iter().enumerate() {
        let src_valve = all_valves.get(src_valve_name).unwrap();

        for dst_valve_name in src_valve.tunnels_lead_to.keys() {
            let &dst_valve_idx = valve_to_idx.get(dst_valve_name).unwrap();

            adjacency_matrix[src_valve_idx][dst_valve_idx] = 1;
            adjacency_matrix[dst_valve_idx][src_valve_idx] = 1;
            adjacency_matrix[src_valve_idx][src_valve_idx] = 0;
            adjacency_matrix[dst_valve_idx][dst_valve_idx] = 0;
        }
    }

    // Floyd warshall
    for k in 0..valve_count {
        for i in 0..valve_count {
            for j in 0..valve_count {
                let w_ik = adjacency_matrix[i][k];
                let w_kj = adjacency_matrix[k][j];
                let w_ij = &mut adjacency_matrix[i][j];

                if w_ik == u64::MAX || w_kj == u64::MAX {
                    continue;
                }

                *w_ij = std::cmp::min(*w_ij, w_ik + w_kj);
            }
        }
    }

    {
        let tmp_all_valves = all_valves.clone();

        for valve in all_valves.values_mut() {
            if valve.pressure_release_per_minute != 0 || valve.name == "AA" {
                let &valve_idx = valve_to_idx.get(&valve.name).unwrap();

                valve.tunnels_lead_to.clear();

                let valve_adjacencies = &adjacency_matrix[valve_idx];

                for (dst_idx, &distance) in valve_adjacencies.iter().enumerate() {
                    if valve_idx == dst_idx {
                        continue;
                    }

                    let dst_valve_name = &idx_to_valve[dst_idx];
                    let dst_valve = tmp_all_valves.get(dst_valve_name).unwrap();
                    if dst_valve.pressure_release_per_minute == 0 {
                        continue;
                    }

                    valve
                        .tunnels_lead_to
                        .insert(dst_valve_name.clone(), distance);
                }
            }
        }
    }

    let mut non_zero_valves_sorted: Vec<_> = all_valves
        .iter()
        .filter(|&x| x.1.pressure_release_per_minute != 0)
        .map(|x| x.1)
        .collect();
    non_zero_valves_sorted.sort_by_key(|x| x.pressure_release_per_minute);
    non_zero_valves_sorted.reverse();
    let non_zero_valves_sorted = non_zero_valves_sorted;

    const TIME_LIMIT_PART1: u64 = 30;

    struct ProblemSolutionPart1 {
        pub current_valve: String,
        pub expected_release: u64,
        pub current_time: u64,
        pub opened_valves: HashSet<String>,
    }

    impl ProblemSolutionPart1 {
        pub fn is_complete(&self, time_limit: u64, non_zero_valves_sorted: &[&Valve]) -> bool {
            assert!(self.current_time <= time_limit);
            self.current_time == time_limit
                || self.opened_valves.len() == non_zero_valves_sorted.len()
        }

        pub fn release_upper_bound(
            &self,
            all_valves: &HashMap<String, Valve>,
            non_zero_valves_sorted: &[&Valve],
            time_limit: u64,
        ) -> u64 {
            let mut release_upper_bound = self.expected_release;
            let mut remaining_time = time_limit - self.current_time;

            let current_valve = all_valves.get(&self.current_valve).unwrap();
            if current_valve.pressure_release_per_minute != 0
                && remaining_time > 1
                && !self.opened_valves.contains(&self.current_valve)
            {
                remaining_time -= 1;
                release_upper_bound += remaining_time * current_valve.pressure_release_per_minute;
            }

            for valve in non_zero_valves_sorted {
                if remaining_time < 2 {
                    break;
                }
                if !self.opened_valves.contains(&valve.name) && valve.name != current_valve.name {
                    remaining_time -= 2;
                    release_upper_bound += remaining_time * valve.pressure_release_per_minute;
                }
            }

            release_upper_bound
        }
    }

    fn solve(
        time_limit: u64,
        all_valves: &HashMap<String, Valve>,
        non_zero_valves_sorted: &[&Valve],
    ) -> u64 {
        let mut solutions_queue = VecDeque::new();
        solutions_queue.push_back(ProblemSolutionPart1 {
            current_valve: "AA".to_string(),
            expected_release: 0,
            current_time: 0,
            opened_valves: HashSet::new(),
        });

        let mut best_complete_release = 0;

        while let Some(current_solution) = solutions_queue.pop_front() {
            if current_solution.is_complete(time_limit, non_zero_valves_sorted) {
                best_complete_release =
                    std::cmp::max(best_complete_release, current_solution.expected_release);
                continue;
            }

            let current_valve = all_valves.get(&current_solution.current_valve).unwrap();

            if current_valve.pressure_release_per_minute != 0
                && (current_solution.current_time < time_limit - 1)
                && !current_solution.opened_valves.contains(&current_valve.name)
            {
                // +1 as we open the valve
                let new_time = current_solution.current_time + 1;
                let mut new_opened_valves = current_solution.opened_valves.clone();
                new_opened_valves.insert(current_valve.name.clone());

                let new_release = current_solution.expected_release
                    + (time_limit - new_time) * current_valve.pressure_release_per_minute;
                let open_valve_solution = ProblemSolutionPart1 {
                    current_valve: current_solution.current_valve.clone(),
                    expected_release: new_release,
                    current_time: new_time,
                    opened_valves: new_opened_valves,
                };

                let release_upper_bound = open_valve_solution.release_upper_bound(
                    all_valves,
                    non_zero_valves_sorted,
                    time_limit,
                );

                if release_upper_bound < best_complete_release {
                    continue;
                }

                best_complete_release =
                    std::cmp::max(open_valve_solution.expected_release, best_complete_release);

                if let Some(next_potential_solution) = solutions_queue.front() {
                    if open_valve_solution.current_time >= next_potential_solution.current_time {
                        solutions_queue.push_front(open_valve_solution);
                    } else {
                        solutions_queue.push_back(open_valve_solution);
                    }
                } else {
                    solutions_queue.push_back(open_valve_solution);
                }
            }

            for (destination_name, &distance) in current_valve.tunnels_lead_to.iter() {
                if current_solution.opened_valves.contains(destination_name)
                    || (time_limit - current_solution.current_time) < distance
                    || !all_valves.contains_key(destination_name)
                {
                    continue;
                }

                let new_solution = ProblemSolutionPart1 {
                    current_valve: destination_name.to_string(),
                    expected_release: current_solution.expected_release,
                    current_time: current_solution.current_time + distance,
                    opened_valves: current_solution.opened_valves.clone(),
                };

                let release_upper_bound = new_solution.release_upper_bound(
                    all_valves,
                    non_zero_valves_sorted,
                    time_limit,
                );

                if release_upper_bound < best_complete_release {
                    continue;
                }

                best_complete_release =
                    std::cmp::max(new_solution.expected_release, best_complete_release);

                if let Some(next_potential_solution) = solutions_queue.front() {
                    if new_solution.current_time >= next_potential_solution.current_time {
                        solutions_queue.push_front(new_solution);
                    } else {
                        solutions_queue.push_back(new_solution);
                    }
                } else {
                    solutions_queue.push_back(new_solution);
                }
            }
        }
        best_complete_release
    }

    let best_complete_release_part1 = solve(TIME_LIMIT_PART1, &all_valves, &non_zero_valves_sorted);

    println!("Part 1: {best_complete_release_part1}");

    const TIME_LIMIT_PART2: u64 = 26;

    let mut best_solution_per_subset: Vec<u64> =
        Vec::with_capacity(2_usize.pow(non_zero_valves_sorted.len() as u32));

    for selector in 0..2usize.pow(non_zero_valves_sorted.len() as u32) {
        // println!("Selector: {selector}");
        let mut subset: HashMap<String, Valve> = HashMap::new();
        subset.insert("AA".to_string(), all_valves.get("AA").unwrap().clone());
        for (valve_idx, &valve) in non_zero_valves_sorted.iter().enumerate() {
            let bit_selector = 1 << valve_idx;

            if (selector & bit_selector) >> valve_idx == 1 {
                subset.insert(valve.name.clone(), valve.clone());
            }
        }

        let mut subset_non_zero_valves_sorted: Vec<_> = subset
            .iter()
            .filter(|&x| x.1.pressure_release_per_minute != 0)
            .map(|x| x.1)
            .collect();
        subset_non_zero_valves_sorted.sort_by_key(|x| x.pressure_release_per_minute);
        subset_non_zero_valves_sorted.reverse();
        let subset_non_zero_valves_sorted = subset_non_zero_valves_sorted;

        let best_subset_solution = solve(TIME_LIMIT_PART2, &subset, &subset_non_zero_valves_sorted);
        best_solution_per_subset.push(best_subset_solution);
    }

    let subset_count = best_solution_per_subset.len();

    let complementary_mask = subset_count - 1;

    let mut part_2_solution = 0;

    for subset in 0..subset_count {
        let complementary_subset = (subset ^ complementary_mask) % subset_count;

        part_2_solution = std::cmp::max(
            part_2_solution,
            best_solution_per_subset[subset] + best_solution_per_subset[complementary_subset],
        );
    }

    println!("Part 2: {part_2_solution}");
}
