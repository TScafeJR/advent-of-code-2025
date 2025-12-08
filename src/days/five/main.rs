use crate::days::Day;
use crate::util::linked_list::LinkedList;
use crate::util::nodes::LinkedListNode;
use crate::util::range;

pub fn insert_range(ll: &mut LinkedList<range::Range>, r: range::Range) {
    insert_recursive(&mut ll.head, r);
}

fn insert_recursive(link: &mut Option<Box<LinkedListNode<range::Range>>>, r: range::Range) {
    if link.is_none() {
        *link = Some(Box::new(LinkedListNode::new(r)));
        return;
    }

    let node = link.as_mut().unwrap();

    if r.high < node.value.low {
        let mut new_node = LinkedListNode::new(r);
        new_node.next = link.take();
        *link = Some(Box::new(new_node));
        return;
    }

    if r.low > node.value.high {
        insert_recursive(&mut node.next, r);
        return;
    }

    node.value.low = node.value.low.min(r.low);
    node.value.high = node.value.high.max(r.high);

    while let Some(ref next_node) = node.next {
        if node.value.high >= next_node.value.low - 1 {
            node.value.high = node.value.high.max(next_node.value.high);

            node.next = node.next.as_mut().unwrap().next.take();
        } else {
            break;
        }
    }
}

fn part1(data: Vec<String>) -> u64 {
    let mut fresh = 0;
    let mut ranges: Vec<range::Range> = Vec::new();
    let mut curr = 0;

    for i in 0..data.len() {
        curr = i;

        if data[i].is_empty() {
            break;
        }

        let r = range::Range::new(&data[i]);
        ranges.push(r);
    }

    curr += 1;

    for i in curr..data.len() {
        let j: u64 = data[i].parse().unwrap();
        for k in 0..ranges.len() {
            let r = &ranges[k];
            if j >= r.low && j <= r.high {
                fresh += 1;
                break;
            }
        }
    }

    fresh
}

fn part2(data: Vec<String>) -> u64 {
    let mut count = 0;
    let mut ll = LinkedList::<range::Range>::new();

    for i in 0..data.len() {
        if data[i].is_empty() {
            break;
        }

        let r = range::Range::new(&data[i]);
        insert_range(&mut ll, r);
    }

    let mut curr = ll.head.as_ref();
    while curr.is_some() {
        let r = &curr.unwrap().value;
        count += r.high - r.low + 1;
        curr = curr.unwrap().next.as_ref();
    }

    count
}

pub fn functions() -> Day {
    Day {
        part1: Some(part1),
        part2: Some(part2),
    }
}
