use super::load_file;

/// --- Day 15: Beacon Exclusion Zone ---
/// You feel the ground rumble again as the distress signal leads you to a large network of
/// subterranean tunnels. You don't have time to search them all, but you don't need to: your pack
/// contains a set of deployable sensors that you imagine were originally built to locate lost
/// Elves.
///
/// The sensors aren't very powerful, but that's okay; your handheld device indicates that you're
/// close enough to the source of the distress signal to use them. You pull the emergency sensor
/// system out of your pack, hit the big button on top, and the sensors zoom off down the tunnels.
///
/// Once a sensor finds a spot it thinks will give it a good reading, it attaches itself to a hard
/// surface and begins monitoring for the nearest signal source beacon. Sensors and beacons always
/// exist at integer coordinates. Each sensor knows its own position and can determine the position
/// of a beacon precisely; however, sensors can only lock on to the one beacon closest to the sensor
/// as measured by the Manhattan distance. (There is never a tie where two beacons are the same
/// distance to a sensor.)
///
/// It doesn't take long for the sensors to report back their positions and closest beacons (your
/// puzzle input). For example:
///
/// Sensor at x=2, y=18: closest beacon is at x=-2, y=15
/// Sensor at x=9, y=16: closest beacon is at x=10, y=16
/// Sensor at x=13, y=2: closest beacon is at x=15, y=3
/// Sensor at x=12, y=14: closest beacon is at x=10, y=16
/// Sensor at x=10, y=20: closest beacon is at x=10, y=16
/// Sensor at x=14, y=17: closest beacon is at x=10, y=16
/// Sensor at x=8, y=7: closest beacon is at x=2, y=10
/// Sensor at x=2, y=0: closest beacon is at x=2, y=10
/// Sensor at x=0, y=11: closest beacon is at x=2, y=10
/// Sensor at x=20, y=14: closest beacon is at x=25, y=17
/// Sensor at x=17, y=20: closest beacon is at x=21, y=22
/// Sensor at x=16, y=7: closest beacon is at x=15, y=3
/// Sensor at x=14, y=3: closest beacon is at x=15, y=3
/// Sensor at x=20, y=1: closest beacon is at x=15, y=3
/// So, consider the sensor at 2,18; the closest beacon to it is at -2,15. For the sensor at 9,16,
/// the closest beacon to it is at 10,16.
///
/// Drawing sensors as S and beacons as B, the above arrangement of sensors and beacons looks like
/// this:
///
///                1    1    2    2
///      0    5    0    5    0    5
///  0 ....S.......................
///  1 ......................S.....
///  2 ...............S............
///  3 ................SB..........
///  4 ............................
///  5 ............................
///  6 ............................
///  7 ..........S.......S.........
///  8 ............................
///  9 ............................
/// 10 ....B.......................
/// 11 ..S.........................
/// 12 ............................
/// 13 ............................
/// 14 ..............S.......S.....
/// 15 B...........................
/// 16 ...........SB...............
/// 17 ................S..........B
/// 18 ....S.......................
/// 19 ............................
/// 20 ............S......S........
/// 21 ............................
/// 22 .......................B....
/// This isn't necessarily a comprehensive map of all beacons in the area, though. Because each
/// sensor only identifies its closest beacon, if a sensor detects a beacon, you know there are no
/// other beacons that close or closer to that sensor. There could still be beacons that just happen
/// to not be the closest beacon to any sensor. Consider the sensor at 8,7:
///
///                1    1    2    2
///      0    5    0    5    0    5
/// -2 ..........#.................
/// -1 .........###................
///  0 ....S...#####...............
///  1 .......#######........S.....
///  2 ......#########S............
///  3 .....###########SB..........
///  4 ....#############...........
///  5 ...###############..........
///  6 ..#################.........
///  7 .#########S#######S#........
///  8 ..#################.........
///  9 ...###############..........
/// 10 ....B############...........
/// 11 ..S..###########............
/// 12 ......#########.............
/// 13 .......#######..............
/// 14 ........#####.S.......S.....
/// 15 B........###................
/// 16 ..........#SB...............
/// 17 ................S..........B
/// 18 ....S.......................
/// 19 ............................
/// 20 ............S......S........
/// 21 ............................
/// 22 .......................B....
/// This sensor's closest beacon is at 2,10, and so you know there are no beacons that close or
/// closer (in any positions marked #).
///
/// None of the detected beacons seem to be producing the distress signal, so you'll need to work
/// out where the distress beacon is by working out where it isn't. For now, keep things simple by
/// counting the positions where a beacon cannot possibly be along just a single row.
///
/// So, suppose you have an arrangement of beacons and sensors like in the example above and, just
/// in the row where y=10, you'd like to count the number of positions a beacon cannot possibly
/// exist. The coverage from all sensors near that row looks like this:
///
///                  1    1    2    2
///        0    5    0    5    0    5
///  9 ...#########################...
/// 10 ..####B######################..
/// 11 .###S#############.###########.
/// In this example, in the row where y=10, there are 26 positions where a beacon cannot be present.
///
/// Consult the report from the sensors you just deployed. In the row where y=2000000, how many
/// positions cannot contain a beacon?
///
/// --- Part Two ---
/// Your handheld device indicates that the distress signal is coming from a beacon nearby. The
/// distress beacon is not detected by any sensor, but the distress beacon must have x and y
/// coordinates each no lower than 0 and no larger than 4000000.
///
/// To isolate the distress beacon's signal, you need to determine its tuning frequency, which can
/// be found by multiplying its x coordinate by 4000000 and then adding its y coordinate.
///
/// In the example above, the search space is smaller: instead, the x and y coordinates can each be
/// at most 20. With this reduced search area, there is only a single position that could have a
/// beacon: x=14, y=11. The tuning frequency for this distress beacon is 56000011.
///
/// Find the only possible position for the distress beacon. What is its tuning frequency?
pub fn day_15() {
    let data = load_file(15);

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
    struct Pos {
        pub x: i64,
        pub y: i64,
    }

    impl Pos {
        pub fn distance_to(&self, other: &Pos) -> u64 {
            self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
        }
    }

    #[derive(Debug)]
    struct Sensor {
        pub pos: Pos,
        pub closest_beacon: Pos,
    }

    impl Sensor {
        pub fn distance_to_closest_beacon(&self) -> u64 {
            self.pos.distance_to(&self.closest_beacon)
        }
    }

    use std::str::FromStr;

    impl FromStr for Pos {
        type Err = Box<dyn std::error::Error>;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (x, y) = s.trim().split_once(',').unwrap();
            let (_, x) = x.split_once("x=").unwrap();
            let (_, y) = y.split_once("y=").unwrap();

            Ok(Self {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            })
        }
    }

    impl FromStr for Sensor {
        type Err = Box<dyn std::error::Error>;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (sensor_coords_str, beacon_coords_str) = s.trim().split_once(':').unwrap();

            let (_, sensor_coords_str) = sensor_coords_str.split_once("at").unwrap();
            let sensor_pos: Pos = sensor_coords_str.parse().unwrap();

            let (_, beacon_coords_str) = beacon_coords_str.split_once("at ").unwrap();
            let beacon_pos: Pos = beacon_coords_str.parse().unwrap();

            Ok(Self {
                pos: sensor_pos,
                closest_beacon: beacon_pos,
            })
        }
    }

    let mut sensors: Vec<Sensor> = data
        .trim()
        .split('\n')
        .into_iter()
        .map(|x| x.parse().unwrap())
        .collect();

    // Sort by x
    sensors.sort_by_key(|sensor| sensor.pos.x);

    let sensors = sensors;

    let (min_x, max_x) = sensors.iter().fold((i64::MAX, i64::MIN), |acc, sensor| {
        let (mut min_x, mut max_x) = acc;

        min_x = std::cmp::min(
            sensor.pos.x - sensor.distance_to_closest_beacon() as i64,
            min_x,
        );
        max_x = std::cmp::max(
            sensor.pos.x + sensor.distance_to_closest_beacon() as i64,
            max_x,
        );

        (min_x, max_x)
    });

    let y = 2_000_000;
    let mut no_beacon_locations = 0;
    let mut x = min_x;
    'outer: while x <= max_x {
        let potential_beacon = Pos { x, y };

        for sensor in sensors.iter() {
            let distance_to_closest_beacon = sensor.distance_to_closest_beacon();
            let distance_to_potential_beacon = sensor.pos.distance_to(&potential_beacon);

            if distance_to_potential_beacon <= distance_to_closest_beacon {
                let x_half_span: i64 = (distance_to_closest_beacon - y.abs_diff(sensor.pos.y))
                    .try_into()
                    .unwrap();
                let x_diff = sensor.pos.x - x;
                let x_skip = x_diff + x_half_span + 1;
                assert!(x_skip >= 1);
                x += x_skip;

                no_beacon_locations += x_skip;
                continue 'outer;
            }
        }

        x += 1;
    }

    let mut beacons_at_y = std::collections::HashSet::new();
    beacons_at_y.extend(
        sensors
            .iter()
            .filter(|sensor| sensor.closest_beacon.y == y)
            .map(|sensor| sensor.closest_beacon),
    );

    no_beacon_locations -= beacons_at_y.len() as i64;

    println!("Part 1: {no_beacon_locations}");

    let beacons: std::collections::HashSet<Pos> =
        sensors.iter().map(|sensor| sensor.closest_beacon).collect();

    let (x_min, x_max) = (0, 4_000_000);
    let (y_min, y_max) = (0, 4_000_000);

    let mut tuning_f = 0;

    'outer: for y in y_min..=y_max {
        let mut x = x_min;
        'x_outer: while x <= x_max {
            let potential_beacon = Pos { x, y };

            for sensor in sensors.iter() {
                let distance_to_closest_beacon = sensor.distance_to_closest_beacon();
                let distance_to_potential_beacon = sensor.pos.distance_to(&potential_beacon);

                if distance_to_potential_beacon <= distance_to_closest_beacon {
                    let x_half_span: i64 = (distance_to_closest_beacon - y.abs_diff(sensor.pos.y))
                        .try_into()
                        .unwrap();
                    let x_diff = sensor.pos.x - x;
                    let x_skip = x_diff + x_half_span + 1;
                    assert!(x_skip >= 1);
                    x += x_skip;

                    continue 'x_outer;
                }
            }

            if !beacons.contains(&potential_beacon) {
                tuning_f = 4_000_000 * x + y;
                break 'outer;
            }

            x += 1
        }
    }

    println!("Part 2: {tuning_f}");
}
