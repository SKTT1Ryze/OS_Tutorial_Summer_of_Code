/*
 * Learning Rust: test
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
    fn can_hold(&self,other: &Rectangle)->bool{
        self.width>other.width&&self.height>other.height
    }
}
pub struct Guess{
    value: i32,
}
impl Guess{
    pub fn new(value: i32)->Guess{
        if value <1||value>100{
            panic!("Guess value must be between 1 and 100, got {}.",value);
        }
        Guess{
            value
        }
    }
}
pub fn add_two(a: i32)->i32{
    a+2
}
pub fn greeting(name: &str)->String{
    format!("hello {}!",name)
    //String::from("hello!")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }
    /*
    #[test]
    fn another(){
        panic!("Make the test fail");
    }*/
    #[test]
    fn larger_can_hold_smaller(){
        let larger=Rectangle{width: 8,height: 7};
        let smaller=Rectangle{width: 5,height: 1};
        assert!(larger.can_hold(&smaller));
    }
    /*
    #[test]
    fn it_adds_two(){
        assert_eq!(4,add_two(2));
    }
    */
    #[test]
    fn greeting_contains_name(){
        let result=greeting("hustccc");
        assert!(result.contains("hustccc"),"Greeting did not contain name,value was '{}'",result );
    }
    #[test]
    //#[should_panic(expected = "Guess value must be less than or equal to 100")]
    #[should_panic]
    fn greater_than_100(){
        Guess::new(200);
    }
    #[test]
    fn it_works()->Result<(),String>{
        if 2 + 2 == 4{
            Ok(())
        }
        else{
            Err(String::from("two plus two does not equal four"))
        }
    }
}
