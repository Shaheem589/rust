use std::io::Write;
fn main()
{   std::io::stdout().flush().unwrap();
    let mut buf: String=String::new();
    std::io::stdin().read_line(& mut buf).unwrap();
    println!("{buf:?}");
    println!("{buf}");
    buf=buf.trim().to_string(); // removes new line
    println!("{buf:?}");
    println!("{buf}");
}