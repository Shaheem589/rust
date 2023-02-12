const AUTHOR: &str ="SHAHEEM";
fn main()
{   let x=x("-12");
    println!("{AUTHOR}: {x}");
}
fn x(st:&str)->i32
{   st.parse().unwrap()
}