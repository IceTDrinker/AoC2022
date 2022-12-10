mod days;

const DAY_FUNCS: [fn() -> (); 6] = [
    days::day_01::day_01,
    days::day_02::day_02,
    days::day_03::day_03,
    days::day_04::day_04,
    days::day_05::day_05,
    days::day_06::day_06,
];

fn main() {
    let day_arg: usize = std::env::args()
        .skip(1)
        .take(1)
        .next()
        .expect("Expected the number of the day as first argument")
        .parse()
        .expect("Expected an integer.");

    let day_func = DAY_FUNCS.get(day_arg - 1).expect(&format!(
        "Day argument out of range, max: {}",
        DAY_FUNCS.len()
    ));

    day_func()
}
