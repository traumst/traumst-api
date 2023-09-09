pub struct LinkedList {
    head: Option<Box<Node>>
}

pub struct Node {
    value: u32,
    next: Option<Box<Node>>
}

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn push(&mut self, value: u32) {
        let new_node = Box::new(Node {
            value,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<u32> {
        match self.head.take() {
            Some(node) => {
                self.head = node.next;
                Some(node.value)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        while let Some(val) = list.pop() {
            println!("{}", val);
        }
    }
}