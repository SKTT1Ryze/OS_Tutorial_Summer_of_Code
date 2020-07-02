/*
 * Learning Rust: cut modes
 * 2020/7/1
 * hustccc
 * Manjaro
 */
mod front_of_house;
pub use crate::front_of_house::hosting;
pub fn eat_at_restaurant(){
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
