/*
 * Learn Rust The Hard Way: Double Linked List
 * 2020/7/6
 * hustccc
 * Manjaro
 */
/*
 pub struct DListNode {
     value: i32,
     prev:  Box<DListNode>,
     next:  Box<DListNode>
 }*/
 pub struct DListNode {
    value: i32,
    prev:  Option<&DListNode>,
    next:  Option<&DListNode>
}

 pub struct DList {
     count: i32,
     first: Option<&DListNode>,
     last:  Option<&DListNode>
 }

 pub impl DListNode {
     fn new (value: 132, prev: Option<&DListNode>, next: Option<&DListNode>) -> DListNode {
         let new_dlistnode = DListNode {
            value: value, 
            prev: prev,
            next: next
         };
         new_dlistnode
     }
 }