fn main() {
    println!("Q3");

    // print area
    let s = Square{
        width: 10,
        height: 20,
    };
    printarea(&s)
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
