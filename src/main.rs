fn main() {
    let input = include_str!("input.txt");
// let input = "
// pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
// hcl:#623a2f

// eyr:2029 ecl:blu cid:129 byr:1989
// iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

// hcl:#888785
// hgt:164cm byr:2001 iyr:2015 cid:88
// pid:545766238 ecl:hzl
// eyr:2022

// iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
// ";
    let needed: Vec<&str> = "byr
iyr
eyr
hgt
hcl
ecl
pid
cid"
    .split_whitespace()
    .collect();
    let inputPassport: Vec<&str> = input.split_terminator("\r\n\r\n").collect();
    // let inputPassport: Vec<&str> = input.split_terminator("\n\n").collect();
    let mut count = 0;
    for &passport in &inputPassport {
        // println!("{}", passport);
    }
    for &passport in &inputPassport {
        let mut valid: Vec<bool> = Vec::new();
        let mut valid_fields: Vec<&str> = Vec::new();
        for _ in 0..needed.len() {
            valid.push(false);
            valid_fields.push("");
        }
        let fields: Vec<Vec<&str>> = passport
            .split_whitespace()
            .map(|str| str.split_terminator(":").collect())
            .collect();
        for field in fields {
            for neededField in 0..needed.len() {
                if field[0] == needed[neededField] {
                    valid[neededField] = true;
                    valid_fields[neededField] = field[1];
                }
            }
        }
        if valid[0..needed.len() - 1].iter().all(|&x| x) {
            let valid3: bool = {
                let b;
                if (valid_fields[3].ends_with("in")) {
                    let number = &valid_fields[3][0..valid_fields[3].len() - 2];
                    if (number.parse::<i32>().is_ok()
                        && number.parse::<i32>().unwrap() >= 59
                        && number.parse::<i32>().unwrap() <= 76)
                    {
                        b = true;
                    } else {
                        b = false;
                    }
                } else if (valid_fields[3].ends_with("cm")) {
                    let number = &valid_fields[3][0..valid_fields[3].len() - 2];
                    if (number.parse::<i32>().is_ok()
                        && number.parse::<i32>().unwrap() >= 150
                        && number.parse::<i32>().unwrap() <= 193)
                    {
                        b = true;
                    } else {
                        b = false;
                    }
                }
                else {
                    b = false;
                }
                b
            };
            let eye_colors:Vec<&str> ="amb blu brn gry grn hzl oth".split_whitespace().collect();
            let validator = [
                valid_fields[0].len() == 4
                    && valid_fields[0].parse::<i32>().is_ok()
                    && valid_fields[0].parse::<i32>().unwrap() >= 1920
                    && valid_fields[0].parse::<i32>().unwrap() <= 2002,
                valid_fields[1].len() == 4
                    && valid_fields[1].parse::<i32>().is_ok()
                    && valid_fields[1].parse::<i32>().unwrap() >= 2010
                    && valid_fields[1].parse::<i32>().unwrap() <= 2020,
                valid_fields[2].len() == 4
                    && valid_fields[2].parse::<i32>().is_ok()
                    && valid_fields[2].parse::<i32>().unwrap() >= 2020
                    && valid_fields[2].parse::<i32>().unwrap() <= 2030,
                valid3,
                valid_fields[4].chars().next().unwrap() == '#' && valid_fields[4].chars().skip(1).all(|c| c>='0' && c<= '9' || c >='a' && c <='f') && valid_fields[4].len()==7,
                eye_colors.contains(&valid_fields[5]),
                valid_fields[6].len() ==9 && valid_fields[6].parse::<usize>().is_ok()
            ];
            if (validator.iter().all(|&x| x)) {
                count = count + 1;
            }
        };
    }
    println!("{}", count);
    // for line in inputLines{
    // map.
    // }
}
