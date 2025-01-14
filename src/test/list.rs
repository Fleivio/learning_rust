

fn main() {
  println!("{}", (1..10).filter(|&x| x % 2 == 0).sum::<i32>());
  println!("{}", vec![1,2,3,4,5,6,7,8,9,10].iter().filter(|&x| x % 2 == 0).sum::<i32>());
}