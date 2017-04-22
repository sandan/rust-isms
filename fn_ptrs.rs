fn main(){
  
  let f: fn(u32) -> u32 = fib;
  // f is a function that returns an unsigned int given an unsigned int
  // -> f is a function pointer
  
  for x in 0 .. 10 {
    showfib(x, f)
  }
}

// takes u32 and a function pointer, returns nothing
fn showfib(i:u32, f: fn(u32) -> u32) -> () {
  println!("fib[{}] = {}", i, f(i))
}
// returns the nth fibonacci number
// n:      0 1 2 3 4 5 6 . . .
// fib(n): 0 1 1 2 3 5 8 . . .  
fn fib(n:u32)-> u32{
  if n == 0 {
    0
  } else if n == 1 || n == 2 {
    1
  } else if n > 2 {
    fib(n-1) + fib(n-2)
  }else {
    panic!("nth negative fibonacci number undefined");
  }
}
