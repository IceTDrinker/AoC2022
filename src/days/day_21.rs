use super::load_file;

/// --- Day 21: Monkey Math ---
/// The monkeys are back! You're worried they're going to try to steal your stuff again, but it
/// seems like they're just holding their ground and making various monkey noises at you.
///
/// Eventually, one of the elephants realizes you don't speak monkey and comes over to interpret. As
/// it turns out, they overheard you talking about trying to find the grove; they can show you a
/// shortcut if you answer their riddle.
///
/// Each monkey is given a job: either to yell a specific number or to yell the result of a math
/// operation. All of the number-yelling monkeys know their number from the start; however, the math
/// operation monkeys need to wait for two other monkeys to yell a number, and those two other
/// monkeys might also be waiting on other monkeys.
///
/// Your job is to work out the number the monkey named root will yell before the monkeys figure it
/// out themselves.
///
/// For example:
///
/// root: pppw + sjmn
/// dbpl: 5
/// cczh: sllz + lgvd
/// zczc: 2
/// ptdq: humn - dvpt
/// dvpt: 3
/// lfqf: 4
/// humn: 5
/// ljgn: 2
/// sjmn: drzm * dbpl
/// sllz: 4
/// pppw: cczh / lfqf
/// lgvd: ljgn * ptdq
/// drzm: hmdt - zczc
/// hmdt: 32
/// Each line contains the name of a monkey, a colon, and then the job of that monkey:
///
/// A lone number means the monkey's job is simply to yell that number.
/// A job like aaaa + bbbb means the monkey waits for monkeys aaaa and bbbb to yell each of their
/// numbers; the monkey then yells the sum of those two numbers. aaaa - bbbb means the monkey yells
/// aaaa's number minus bbbb's number. Job aaaa * bbbb will yell aaaa's number multiplied by bbbb's
/// number. Job aaaa / bbbb will yell aaaa's number divided by bbbb's number.
/// So, in the above example, monkey drzm has to wait for monkeys hmdt and zczc to yell their
/// numbers. Fortunately, both hmdt and zczc have jobs that involve simply yelling a single number,
/// so they do this immediately: 32 and 2. Monkey drzm can then yell its number by finding 32 minus
/// 2: 30.
///
/// Then, monkey sjmn has one of its numbers (30, from monkey drzm), and already has its other
/// number, 5, from dbpl. This allows it to yell its own number by finding 30 multiplied by 5: 150.
///
/// This process continues until root yells a number: 152.
///
/// However, your actual situation involves considerably more monkeys. What number will the monkey
/// named root yell?
///
/// --- Part Two ---
/// Due to some kind of monkey-elephant-human mistranslation, you seem to have misunderstood a few
/// key details about the riddle.
///
/// First, you got the wrong job for the monkey named root; specifically, you got the wrong math
/// operation. The correct operation for monkey root should be =, which means that it still listens
/// for two numbers (from the same two monkeys as before), but now checks that the two numbers
/// match.
///
/// Second, you got the wrong monkey for the job starting with humn:. It isn't a monkey - it's you.
/// Actually, you got the job wrong, too: you need to figure out what number you need to yell so
/// that root's equality check passes. (The number that appears after humn: in your input is now
/// irrelevant.)
///
/// In the above example, the number you need to yell to pass root's equality test is 301. (This
/// causes root to get the same number, 150, from both of its monkeys.)
///
/// What number do you yell to pass root's equality test?
pub fn day_21() {
    let data = load_file(21);

    #[derive(Debug, PartialEq, Clone)]
    enum Op {
        Add(String, String),
        Sub(String, String),
        Mul(String, String),
        Div(String, String),
        Val(f64),
    }

    #[derive(Debug, PartialEq, Clone)]
    struct Monkey {
        pub name: String,
        pub op: Op,
    }

    impl Monkey {
        pub fn name(&self) -> String {
            self.name.clone()
        }

        pub fn op(&self) -> &Op {
            &self.op
        }
    }

    use std::str::FromStr;

    impl FromStr for Monkey {
        type Err = Box<dyn std::error::Error>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (monkey_name, num_or_op) = s.trim().split_once(':').unwrap();
            let num: Result<f64, _> = num_or_op.trim().parse();

            if let Ok(num) = num {
                return Ok(Monkey {
                    name: monkey_name.to_string(),
                    op: Op::Val(num),
                });
            }

            let mut op_iter = num_or_op.trim().split(' ');
            let lhs = op_iter.next().unwrap().to_string();
            let op = op_iter.next().unwrap();
            let rhs = op_iter.next().unwrap().to_string();

            let op_fn = match op {
                "+" => Op::Add(lhs, rhs),
                "-" => Op::Sub(lhs, rhs),
                "*" => Op::Mul(lhs, rhs),
                "/" => Op::Div(lhs, rhs),
                _ => unreachable!(),
            };

            Ok(Monkey {
                name: monkey_name.to_string(),
                op: op_fn,
            })
        }
    }

    let data_as_lines = data.trim().split('\n');

    use std::collections::HashMap;

    let monkey_map: HashMap<String, Monkey> = data_as_lines
        .map(|s| {
            let monkey: Monkey = s.parse().unwrap();
            let name = monkey.name();
            (name, monkey)
        })
        .collect();
    let mut monkey_results: HashMap<String, f64> = HashMap::new();

    fn solve_monkeys_part_1(
        monkey_map: &HashMap<String, Monkey>,
        monkey_results: &mut HashMap<String, f64>,
        monkey_name: &String,
    ) -> f64 {
        if let Some(&monkey_result) = monkey_results.get(monkey_name) {
            monkey_result
        } else {
            let monkey = monkey_map.get(monkey_name).unwrap();
            let res = if let Op::Val(val) = monkey.op() {
                *val
            } else {
                let (lhs, rhs) = match monkey.op() {
                    Op::Add(lhs, rhs)
                    | Op::Sub(lhs, rhs)
                    | Op::Mul(lhs, rhs)
                    | Op::Div(lhs, rhs) => (lhs, rhs),
                    _ => unreachable!(),
                };

                let lhs_val = solve_monkeys_part_1(monkey_map, monkey_results, lhs);
                let rhs_val = solve_monkeys_part_1(monkey_map, monkey_results, rhs);

                match monkey.op() {
                    Op::Add(_, _) => lhs_val + rhs_val,
                    Op::Sub(_, _) => lhs_val - rhs_val,
                    Op::Mul(_, _) => lhs_val * rhs_val,
                    Op::Div(_, _) => lhs_val / rhs_val,
                    _ => unreachable!(),
                }
            };

            monkey_results.insert(monkey.name(), res);
            res
        }
    }

    let root_part_1 = solve_monkeys_part_1(&monkey_map, &mut monkey_results, &"root".to_string());

    println!("Part 1: {root_part_1}");

    let root_monkey = monkey_map.get(&"root".to_string()).unwrap();
    let (lhs_root, rhs_root) = match root_monkey.op() {
        Op::Add(lhs, rhs) | Op::Sub(lhs, rhs) | Op::Mul(lhs, rhs) | Op::Div(lhs, rhs) => {
            (lhs.clone(), rhs.clone())
        }
        _ => unreachable!(),
    };

    // Like solve part 1 without memoization
    fn compute_monkeys(monkey_map: &HashMap<String, Monkey>, monkey_name: &String) -> f64 {
        let monkey = monkey_map.get(monkey_name).unwrap();
        let res = if let Op::Val(val) = monkey.op() {
            *val
        } else {
            let (lhs, rhs) = match monkey.op() {
                Op::Add(lhs, rhs) | Op::Sub(lhs, rhs) | Op::Mul(lhs, rhs) | Op::Div(lhs, rhs) => {
                    (lhs, rhs)
                }
                _ => unreachable!(),
            };

            let lhs_val = compute_monkeys(monkey_map, lhs);
            let rhs_val = compute_monkeys(monkey_map, rhs);

            match monkey.op() {
                Op::Add(_, _) => lhs_val + rhs_val,
                Op::Sub(_, _) => lhs_val - rhs_val,
                Op::Mul(_, _) => lhs_val * rhs_val,
                Op::Div(_, _) => lhs_val / rhs_val,
                _ => unreachable!(),
            }
        };

        res
    }

    let lhs = compute_monkeys(&monkey_map, &lhs_root);
    let rhs = compute_monkeys(&monkey_map, &rhs_root);

    let mut modified_monkey_map = monkey_map.clone();

    let humn_name = "humn".to_string();

    let mut lhs_constant = true;
    let mut rhs_constant = true;

    for idx in 0..i64::MAX {
        modified_monkey_map.insert(
            humn_name.clone(),
            Monkey {
                name: humn_name.clone(),
                op: Op::Val(idx as f64),
            },
        );
        let tmp_lhs = compute_monkeys(&modified_monkey_map, &lhs_root);
        let tmp_rhs = compute_monkeys(&modified_monkey_map, &rhs_root);

        if tmp_lhs != lhs {
            println!("lhs: {tmp_lhs} != {lhs}");
            lhs_constant = false;
        }

        if tmp_rhs != rhs {
            println!("lhs: {tmp_rhs} != {rhs}");
            rhs_constant = false;
        }

        if !lhs_constant || !rhs_constant {
            break;
        }
    }

    assert_ne!(lhs_constant, rhs_constant);

    let (eval_branch_name, target, _pruned_branch_name) = if !lhs_constant {
        (lhs_root, rhs, rhs_root)
    } else {
        (rhs_root, lhs, lhs_root)
    };

    let mut modified_monkey_map = monkey_map;

    let mut min_bound = 0f64;
    let mut max_bound = 1f64;

    modified_monkey_map.insert(
        humn_name.clone(),
        Monkey {
            name: humn_name.clone(),
            op: Op::Val(min_bound),
        },
    );
    let min_bound_eval = compute_monkeys(&modified_monkey_map, &eval_branch_name);

    modified_monkey_map.insert(
        humn_name.clone(),
        Monkey {
            name: humn_name.clone(),
            op: Op::Val(max_bound),
        },
    );
    let mut max_bound_eval = compute_monkeys(&modified_monkey_map, &eval_branch_name);

    while min_bound_eval == max_bound_eval {
        max_bound *= 2.0;

        modified_monkey_map.insert(
            humn_name.clone(),
            Monkey {
                name: humn_name.clone(),
                op: Op::Val(max_bound),
            },
        );
        max_bound_eval = compute_monkeys(&modified_monkey_map, &eval_branch_name);
    }

    let fn_increasing = min_bound_eval <= max_bound_eval;

    println!("{fn_increasing}");

    if target < max_bound_eval && !fn_increasing {
        while target < max_bound_eval {
            println!("{max_bound}");
            max_bound *= 2.0;

            modified_monkey_map.insert(
                humn_name.clone(),
                Monkey {
                    name: humn_name.clone(),
                    op: Op::Val(max_bound),
                },
            );
            max_bound_eval = compute_monkeys(&modified_monkey_map, &eval_branch_name);
        }
    } else {
        todo!("Code other cases yourself");
    }

    let mut res = 0.0;

    while (min_bound - max_bound).abs() > 1.0 {
        let mid = (min_bound + max_bound) / 2.0;

        modified_monkey_map.insert(
            humn_name.clone(),
            Monkey {
                name: humn_name.clone(),
                op: Op::Val(mid),
            },
        );

        let mid_eval = compute_monkeys(&modified_monkey_map, &eval_branch_name);

        println!("target: {target}, mid_eval: {mid_eval}");
        println!("min: {min_bound}, mid: {mid}, max: {max_bound}");

        if mid_eval == target {
            res = mid;
            break;
        }

        if !fn_increasing {
            if mid_eval >= target {
                min_bound = mid;
            } else {
                max_bound = mid;
            }
        } else {
            todo!("Code other cases yourself")
        }
    }

    if res == 0.0 {
        if min_bound_eval == target {
            res = min_bound;
        }
        if max_bound_eval == target {
            res = max_bound;
        }
    }

    loop {
        let new_res = res - 1.0;

        modified_monkey_map.insert(
            humn_name.clone(),
            Monkey {
                name: humn_name.clone(),
                op: Op::Val(new_res),
            },
        );

        let curr_eval = compute_monkeys(&modified_monkey_map, &eval_branch_name);
        if curr_eval != target {
            break;
        } else {
            res = new_res;
        }
    }

    println!("Part 2: {res}");
}
