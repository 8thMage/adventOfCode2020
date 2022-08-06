fn main() {
    let input = include_str!("input.txt");
    let kernel: Vec<u8> = input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == '#' { 1 } else { 0 })
        .collect();
    // let image = vec![vec![1,1,1], vec![1,1,1], vec![1,1,1]];
    let mut image: Vec<Vec<u8>> = input
        .lines()
        .skip(2)
        .map(|s| s.chars().map(|c| if c == '#' { 1 } else { 0 }).collect())
        .collect();
    // for y in 0..image.len() {
    //     for x in 0..image.len() {
    //         print!("{}", if image[y][x] == 1 { '#' } else { '.' })
    //     }
    //     println!("");
    // }
    // println!("");

    let mut default = 0;
    for i in 0..50 {
        image = conv(&image, &kernel, default);
        default = kernel[default as usize * 511]
        // for y in 0..new_image.len() {
        //     for x in 0..new_image.len() {
        //         print!("{}", if new_image[y][x] == 1 { '#' } else { '.' })
        //     }
        //     println!("");
        // }
        // println!("");

        // let new_new_image = conv(&new_image, &kernel, kernel[default as usize * 511]);
    }
    println!(
        "{}",
        image.iter().fold(0_i32, |acc, v| acc
            + v.iter().fold(0, |acc, v| acc + *v as i32))
    );

    // for y in 0..new_new_image.len() {
    //     for x in 0..new_new_image.len() {
    //         print!("{}", if new_new_image[y][x] == 1 { '#' } else { '.' })
    //     }
    //     println!("");
    // }

    let x = 0;
}

fn conv(image: &Vec<Vec<u8>>, kernel: &Vec<u8>, default: u8) -> Vec<Vec<u8>> {
    let mut new_image = vec![];
    for y in 0..image.len() + 2 {
        new_image.push(vec![]);
        for x in 0..image[0].len() + 2 {
            let mut acc = 0_usize;
            for ky in 0..3 {
                for kx in 0..3 {
                    acc = acc * 2
                        + *image
                            .get((y + ky).wrapping_sub(2))
                            .and_then(|v| v.get((x + kx).wrapping_sub(2)))
                            .unwrap_or(&default) as usize
                }
            }
            new_image.last_mut().unwrap().push(kernel[acc]);
        }
    }
    new_image
}
