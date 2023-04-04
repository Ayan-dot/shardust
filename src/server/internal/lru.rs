use std::{
    cell::{RefCell, RefMut},
     sync::atomic::AtomicBool,
     sync::Mutex
};
use dashmap::DashMap;
use once_cell::sync::OnceCell;

struct StatefulLRULock<K,V> {
    hashmap: DashMap<K, Box<Node<K,V>>>,
    capacity: u64, 
    list_head: Box<Node<K,V>>,
    list_tail: Box<Node<K,V>>,
    size: u64,
}

pub struct LRU<K,V> {
    lock: Mutex<StatefulLRULock<K,V>>
}

struct NState<K,V> {
    active: u32, 
    key: Option<K>,
    next: Box<Option<Node<K,V>>>,
    prev: Box<Option<Node<K,V>>>
}

struct Node<K,V> {

    value: OnceCell<V>,
    initialized: AtomicBool,
    state: RefCell<NState<K,V>>
}

impl<K,V> Node<K,V> {
    // unsure as to whether refmut will suffice, otherwise unsafe rust must be brought in (sad -n-)
    fn state(&self, lock: &mut StatefulLRULock<K,V>) -> RefMut<NState<K,V>> {
        self.state.borrow_mut()
    }
}

#[cfg(test)]
mod tests {
    use core::panic;

    use super::*;

    #[test]
    fn simple_node_test() {
        let node = Node::<u32,u32> {
            value: OnceCell::new(),
            initialized: AtomicBool::new(false),
            state: RefCell::new(NState { active: 1, key: Some(1), next: Box::new(None), prev: Box::new(None) })
        };
        if let 4 = node.value.get_or_init(|| 4) {
            println!("functions")
        }
        else {
            panic!("something is wrong")
        }
    }
}