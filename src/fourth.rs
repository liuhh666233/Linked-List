use std::ptr;

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {
        let mut new_tail = Box::new(Node { elem, next: None });
        // 为new tail 创建raw pointer,
        // *new_tail, 解引用new_tail, 获取到Node<T>,
        // &mut *new_tail, 获取到Node<T>的引用, 并可变,
        // *mut _ 表示raw pointer, _ 表示让编译器推断类型
        let raw_tail: *mut _ = &mut *new_tail;
        // is_null 检查raw pointer是否为null
        if !self.tail.is_null() {
            // 对于操作raw pointer, 需要使用unsafe
            unsafe {
                // 注意运算符优先级, 需要加括号
                (*self.tail).next = Some(new_tail);
            }
        } else {
            self.head = Some(new_tail);
        }
        self.tail = raw_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            self.head = head.next;
            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }
            head.elem
        })
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
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // Check the exhaustion case fixed the pointer right
        list.push(6);
        list.push(7);

        // Check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }
}
