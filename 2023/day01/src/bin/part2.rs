fn main() {
    let input = include_str!("input.txt");
    let result = part2(input);

    println!("{}", result);
}

// .inspect dbg!

fn part2(input: &str) -> String {
    let output: u32 = input.lines().map(process_line).sum();

    output.to_string()
}

fn process_line(line: &str) -> u32 {
    let mut index = 0;
    // build an iterator from a function which loops through the line and changes text based
    // representations of digits into characters, or just returns the next character at the
    // index we track for sliding right
    let line_iter = std::iter::from_fn(move || {
        let reduced_line = &line[index..];

        let result = if reduced_line.starts_with("one") {
            Some('1')
        } else if reduced_line.starts_with("two") {
            Some('2')
        } else if reduced_line.starts_with("three") {
            Some('3')
        } else if reduced_line.starts_with("four") {
            Some('4')
        } else if reduced_line.starts_with("five") {
            Some('5')
        } else if reduced_line.starts_with("six") {
            Some('6')
        } else if reduced_line.starts_with("seven") {
            Some('7')
        } else if reduced_line.starts_with("eight") {
            Some('8')
        } else if reduced_line.starts_with("nine") {
            Some('9')
        } else {
            let result = reduced_line.chars().next();
            result
        };

        index += 1;

        result
    });

    let mut it = line_iter.filter_map(|character| character.to_digit(10));

    let first = it.next().expect("should be a number");
    let last = it.last();

    match last {
        None => first * 10 + first,
        Some(num) => first * 10 + num,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    #[case("fivezg8twoneg", 51)]
    fn line_test(#[case] line: &str, #[case] expected: u32) {
        assert_eq!(expected, process_line(line))
    }

    #[test]
    fn test_part2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!("281", part2(input));
    }
}
