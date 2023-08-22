fn test_unreachable_evens(x: usize) -> bool {
    if (x % 2) == 1 {
        unreachable!("this can't happen")
    }
    true
}

fn test_unwrap(x: usize) -> bool {
    if (x % 2) == 1 {
        None.unwrap()
    }
    true
}

fn test_todos(flag: bool) -> bool {
    if flag {
        todo!("this is not implemented")
    }
    true
}

fn main() {
    test_unreachable_evens(2);
    test_todos(false);
    test_unwrap(2);

    //todo case
    //test_todos(true)

    //unwrap explosion
    //test_unwrap(3);

    //unreachable case
    //test_unreachable_evens(3);
}
