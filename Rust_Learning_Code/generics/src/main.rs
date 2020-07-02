/*
 * Learning Rust: generics
 * 2020/7/1
 * hustccc
 * Manjaro
 */
struct Point<T,U>{
    x: T,
    y: U,
}
impl<T,U> Point<T,U>{
    fn mixup<V,W>(self,other: Point<V,W>)->Point<T,W>{
        Point{
            x: self.x,
            y: other.y,
        }
    }
    
}

fn main(){
    //let both_integer=Point{x:5,y:10};
    //let both_float =Point{x:1.0,y:4.0};
    //let integer_and_float =Point{x:15,y:4.0};
    let p1 = Point{x:5,y:10.4};
    let p2 = Point{x:"Hello",y:'c'};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}",p3.x,p3.y);
}