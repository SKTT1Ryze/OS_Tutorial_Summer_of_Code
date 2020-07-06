/*
 * Learn Rust The Hard Way: Stack
 * 2020/7/6
 * hustccc
 * Manjaro
 */
#[derive(Debug)]
pub struct stack<T> {
    top: Option<Box<node<T>>>,
}
#[derive(Debug)]
struct node<T> {
    value: T,
    next: Option<Box<node<T>>>,
}
impl<T> stack<T> {
    pub fn new() -> stack<T>{
        stack {
            top: None
        }
    }

    pub fn push(&mut self, value: T) {
        let temp_node = Box::new(node{
            value: value,
            next: self.top.take(),
        });
        self.top = Some(temp_node);
    }
    pub fn pop(&mut self) -> Option<T> {
        let temp_node = self.top.take();
        match temp_node {
            None => {
                println!("stack empty");
                return None;
            }
            Some(top_node) => {
                self.top = top_node.next;
                return Some(top_node.value);
            }
        }
    }
}

fn main () {
    let mut stack_01 = stack::new();
    stack_01.push(0);
    stack_01.push(1);
    stack_01.push(2);
    stack_01.push(3);
    println!("stack 01 is: {:#?}", stack_01);
    if let Some(temp_value) = stack_01.pop() {
        println!("get top: {}",temp_value);
    };
    println!("stack 01 after pop is: {:#?}", stack_01);
}