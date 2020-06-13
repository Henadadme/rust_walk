 /*
    Variable hold primitive data or reference to data
    Variable are immutable by default
    Rust is a block-scoped language
  */

 pub fn run(){
     let name = "James";
     let mut age = 21;

     age = 20;
     //remember you cant resign it without adding the keyword -> mut
     println!("My name is {} and I am {}", name, age);

     //defining constants;
     //you must add type to a constant
     const ID:i32 = 004 ;
     println!("ID:  {}", ID);

     //assign multiple variables;
     let (named, aged) = ("Toluwanimi", 21);
     println!("{}  is  {}", named, aged);
 }