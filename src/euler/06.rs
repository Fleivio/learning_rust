
fn sum_of_squares(val : u32) -> u32 {
  (1..=val).map(|x| x.pow(2)).sum()
}

fn square_of_sum(val : u32) -> u32 {
  (1..=val).sum::<u32>().pow(2)
}

fn difference(val : u32) -> u32 {
  square_of_sum(val) - sum_of_squares(val)
}

fn main() {
  let val = 10;
  println!("{}", difference(100));
}