

fn functional(limit : i32, vec : Vec<i32>) -> i32 {
  (1..limit)
    .filter(|x| vec.iter().any(|y| x % y == 0))
    .fold(0, |acc, x| acc + x)
}


fn interactive(limit : i32, vec : Vec<i32>) -> i32 {
  let mut sum = 0;

  for i in 1..limit {
    if vec.iter().any(|x| i % x == 0) {
      sum += i;
    }
  }

  return sum;
}

fn main() {
  let limit : i32 = 10;
  
  
  println!("{}", interactive(limit, vec![3,5]));
}