fn main() {
  let input = include_str!("input.txt");
  let result = part1(input);

  println!("{}", result);
}

// .inspect dbg!

fn part1(input: &str) -> String {
  let output: u32 = input
    .lines()
    .map(|line| {
      let mut it = line.chars().filter_map(|character| {
        character.to_digit(10)
      });

      let first = it.next().expect("should be a number");
      let last = it.last();

      match last {
        None => first * 10 + first,
        Some(num) => first * 10 + num
      }
    })
    .sum();

  output.to_string()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part1() {
    let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    assert_eq!("142", part1(input));
  }
}
