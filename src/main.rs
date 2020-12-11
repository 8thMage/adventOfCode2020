use std::collections::HashMap;
fn main() {
    let input = include_str!("input.txt");
//         let input =
// "16
// 10
// 15
// 5
// 1
// 11
// 7
// 19
// 6
// 12
// 4";
    let difference = input
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .max()
        .unwrap()
        + 3;
    let mut sorted_input: Vec<i32> = input
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    sorted_input.push(0);
    sorted_input.push(difference);
    sorted_input.sort();
    let mut count_one = 0;
    let mut count_three = 0;
    for x in 0..sorted_input.len() - 1 {
        if sorted_input[x] == sorted_input[x + 1] - 1 {
            count_one += 1;
        } else if sorted_input[x] != sorted_input[x + 1] - 2 {
            count_three += 1;
        }
    }
    println!("{} {} {}", count_one, count_three, count_one * count_three);
    println!("{}", number_of_connections(&sorted_input, &mut HashMap::new()));

}

fn number_of_connections(
    sorted_input: &[i32],
    ran_table: &mut HashMap<i32, i64>,
) -> i64 {
    if sorted_input.len() == 1 {
        return 1;
    }
    if ran_table.contains_key(&sorted_input[0]) {
        return ran_table[&sorted_input[0]];
    } else {
        let mut count = 0;
        for i in 1..4.min((sorted_input.len()) as i32) {
            if sorted_input[i as usize] < sorted_input[0] + 4 {
                count += number_of_connections(&sorted_input[i as usize..], ran_table);
            }
        }
        ran_table.insert(sorted_input[0], count);
        return count;
    }
}
