fn main(){
  println!("Hello Universe!");
  println!("println! is a {}", "macro");
  // moustaches are used as placeolders
  
  println!("{} + {} = {}", 1,2,3);
  
  let x = 3; // bindings are immutable - like const in C
  //x=4;//error - re-assignment of immutable variable
  
  {
    //x = 21; not ok
    let x = 20; //ok since scope is different
  }
  
  println!("{}",x);
  
  //functions
  showadd(1,1, add(1,1));
}

// "void" function
fn showadd(a:i32, b:i32, c:i32) -> (){
  println!("{} + {} = {}", a, b, c);
}

// recursive
fn add(i:i32, times:i32) -> i32{
  if times == 0 {
    i //no ;
    //https://doc.rust-lang.org/book/functions.html#expressions-vs-statements
  }
  else if times < 0 {
    add(i - 1, times + 1 )
  }
  else {
    add(i + 1, times - 1)
  }
}
