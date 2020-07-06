/*
 * Learn Rust The Hard Way: Linked List
 * 2020/7/6
 * hustccc
 * Manjaro
 */
//单链表，实现了创建，取中间节点，和翻转功能
use linked_list::ListNode;
fn main () {
    //create a linked list
    let mut linked_list_01 = ListNode::new(0);
    linked_list_01.append(1);
    linked_list_01.append(2);
    linked_list_01.append(3);
    linked_list_01.append(4);
    println!("linked list 01: {:#?}", linked_list_01);
    //overturn it
    let linked_list_02 = ListNode::overturn_list(Some(Box::new(linked_list_01)));
    println!("linked list 02: {:#?}", linked_list_02);
    //get middle node
    let middle_node_02 = ListNode::get_middle(linked_list_02);
    println!("the middle node of linked list 02: {:#?}", middle_node_02);
}