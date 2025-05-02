struct Node<T> {
    elem: T,
    next: Link<T>,
}

// 使用类型别名, 简化Link定义
type Link<T> = Option<Box<Node<T>>>;
pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Node {
            elem: elem,
            // 将self.head持有数据的所有权转移给new_node.next,
            // 同时将self.head设置为None
            next: self.head.take(),
        };
        // 将new_node的所有权转移给self.head
        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        // map 方法会处理 Some(node) 的情况，返回闭包函数执行结果
        // 如果 self.head 是 None，则 map 返回 None
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // 1. 首先获取头节点的所有权，同时将 head 置为 None
        let mut current_link = self.head.take();

        // 2. 循环处理每个节点
        while let Some(mut boxed_node) = current_link {
            // 3. 获取下一个节点的所有权，同时将当前节点的 next 置为 None
            current_link = boxed_node.next.take();
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

        // Check None list behaves right
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


        // 测试泛型
        let mut list = List::new();
        list.push("hello");
        list.push("world");
        assert_eq!(list.pop(), Some("world"));
        assert_eq!(list.pop(), Some("hello"));
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
