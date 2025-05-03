use std::ptr;

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

type Link<T> = *mut Node<T>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {
        unsafe {
            // into_raw 消费掉 Box (拿走所有权)，返回一个裸指针。该指针会被正确的对齐且不为 null
            // 在调用该函数后，调用者需要对之前被 Box 所管理的内存负责，特别地，调用者需要正确的清理 T 并释放相应的内存。
            // 最简单的方式是通过 Box::from_raw 函数将裸指针再转回到 Box，然后 Box 的析构器就可以自动执行清理了。
            let new_tail = Box::into_raw(Box::new(Node { elem, next: ptr::null_mut() }));
            let raw_tail: *mut _ = &mut *new_tail;
            if !self.tail.is_null() {
                (*self.tail).next = raw_tail;
            } else {
                self.head = raw_tail;
            }
            self.tail = raw_tail;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                None
            } else {
                let head = Box::from_raw(self.head);
                self.head = head.next;
                if self.head.is_null() {
                    self.tail = ptr::null_mut();
                }
                Some(head.elem)
            }
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
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
