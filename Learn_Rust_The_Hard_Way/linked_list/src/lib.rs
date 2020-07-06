/*
 * Learn Rust The Hard Way: Linked List
 * 2020/7/6
 * hustccc
 * Manjaro
 */

#[derive(Debug)]
pub struct ListNode {
    pub value: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    pub fn new(value: i32) -> ListNode {
        ListNode{
            value: value,
            next: None
        }
    }

    //get the last node
    pub fn get_last(&mut self) -> &mut ListNode {
        if let Some(ref mut temp_box) = self.next {
            return temp_box.get_last();
        }
        else {
            return self;
        }
    }

    //get the middle node
    pub fn get_middle(head_node: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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

    //add new node
    pub fn append(&mut self, new_value: i32) {
        let temp_node = ListNode::new(new_value);
        self.get_last().next = Some(Box::new(temp_node));
    }

    //overturn the linked list
    pub fn overturn_list(head_node: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev_node = None;//prev node
        let mut cur_node= head_node;//current node
        while let Some(mut temp_node) = cur_node {
            cur_node = temp_node.next.take();
            temp_node.next = prev_node;
            prev_node = Some(temp_node);
        }
        prev_node
    }


}
