use std::panic;

fn main() {
    println!("Q2");

    // u32 summary
    let xs: [i32; 5] = [1,2,3,4,i32::MAX];
    printsummary(&xs);

    let xt: [i32; 5] = [1,2,3,4,5];
    printsummary(&xt);
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
