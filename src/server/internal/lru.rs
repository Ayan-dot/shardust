use dashmap::DashMap;
use once_cell::sync::OnceCell;
use std::{
    cell::{RefCell, RefMut},
    hash::Hash,
    ops::{Deref, DerefMut},
    ptr,
    sync::atomic::AtomicBool,
    sync::Mutex,
};

struct NState<K, V> {
    active: u32,
    key: Option<K>,
    next: NodePtr<K, V>,
    prev: NodePtr<K, V>,
}

struct Node<K, V> {
    value: OnceCell<V>,
    initialized: AtomicBool,
    state: RefCell<NState<K, V>>,
}

impl<K, V> Node<K, V>
where
    K: Hash + Eq + Clone,
    V: Copy + Clone,
{
    // unsure as to whether refmut will suffice, otherwise unsafe rust must be brought in (sad -n-)
    fn state(&self, lock: &mut StatefulLRULock<K, V>) -> RefMut<NState<K, V>> {
        self.state.borrow_mut()
    }
    fn val(&self) -> V {
        return *(self.value.get().unwrap());
    }
}

struct NodePtr<K, V>(*const Node<K, V>);

impl<K, V> Copy for NodePtr<K, V> {}

impl<K, V> Clone for NodePtr<K, V> {
    fn clone(&self) -> Self {
        NodePtr(self.0)
    }
}

impl<K, V> PartialEq for NodePtr<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<K, V> Eq for NodePtr<K, V> {}

impl<K, V> Deref for NodePtr<K, V> {
    type Target = Node<K, V>;

    fn deref(&self) -> &Self::Target {
        assert!(self.0 != ptr::null());
        unsafe { &*self.0 }
    }
}

struct StatefulLRULock<K, V> {
    hashmap: DashMap<K, Box<Node<K, V>>>,
    capacity: u64,
    list_head: NodePtr<K, V>,
    list_tail: NodePtr<K, V>,
    size: u64,
}

pub struct LRU<K, V> {
    lock: Mutex<StatefulLRULock<K, V>>,
}

unsafe impl<K, V> Send for LRU<K, V> {}
unsafe impl<K, V> Sync for LRU<K, V> {}

impl<'a, K, V> LRU<K, V>
where
    K: Hash + Eq + Clone,
    V: Copy + Clone,
{
    fn append(&self, node: NodePtr<K, V>) {
        let mut lock = self.lock.lock().unwrap();
        if let 0 = lock.size {
            lock.list_head = node;
            lock.list_tail = node;
        } else {
            let head = lock.list_head;
            head.state(&mut lock).prev = node;
            node.state(&mut lock).next = head;
            lock.list_head = node;
        }
        lock.size += 1;
    }

    fn get_head(&self) -> Result<V, &'static str> {
        let lock = self.lock.lock().unwrap();
        let not_null = lock.list_head.0.is_null();
        let head = lock.list_head;
        if (!not_null) {
            let val = head.val();
            return Ok(val);
        } else {
            return Err("head doesn't exist");
        }
    }
    pub fn new(capacity: u64) -> Self {
        Self {
            lock: Mutex::new(StatefulLRULock {
                hashmap: DashMap::with_capacity(capacity.try_into().unwrap()),
                capacity: (capacity),
                list_head: (NodePtr(ptr::null())),
                list_tail: (NodePtr(ptr::null())),
                size: (0),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn simple_lru_test() {
        let n_lru = LRU::<u32, u32>::new(10);

        let node = Node::<u32, u32> {
            value: OnceCell::with_value(5),
            initialized: AtomicBool::new(false),
            state: RefCell::new(NState {
                active: 1,
                key: Some(1),
                next: NodePtr(ptr::null()),
                prev: NodePtr(ptr::null()),
            }),
        };
        assert_eq!(n_lru.get_head(), Err("head doesn't exist"));

        n_lru.append(NodePtr(&node));

        assert_eq!(n_lru.get_head(), Ok(5));
    }
    #[test]
    fn simple_node_test() {
        let node = Node::<u32, u32> {
            value: OnceCell::new(),
            initialized: AtomicBool::new(false),
            state: RefCell::new(NState {
                active: 1,
                key: Some(1),
                next: NodePtr(ptr::null()),
                prev: NodePtr(ptr::null()),
            }),
        };
        if let 4 = node.value.get_or_init(|| 4) {
            println!("functions")
        } else {
            panic!("you should never be getting this error lol")
        }
    }
}
