use std::collections::{HashMap, HashSet};
fn main() {
    let start_time = std::time::Instant::now();
    let input = include_str!("input.txt");
    //     let input = ".#.
    // ..#
    // ###";
    let mut gen = input
        .lines()
        .enumerate()
        .flat_map(|(y, s)| {
            s.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some((x as i32, y as i32, 0, 0))
                } else {
                    None
                }
            })
        })
        .collect::<HashSet<(i32, i32, i32, i32)>>();
    let mut maxX = i32::MIN;
    let mut maxY = i32::MIN;
    let mut minX = i32::MAX;
    let mut minY = i32::MAX;
    let mut maxZ = i32::MIN;
    let mut minZ = i32::MAX;
    let mut maxW = i32::MIN;
    let mut minW = i32::MAX;

    for &(x, y, z, w) in &gen {
        maxX = maxX.max(x);
        maxY = maxY.max(y);
        minX = minX.min(x);
        minY = minY.min(y);
        minZ = minZ.min(z);
        maxZ = maxZ.max(z);
        minW = minW.min(z);
        maxW = maxW.max(z);
    }

    for _ in 1..7 {
        let mut newGen = HashSet::new();

        let mut newMaxX = i32::MIN;
        let mut newMaxY = i32::MIN;
        let mut newMinX = i32::MAX;
        let mut newMinY = i32::MAX;
        let mut newMaxZ = i32::MIN;
        let mut newMinZ = i32::MAX;
        let mut newMaxW = i32::MIN;
        let mut newMinW = i32::MAX;

        for x in minX - 1..maxX + 2 {
            for y in minY - 1..maxY + 2 {
                for z in minZ - 1..maxZ + 2 {
                    for w in minW - 1..maxW + 2 {
                        let mut count = 0;
                        for dx in -1..2 {
                            for dy in -1..2 {
                                for dz in -1..2 {
                                    for dw in -1..2 {
                                        if dx != 0 || dy != 0 || dz != 0 || dw != 0 {
                                            count += gen.contains(&(x + dx, y + dy, z + dz, w + dw))
                                                as i32;
                                        }
                                    }
                                }
                            }
                        }
                        if gen.contains(&(x, y, z, w)) {
                            if count == 2 || count == 3 {
                                newGen.insert((x, y, z, w));
                                newMaxX = newMaxX.max(x);
                                newMaxY = newMaxY.max(y);
                                newMinX = newMinX.min(x);
                                newMinY = newMinY.min(y);
                                newMinZ = newMinZ.min(z);
                                newMaxZ = newMaxZ.max(z);
                                newMinW = newMinW.min(z);
                                newMaxW = newMaxW.max(z);
                            }
                        } else {
                            if count == 3 {
                                newGen.insert((x, y, z, w));
                                newMaxX = newMaxX.max(x);
                                newMaxY = newMaxY.max(y);
                                newMinX = newMinX.min(x);
                                newMinY = newMinY.min(y);
                                newMinZ = newMinZ.min(z);
                                newMaxZ = newMaxZ.max(z);
                                newMinW = newMinW.min(z);
                                newMaxW = newMaxW.max(z);
                            }
                        }
                    }
                }
            }
        }
        gen = newGen;
        maxX = newMaxX;
        maxY = newMaxY;
        minX = newMinX;
        minY = newMinY;
        maxZ = newMaxZ;
        minZ = newMinZ;
        maxW = newMaxW;
        minW = newMinW;
    }
    println!("{}", gen.iter().count());
    let end_time = std::time::Instant::now();
    println!(
        "{}",
        end_time
            .checked_duration_since(start_time)
            .unwrap()
            .as_millis()
    );
}
