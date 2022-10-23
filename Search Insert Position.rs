fn main() {
  let arr = [1,2,3,4,5]; 
  let target = 1; 
  for i in arr{
    if i == target {
      println!("{:?}", arr.iter().position(|&r| r ==   i).unwrap());
    }
  }
}
