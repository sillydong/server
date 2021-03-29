fn main() {
    // print light timer
    println!("Q1");
    printtime(&Lights::Red);
    printtime(&Lights::Green);
    printtime(&Lights::Yellow);
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

