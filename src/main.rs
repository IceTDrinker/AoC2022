mod days;

const DAY_FUNCS: [fn() -> (); 1] = [days::day_01::day_01];

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
