fn welcome_func() {
    println!("Welcome , do you know 2+2 = {} ", 2 + 2 ); //runing operations
}

fn vars() {
    let a :i32 = 56;
    let b :i32 = 44;
    println!("if a = {a} and b = {b} , therefore {a} + {b} = {}", {a} + {b});
}

fn main(){
    println!("hello User!");
    welcome_func();
    vars();
}
