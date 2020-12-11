fn main() {
    let input = include_str!("input.txt");
    //     let input =
    // "35
    // 20
    // 15
    // 25
    // 47
    // 40
    // 62
    // 55
    // 65
    // 95
    // 102
    // 117
    // 150
    // 182
    // 127
    // 219
    // 299
    // 277
    // 309
    // 576";
    let numberOfElements = 25;
    let vec = input
        .lines()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let mut heap = vec[0..numberOfElements].to_vec();
    heap.sort();
    let mut invalid = 0;
    for (i, x) in vec[numberOfElements..vec.len()].iter().enumerate() {
        let isOk = heap
            .iter()
            .enumerate()
            .filter(|(i, &n)| heap[i + 1..].binary_search(&(x - n)).is_ok()).count()
           != 0;
        if !isOk {
            invalid = *x;
            break;
        } else {
            heap.remove(heap.iter().position(|&n| n == vec[i]).unwrap());
            heap.insert(heap.iter().position(|&n| n >= *x).unwrap_or(heap.len()), *x);
        }
    }
        

    let scan = vec
        .iter()
        .scan(0i64, |acc, x| {
            *acc = *acc + x;
            Some(*acc - x)
        })
        .collect::<Vec<i64>>();
    let res = scan
        .iter()
        .enumerate()
        .map(|(index, &n)| {
            scan[index + 1..]
                .binary_search(&(invalid + n))
                .map(|s| (s + index + 1, index))
        })
        .filter(|s| s.is_ok())
        .next()
        .unwrap()
        .unwrap();
    print!(
        "{}, {}",
        invalid,
        vec[res.1..res.0].iter().min().unwrap() + vec[res.1..res.0].iter().max().unwrap()
    )
}
