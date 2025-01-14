

fn fibsum(limit : u64) -> u64 {
  let mut prev = 1;
  let mut act = 2;

  let mut sum = 0;

  while act <= limit {
    if act % 2 == 0 {
      sum += act;
    }
    
    act = prev + act;
    prev = act - prev;
  }

  return sum;
}

fn main() {
  let limit : i32 = 100;
  println!("{}", fibsum(limit as u64));
}