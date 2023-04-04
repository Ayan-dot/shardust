use std::{
    cell::UnsafeCell,
     sync::atomic::AtomicBool,
};
use dashmap::DashMap;
use once_cell::sync::OnceCell;

struct NState<K,V> {
    req: u32, 
    key: Option<K>,
    next: Box<Node<K,V>>,
    prev: Box<Node<K,V>>
}

struct Node<K,V> {

    value: OnceCell<V>,
    initialized: AtomicBool,
    state: UnsafeCell<NState<K,V>>
}

