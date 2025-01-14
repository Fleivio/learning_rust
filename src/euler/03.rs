fn prime_factors(val : u64) -> Vec<u64> {
  let mut current = val;
  let mut factor = 2;
  let mut acc_factors = Vec::new();

  while factor <= current {
    if current % factor == 0 {
      current = current / factor;
      acc_factors.push(factor);
    } else {
      factor += 1;
    }
  }

  return acc_factors;
}

fn main() {
  let num : u64 = 600851475143;

  let vec = prime_factors(num);
  print!("{:?}", vec.iter().max());

}