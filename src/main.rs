use std::io;
fn main() {
    println!("What is the Square of  Number");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("");
    let num1:i32 = num1.trim().parse().unwrap();

    println!("What is the Root of  Number");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("");
    let num2:i32 = num2.trim().parse().unwrap();

    println!("The Square is :{}",square_of_num(num1));
    
    println!("The root is :{}",root_of_num(num2));



}
            //for Square//
 fn square_of_num(num1:i32)->i32{
     num1*num1
 }

        //Root//
  fn root_of_num(num2:i32)->i32{
     num2*num2*num2
 }