/*
 * Learn Rust The Hard Way: Double Linked List
 * 2020/7/6
 * hustccc
 * Manjaro
 */
use double_linked_list::DListNode;
fn main() {
    println!("Double Linked List");
    let mut dlinked_list_01 = DListNode::new(0);
    dlinked_list_01.append(1);
    dlinked_list_01.append(2);
    dlinked_list_01.append(3);
    dlinked_list_01.append(4);
    println!("double linked list 01: {:#?}", dlinked_list_01);
    let middle_node_01 = DListNode::get_middle(Some(Box::new(dlinked_list_01)));
    println!("the middle node of double linked list 01: {:#?}", middle_node_01);
    let mut dlinked_list_02 = DListNode::new(0);
    dlinked_list_02.append(1);
    dlinked_list_02.append(2);
    dlinked_list_02.append(3);
    dlinked_list_02.append(4);
    dlinked_list_02.append(1);
    dlinked_list_02.append(2);
    dlinked_list_02.append(3);
    dlinked_list_02.append(4);
    println!("double linked list 01: {:#?}", dlinked_list_02);
    let find_value = 3;
    let next_node_value = DListNode::find_next(Some(Box::new(dlinked_list_02)), find_value);
    match next_node_value {
        None => {},
        Some(val) => {
            println!("the next value of node with value {} is: {}",find_value, val);
        }
    }
}

 
