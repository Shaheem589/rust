trait Shape{
    fn perimeter(&self)->i32;
    fn area(&self)->i32;
}
struct Circle{
    radius: i32
}
impl Shape for Circle{
    fn perimeter(&self) -> i32{   
        2*3*self.radius
    }
    fn area(&self) -> i32{   
        3*self.radius*self.radius
    }
}
struct Square{
    side: i32
}
impl Shape for Square{
    fn perimeter(&self) -> i32{   
        4*self.side
    }
    fn area(&self) -> i32{   
        self.side*self.side
    }
}
fn print_perimeter(shape: &impl Shape)
{   let perimeter=shape.perimeter();
    println!("{perimeter}");
}
fn main()
{   let circle = Circle{
        radius: 1
    };
    print_perimeter(&circle);
    let square = Square{
        side: 100
    };
    print_perimeter(&square);
}