mod days;

const DAY_FUNCS: [fn() -> (); 17] = [
    days::day_01::day_01,
    days::day_02::day_02,
    days::day_03::day_03,
    days::day_04::day_04,
    days::day_05::day_05,
    days::day_06::day_06,
    days::day_07::day_07,
    days::day_08::day_08,
    days::day_09::day_09,
    days::day_10::day_10,
    days::day_11::day_11,
    days::day_12::day_12,
    days::day_13::day_13,
    days::day_14::day_14,
    days::day_15::day_15,
    days::day_16::day_16,
    days::day_17::day_17,
];

fn main() {
    let day_arg: usize = std::env::args()
        .skip(1)
        .take(1)
        .next()
        .expect("Expected the number of the day as first argument")
        .parse()
        .expect("Expected an integer.");

    let day_func = DAY_FUNCS
        .get(day_arg - 1)
        .unwrap_or_else(|| panic!("Day argument out of range, max: {}", DAY_FUNCS.len()));

    day_func()
}
