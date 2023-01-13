fn request() -> String {
 let mut value: String = String::new();

 std::io::stdin().read_line(&mut value).expect("Input failed");

 value = value.trim().to_string();
 value = value.replace("\n", "") ;
 value = value.replace("\r", "") ;

 value
}//fn request() -> String {

fn main() {
 loop {
  println!("\r\n\r\ntext:");

  let text: String = request();

  if &text[..] == "exit" {
   break;

  } else {//if &text[..] == "exit" {
   let     chars  : Vec<char> = text.chars().collect();
   let mut correct: bool      = true                  ;
   let mut stack  : Vec<char> = vec![]                ;
    
   for char in chars {
    match char {
     '(' => { stack.push(char) }
     '[' => { stack.push(char) }
     '{' => { stack.push(char) }

     ')' => if let Some(item) = stack.pop() { if item != '(' { correct = false; break; } } else { correct = false; break; }
     ']' => if let Some(item) = stack.pop() { if item != '[' { correct = false; break; } } else { correct = false; break; }
     '}' => if let Some(item) = stack.pop() { if item != '{' { correct = false; break; } } else { correct = false; break; }

     _   => ()
    }//match char {
   }//for char in chars {

   if stack.len() > 0 {
    println!("Error!");

   } else {//if stack.len() > 0 {
    if correct {
     println!("Correct!");

    } else {//if correct {
     println!("Error!");

    }//} else {//if correct {
   }//} else {//if stack.len() > 0 {
  }//} else {//if &text[..] == "exit" {
 }//loop {
}//fn main() {
