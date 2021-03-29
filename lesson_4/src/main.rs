use std::panic;

fn main() {
    // print light timer
    println!("Q1");
    printtime(&Lights::Red);
    printtime(&Lights::Green);
    printtime(&Lights::Yellow);

    println!("\nQ2");

    // u32 summary
    let xs: [i32; 5] = [1,2,3,4,i32::MAX];
    printsummary(&xs);

    let xt: [i32; 5] = [1,2,3,4,5];
    printsummary(&xt);

    println!("\nQ3");

    // print area
    let s = Square{
        width: 10,
        height: 20,
    };
    printarea(&s)
}

// Q1
fn printtime<T: LightTimer>(item: &T){
    println!("{}s",  item.time());
}

pub trait LightTimer {
    fn time(&self)->u8;
}

enum Lights {
    Red,
    Green,
    Yellow,
}

impl LightTimer for Lights {
    fn time(&self)->u8{
        match self {
            Lights::Red => 60,
            Lights::Green => 45,
            Lights::Yellow => 3,
        }
    }
}

// Q2
fn printsummary(slice: &[i32]){
    match summarize(slice){
        None => println!("overload"),
        Some(i) => println!("summary: {}", i),
    }
}

fn summarize(slice: &[i32])->Option<i32>{
    let result  = panic::catch_unwind(||{
        let mut sum = 0i32;
        for val in slice.iter(){
            sum += val;
        }
        sum
    });

    match result{
        Ok(v)=> Some(v),
        Err(_) => None,
    }
}

// Q3
fn printarea<T: Areable>(item: &T){
    println!("area is {}", item.calc())
}

pub trait Areable {
    fn calc(&self)->u32;
}

struct Square {
    width: u32,
    height: u32,
}

impl Areable for Square{
    fn calc(&self)->u32{
        return self.width * self.height
    }
}
