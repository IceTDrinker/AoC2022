use super::load_file;

/// --- Day 18: Boiling Boulders ---
/// You and the elephants finally reach fresh air. You've emerged near the base of a large volcano
/// that seems to be actively erupting! Fortunately, the lava seems to be flowing away from you and
/// toward the ocean.
///
/// Bits of lava are still being ejected toward you, so you're sheltering in the cavern exit a
/// little longer. Outside the cave, you can see the lava landing in a pond and hear it loudly
/// hissing as it solidifies.
///
/// Depending on the specific compounds in the lava and speed at which it cools, it might be forming
/// obsidian! The cooling rate should be based on the surface area of the lava droplets, so you take
/// a quick scan of a droplet as it flies past you (your puzzle input).
///
/// Because of how quickly the lava is moving, the scan isn't very good; its resolution is quite low
/// and, as a result, it approximates the shape of the lava droplet with 1x1x1 cubes on a 3D grid,
/// each given as its x,y,z position.
///
/// To approximate the surface area, count the number of sides of each cube that are not immediately
/// connected to another cube. So, if your scan were only two adjacent cubes like 1,1,1 and 2,1,1,
/// each cube would have a single side covered and five sides exposed, a total surface area of 10
/// sides.
///
/// Here's a larger example:
///
/// 2,2,2
/// 1,2,2
/// 3,2,2
/// 2,1,2
/// 2,3,2
/// 2,2,1
/// 2,2,3
/// 2,2,4
/// 2,2,6
/// 1,2,5
/// 3,2,5
/// 2,1,5
/// 2,3,5
/// In the above example, after counting up all the sides that aren't connected to another cube, the
/// total surface area is 64.
///
/// What is the surface area of your scanned lava droplet?
///
/// --- Part Two ---
/// Something seems off about your calculation. The cooling rate depends on exterior surface area,
/// but your calculation also included the surface area of air pockets trapped in the lava droplet.
///
/// Instead, consider only cube sides that could be reached by the water and steam as the lava
/// droplet tumbles into the pond. The steam will expand to reach as much as possible, completely
/// displacing any air on the outside of the lava droplet but never expanding diagonally.
///
/// In the larger example above, exactly one cube of air is trapped within the lava droplet (at
/// 2,2,5), so the exterior surface area of the lava droplet is 58.
///
/// What is the exterior surface area of your scanned lava droplet?
pub fn day_18() {
    let data = load_file(18);

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    struct Pos3D {
        pub x: i32,
        pub y: i32,
        pub z: i32,
    }

    use std::str::FromStr;

    impl FromStr for Pos3D {
        type Err = Box<dyn std::error::Error>;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (x, yz) = s.trim().split_once(',').unwrap();
            let (y, z) = yz.trim().split_once(',').unwrap();

            let x: i32 = x.parse().unwrap();
            let y: i32 = y.parse().unwrap();
            let z: i32 = z.parse().unwrap();

            Ok(Self { x, y, z })
        }
    }

    use std::collections::HashSet;

    let lava_cubes: HashSet<Pos3D> = data
        .trim()
        .split('\n')
        .into_iter()
        .map(|x| Pos3D::from_str(x).unwrap())
        .collect();

    let total_free_surface: u64 = lava_cubes.iter().fold(0, |mut acc, pos| {
        for diff_x in [-1, 1] {
            let neighbour = Pos3D {
                x: pos.x + diff_x,
                y: pos.y,
                z: pos.z,
            };
            if !lava_cubes.contains(&neighbour) {
                acc += 1;
            }
        }

        for diff_y in [-1, 1] {
            let neighbour = Pos3D {
                x: pos.x,
                y: pos.y + diff_y,
                z: pos.z,
            };
            if !lava_cubes.contains(&neighbour) {
                acc += 1;
            }
        }

        for diff_z in [-1, 1] {
            let neighbour = Pos3D {
                x: pos.x,
                y: pos.y,
                z: pos.z + diff_z,
            };
            if !lava_cubes.contains(&neighbour) {
                acc += 1;
            }
        }

        acc
    });

    println!("Part 1: {total_free_surface}");

    let (min_x, max_x, min_y, max_y, min_z, max_z) = lava_cubes.iter().fold(
        (i32::MAX, i32::MIN, i32::MAX, i32::MIN, i32::MAX, i32::MIN),
        |acc, pos| {
            let (min_x, max_x, min_y, max_y, min_z, max_z) = acc;

            (
                min_x.min(pos.x),
                max_x.max(pos.x),
                min_y.min(pos.y),
                max_y.max(pos.y),
                min_z.min(pos.z),
                max_z.max(pos.z),
            )
        },
    );

    // Safety
    let world_min_x = min_x - 1;
    let world_min_y = min_y - 1;
    let world_min_z = min_z - 1;
    let world_max_x = max_x + 1;
    let world_max_y = max_y + 1;
    let world_max_z = max_z + 1;

    let world_l = world_max_x - world_min_x + 1;
    let world_w = world_max_y - world_min_y + 1;
    let world_h = world_max_z - world_min_z + 1;

    let mut exterior_surface = 0;

    let mut visited: HashSet<Pos3D> = HashSet::new();
    let mut edge_cubes = std::collections::VecDeque::<Pos3D>::new();
    edge_cubes.push_back(Pos3D { x: 0, y: 0, z: 0 });

    while !edge_cubes.is_empty() {
        let current_cube = edge_cubes.pop_front().unwrap();

        if visited.contains(&current_cube) {
            continue;
        }

        visited.insert(current_cube);

        for (diff_x, diff_y, diff_z) in [
            (-1, 0, 0),
            (1, 0, 0),
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
        ] {
            let (x, y, z) = (
                current_cube.x + diff_x,
                current_cube.y + diff_y,
                current_cube.z + diff_z,
            );

            if x < -1 || x >= world_l || y < -1 || y >= world_w || z < -1 || z >= world_h {
                continue;
            }

            let neighbour = Pos3D { x, y, z };

            if lava_cubes.contains(&neighbour) {
                exterior_surface += 1;
            } else {
                edge_cubes.push_back(neighbour);
            }
        }
    }

    println!("Part 2: {exterior_surface}");
}
