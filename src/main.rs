use std::{
    borrow::BorrowMut,
    cell::{Cell, RefCell},
    sync::{Arc, RwLock},
    thread,
};

#[derive(Debug)]
struct Node<'a> {
    val: Cell<i32>,
    adjacent: Vec<&'a Node<'a>>,
}

#[derive(Debug)]
struct NodeStr<'a> {
    val: RefCell<String>,
    adjacent: Vec<&'a NodeStr<'a>>,
}

#[derive(Debug)]
struct NodeStrLock {
    val: RwLock<String>,
    adjacent: Vec<Arc<NodeStrLock>>,
}

fn add_one(node: &Node) {
    let curr_val = node.val.get();
    node.val.set(curr_val + 1);
    for adj in node.adjacent.iter() {
        add_one(&adj);
    }
}

fn add_one_str(node: &NodeStr) {
    let mut curr_val = node.val.borrow_mut();
    curr_val.push('!');
    for adj in node.adjacent.iter() {
        add_one_str(&adj);
    }
}

fn add_one_str_lock(node: &NodeStrLock) {
    {
        let mut curr_val = node.val.write().unwrap();
        curr_val.push('!');
    }
    for adj in node.adjacent.iter() {
        add_one_str_lock(&adj);
    }
}

fn run_num() {
    let a = Node {
        val: Cell::new(1),
        adjacent: vec![],
    };

    let b = Node {
        val: Cell::new(2),
        adjacent: vec![&a],
    };

    let c = Node {
        val: Cell::new(3),
        adjacent: vec![&a],
    };

    add_one(&c);

    dbg!(&a);
    dbg!(&b);
    dbg!(&c);
}

fn run_str() {
    let a = NodeStr {
        val: RefCell::new("aa".to_owned()),
        adjacent: vec![],
    };

    let b = NodeStr {
        val: RefCell::new("bbb".to_owned()),
        adjacent: vec![&a],
    };

    let c = NodeStr {
        val: RefCell::new("cccc".to_owned()),
        adjacent: vec![&a],
    };

    add_one_str(&c);

    dbg!(&a);
    dbg!(&b);
    dbg!(&c);
}

fn run_str_lock() {
    let a = Arc::new(NodeStrLock {
        val: RwLock::new("aa".to_owned()),
        adjacent: vec![],
    });

    let b = Arc::new(NodeStrLock {
        val: RwLock::new("bbb".to_owned()),
        adjacent: vec![a.clone()],
    });

    let c = Arc::new(NodeStrLock {
        val: RwLock::new("cccc".to_owned()),
        adjacent: vec![a.clone()],
    });

    let t1_b = b.clone();
    let t1 = thread::spawn(move || {
        add_one_str_lock(&t1_b);
    });

    let t2_c = c.clone();
    let t2 = thread::spawn(move || {
        add_one_str_lock(&t2_c);
    });

    t1.join();
    t2.join();

    dbg!(&*a);
    dbg!(&*b);
}

fn main() {
    run_str_lock();
}
