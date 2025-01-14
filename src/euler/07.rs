
fn is_prime(val : u32) -> bool {
  if val == 1 { return false; }
  if val == 2 { return true; }

  let bound : u32 = (val as f32).powf(0.5).ceil() as u32;

  for i in 2..=bound {
    if val % i == 0 {
      return false
    }
  }

  true
}


fn primes() -> impl Iterator<Item = u32> {
  (0..).filter(|&x| is_prime(x))
}

fn main(){
  println!("{}", primes().nth(10001).unwrap());
}