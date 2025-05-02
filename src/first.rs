struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem: elem,
            // 将self.head持有数据的所有权转移给new_node.next,
            // 同时将self.head设置为Link::Empty
            next: std::mem::replace(&mut self.head, Link::Empty),
        };
        // 将new_node的所有权转移给self.head
        self.head = Link::More(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        // 1. 首先获取头节点的所有权，同时将 head 置为 Empty
        let mut current_link = std::mem::replace(&mut self.head, Link::Empty);

        // 2. 循环处理每个节点
        while let Link::More(mut boxed_node) = current_link {
            // 3. 获取下一个节点的所有权，同时将当前节点的 next 置为 Empty
            current_link = std::mem::replace(&mut boxed_node.next, Link::Empty);
            // 4. 当前节点在这里被自动释放
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn long_list() {
        let mut list = List::new();
        for i in 0..100000 {
            list.push(i);
        }
        drop(list);
    }
}
