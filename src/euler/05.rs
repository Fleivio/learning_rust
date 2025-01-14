fn primes_bellow(upper_bound : u32) -> Vec<u32> {
  let mut primes = Vec::new();
  
  for i in 2..=upper_bound{
    if primes.iter().any(|x| i % x == 0){
      continue
    }
    primes.push(i);
  }

  primes
}

fn smallest_evenly_divided(upper_bound : u32) -> u32 {
  let mut acc = 1;
  for i in primes_bellow(upper_bound).iter() {
    acc *= (0..).map(|j| i.pow(j))
                .take_while(|&ipowj| !(ipowj > upper_bound))
                .max()
                .unwrap();
  }
  acc
}

fn main(){
  println!("{:?}", smallest_evenly_divided(20));
}