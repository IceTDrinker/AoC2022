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
pub fn day_16() {
    let data = load_file(16);

    use std::collections::{HashMap, HashSet, VecDeque};

    #[derive(Debug, Clone)]
    struct Valve {
        name: String,
        pressure_release_per_minute: u64,
        tunnels_lead_to: Vec<String>,
    }

    impl Valve {
        pub fn new(name: &str, pressure_release_per_minute: u64, tunnels: &[String]) -> Self {
            Self {
                name: name.to_string(),
                pressure_release_per_minute,
                tunnels_lead_to: tunnels.to_owned(),
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
            let tunnels: Vec<String> = tunnels
                .trim_start_matches('s')
                .split(',')
                .into_iter()
                .map(|x| x.trim().to_string())
                .collect();

            Ok(Valve::new(name, flow, &tunnels))
        }
    }

    let mut all_valves = HashMap::new();

    for valve_str in data.trim().split('\n') {
        let valve: Valve = valve_str.parse().unwrap();
        all_valves.insert(valve.name.clone(), valve);
    }

    let mut non_zero_valves_sorted: Vec<_> = all_valves
        .iter()
        .filter(|&x| x.1.pressure_release_per_minute != 0)
        .map(|x| x.1)
        .collect();
    non_zero_valves_sorted.sort_by_key(|x| x.pressure_release_per_minute);
    non_zero_valves_sorted.reverse();
    let non_zero_valves_sorted = non_zero_valves_sorted;

    const TIME_LIMIT: u64 = 30;

    struct ProblemSolution {
        pub current_valve: String,
        pub expected_release: u64,
        pub current_time: u64,
        pub opened_valves: HashSet<String>,
    }

    impl ProblemSolution {
        pub fn is_complete(&self) -> bool {
            self.current_time == TIME_LIMIT
        }

        pub fn release_upper_bound(
            &self,
            all_valves: &HashMap<String, Valve>,
            non_zero_valves_sorted: &[&Valve],
        ) -> u64 {
            let mut release_upper_bound = self.expected_release;
            let mut remaining_time = TIME_LIMIT - self.current_time;

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

    let mut solutions_queue = VecDeque::new();
    solutions_queue.push_back(ProblemSolution {
        current_valve: "AA".to_string(),
        expected_release: 0,
        current_time: 0,
        opened_valves: HashSet::new(),
    });

    let mut best_complete_release = 0;

    while let Some(current_solution) = solutions_queue.pop_front() {
        if current_solution.is_complete() {
            best_complete_release =
                std::cmp::max(best_complete_release, current_solution.expected_release);
            continue;
        }

        let current_valve = all_valves.get(&current_solution.current_valve).unwrap();

        if current_valve.pressure_release_per_minute != 0
            && (current_solution.current_time < TIME_LIMIT - 1)
            && !current_solution.opened_valves.contains(&current_valve.name)
        {
            // +1 as we open the valve
            let new_time = current_solution.current_time + 1;
            let mut new_opened_valves = current_solution.opened_valves.clone();
            new_opened_valves.insert(current_valve.name.clone());

            let new_release = current_solution.expected_release
                + (TIME_LIMIT - new_time) * current_valve.pressure_release_per_minute;
            let open_valve_solution = ProblemSolution {
                current_valve: current_solution.current_valve.clone(),
                expected_release: new_release,
                current_time: new_time,
                opened_valves: new_opened_valves,
            };

            let release_upper_bound =
                open_valve_solution.release_upper_bound(&all_valves, &non_zero_valves_sorted);

            if release_upper_bound < best_complete_release {
                continue;
            }

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

        for destination_name in current_valve.tunnels_lead_to.iter() {
            // +1 as we move once
            let new_solution = ProblemSolution {
                current_valve: destination_name.to_string(),
                expected_release: current_solution.expected_release,
                current_time: current_solution.current_time + 1,
                opened_valves: current_solution.opened_valves.clone(),
            };

            let release_upper_bound =
                new_solution.release_upper_bound(&all_valves, &non_zero_valves_sorted);

            if release_upper_bound < best_complete_release {
                continue;
            }

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

    println!("Part 1: {best_complete_release}");
}
