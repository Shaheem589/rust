#[derive(Debug)]
struct Employee
{   name:String,
    age: i8
}
fn main()
{   let emp = Employee{name:"shaheem".to_string(),age:21};
    println!("{emp:?}");
}