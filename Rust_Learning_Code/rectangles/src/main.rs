/*
 * Learning Rust: rectangles
 * 2020/7/2
 * hustccc
 * Manjaro
 */
#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}
impl Rectangle{
    fn area(&self)->u32{
        self.width*self.height
    }
    fn can_hold(&self,other: &Rectangle)->bool{
        self.width>other.width&&self.height>other.height
    }
    fn square(size: u32)->Rectangle{
        Rectangle{width:size,height:size}
    }
}

fn main()
{
    let rect1=Rectangle{width: 30,height: 50};
    println!("rect1 is {:#?}",rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
        );
    let rect2=Rectangle{width: 10,height: 40};
    let rect3=Rectangle{width: 90,height: 90};
    let rect4=Rectangle::square(100);
    println!("Can rect1 hold rect2? {}",rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}",rect1.can_hold(&rect3));
    println!(
        "The area of the rect4 is {} square pixels.",
        rect4.area()
        );

}
