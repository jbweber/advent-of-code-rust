fn main() {
  let sample = include_str!("input.txt");

  let result: u32 = sample
    .trim()
    .split("\n")
    .map(|l| {
      let digits = l.chars().filter(|c| c.is_digit(10)).collect::<Vec<_>>();
      let first = digits.first().unwrap();
      let last = digits.last().unwrap();
      let num = format!("{}{}", first, last);

      num.parse::<u32>().unwrap()
    })
    .sum();

  println!("{}", result);
}
