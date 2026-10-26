use std::io;
fn main() {
    let mut input  = String::new();
    let mut buffer: Vec<&str>;
    let mut num1:f64;
    let mut num2:f64;
    loop{
        input.clear();
        io::stdin()
    .read_line(&mut input).expect("read your name");
    buffer = input.split_whitespace().collect();
    if buffer.len() <3{
        continue;
    }
    num1=match buffer[0].parse(){
        Ok(num)=>{
            num
        }
        Err(_)=>{
            println!("number symbor");
            continue;
        }
    };
    num2=match buffer[2].parse(){
        Ok(num)=>{
            num
        }
        Err(_)=>{
            println!("number symbor");
            continue;
        }
    };
    let result= match buffer[1]{
        "+"=>num1+num2,
        "-"=>num1-num2,
        "*"=>num1*num2,
        "/"=> if num2 != 0.0{
            num1/num2
        }
        else {
            println!("Cannot divide by zero");
            continue;
        }
        _ => 0.0,
    };
    println!("result = {}", result);
    }
}
