//numericall solving the ODE y' = k - my where k and m are abibrary constants. y'=F(y), thus this DE is autonomous
/* Initial value: y(0) = y_0 
Since y_n = y_{n-1} + F(y_{n-1})*dx,
    y_n = y_{n-1} + (k-my_{n-1})*dx and since the DE is autonomous changing x will t be necessary in the numerical solution. 
    dy = (k-my_{n-1})*dx
    All notation is in LaTex*/
use std::io;
fn main() {
    let mut input:String;

    println!("Numericall solving the ODE y' = k - my where k and m are abibrary constants");
    //get input values
    println!("Enter value of:");
    println!("k:");
    input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let  k: f32 = input.trim().parse()
        .expect("Please type a number!");

    println!("m:");
    input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let  m: f32 = input.trim().parse()
        .expect("Please type a number!");

    println!("Initial value, y(0)=y_0:");
    input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let mut y: f32 = input.trim().parse()
        .expect("Please type a number!");

    println!("Step, dx:");
    input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let  dx: f32 = input.trim().parse()
        .expect("Please type a number!");
    //start the approximation
    for _ in 0..25{
        println!("y:{:.4}   dx:{}     k:{}      m:{}",y,dx,k,m);
        let dy:f32 = (k-m*y) * dx;// dy is the change in y after avery step, dx
        y = y + dy;
    }
}
