const AUTHOR: &str ="SHAHEEM";
fn main()
{   let x=[1,2,3,4,5,6]; //cannot have multiple mutable reference (can have immutable ones)
    let y=&x[..4];
    let z=&x[1..];
    println!("{x:?}");
    println!("{y:?}");
    println!("{z:?}");
    // y[2]=5;
    println!("{x:?}");
    println!("{y:?}");
    println!("{z:?}");
}
