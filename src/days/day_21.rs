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
pub fn day_21() {
    let data = load_file(21);

    #[derive(Debug, PartialEq, Eq, Hash)]
    enum Monkey {
        Number(String, i64),
        Compute(String, (String, String), fn(i64, i64) -> i64),
    }

    impl Monkey {
        pub fn name(&self) -> String {
            match self {
                Monkey::Number(name, _) => name.clone(),
                Monkey::Compute(name, _, _) => name.clone(),
            }
        }
    }

    use std::str::FromStr;

    impl FromStr for Monkey {
        type Err = Box<dyn std::error::Error>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (monkey_name, num_or_op) = s.trim().split_once(':').unwrap();
            let num: Result<i64, _> = num_or_op.trim().parse();

            if let Ok(num) = num {
                return Ok(Monkey::Number(monkey_name.to_string(), num));
            }

            let mut op_iter = num_or_op.trim().split(' ');
            let lhs = op_iter.next().unwrap().to_string();
            let op = op_iter.next().unwrap();
            let rhs = op_iter.next().unwrap().to_string();

            let op_fn = match op {
                "+" => |x, y| x + y,
                "-" => |x, y| x - y,
                "*" => |x, y| x * y,
                "/" => |x, y| x / y,
                _ => unreachable!(),
            };

            Ok(Monkey::Compute(monkey_name.to_string(), (lhs, rhs), op_fn))
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
    let mut monkey_results: HashMap<String, i64> = HashMap::new();

    fn solve_monkeys(
        monkey_map: &HashMap<String, Monkey>,
        monkey_results: &mut HashMap<String, i64>,
        monkey_name: &String,
    ) -> i64 {
        if let Some(&monkey_result) = monkey_results.get(monkey_name) {
            monkey_result
        } else {
            let monkey = monkey_map.get(monkey_name).unwrap();
            let res = match monkey {
                Monkey::Number(_, number) => *number,
                Monkey::Compute(_, (lhs, rhs), op_fn) => {
                    let lhs_v = solve_monkeys(monkey_map, monkey_results, lhs);
                    let rhs_v = solve_monkeys(monkey_map, monkey_results, rhs);

                    op_fn(lhs_v, rhs_v)
                }
            };

            monkey_results.insert(monkey.name(), res);
            res
        }
    }

    let root_part_1 = solve_monkeys(&monkey_map, &mut monkey_results, &"root".to_string());

    println!("Part 1: {root_part_1}");
}
