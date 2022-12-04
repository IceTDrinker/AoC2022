pub fn load_file(day: usize) -> String {
    let path_as_string = format!("data/day_{day:02}.txt");
    let path = std::path::Path::new(&path_as_string);

    std::fs::read_to_string(path).unwrap()
}
