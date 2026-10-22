use std::io;
fn main() {
    let mut input  = String::new();
    io::stdin()
    .read_line(&mut input).expect("read your name");
    let buffer: Vec<&str> = input.split_whitespace().collect();
    let mut num1:i32=0;
    let mut num2:i32=0;
    
    match buffer[0].parse::<i32>(){
        Ok(num)=>{
            num1= num;
        }
        Err(_)=>{
            print!("number symbor")
        }
    }
    match buffer[2].parse::<i32>(){
        Ok(num)=>{
            num2= num;
        }
        Err(_)=>{
            print!("number symbor")
        }
    }
    let result= match buffer[1]{
        "+"=>num1+num2,
        "-"=>num1-num2,
        _ => 0,
    };
    println!("result = {}", result);

}
