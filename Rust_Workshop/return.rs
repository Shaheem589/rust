const AUTHOR: &str ="SHAHEEM";
fn main()
{   let x=x("nil");
    println!("{AUTHOR}: {x}");
}
fn x(st:&str)->i32
{   if st=="nil"
    {   return 0;
    }
    st.parse().unwrap()
}