use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::new();
    assert_eq!(heap.peek(), None);
    heap.push(1);
    heap.push(5);
    heap.push(2);

    assert_eq!(heap.peek(), Some(&5));

    assert_eq!(heap.len(), 3);

    for x in &heap {
        println!("{x}");
    }
    heap.clear();

    // The heap should now be empty.
    assert!(heap.is_empty())
}
