/*
 * Learn Rust The Hard Way: Double Linked List
 * 2020/7/6
 * hustccc
 * Manjaro
 */
#[derive(Debug)]
pub struct DListNode {
    pub value: i32,
    pub next: Option<Box<DListNode>>,
    pub prev: Option<Box<DListNode>>
}

impl DListNode {
    pub fn new(value: i32) -> DListNode {
        DListNode{
            value: value,
            next: None,
            prev: None
        }
    }

    //get the last node
    pub fn get_last(&mut self) -> &mut DListNode {
        if let Some(ref mut temp_box) = self.next {
            return temp_box.get_last();
        }
        else {
            return self;
        }
    }

    pub fn append(&mut self, new_value: i32) {
        let temp_node = DListNode::new(new_value);
        let last_node = DListNode::new(self.get_last().value);
        self.get_last().next = Some(Box::new(temp_node));
        self.get_last().prev = Some(Box::new(last_node));
    }
    pub fn get_middle(head_node: Option<Box<DListNode>>) -> Option<Box<DListNode>> {
        let mut head = head_node;
        let mut temp_node = &head;
        let mut len = 0;//len of linked list
        let mut count = 0;
        while let Some(_node) = temp_node {
            temp_node = &_node.next;
            len += 1;
        }
        len = match len%2 {
            0 => len/2,
            _ => (len-1)/2,
        };
        while count < len {
            if let Some(_node) = head {
                head = _node.next;
            }
            count += 1;
        }
        head
    }
    pub fn find_next(head_node: Option<Box<DListNode>>, value: i32) -> Option<i32> {
        let mut head = head_node;
        let mut temp_node = &head;
        while let Some(_node) = temp_node {
            if (&_node).value == value {
                match &_node.next {
                    None => {
                        println!("the last node");
                        return  None;
                    },
                    Some(next_node) => {
                        return Some(next_node.value);
                    }
                }
            }
            temp_node = &_node.next;
        }
        None
    }

    

}

