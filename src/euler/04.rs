fn is_palindrome(num : u32) -> bool {
  let repr = num.to_string();

  return repr == repr.chars().rev().collect::<String>();
}

fn largest_pal(digits : u32) -> u32 {
  let mut largest = 0;

  for i in 1..10u32.pow(digits){
    for j in 1..10u32.pow(digits){
      if is_palindrome(i*j) && i*j > largest{
        largest = i*j;
      }
    }
  }

  largest
}

fn main(){
  print!("{}", largest_pal(2));
}