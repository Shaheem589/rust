fn main()
{   let mut x=0;
    outer:loop {
        println!("Entered the outer loop");
        inner: loop
        {   println!("Entered the outer loop");
            break

        }
        x+=2;
    }
}