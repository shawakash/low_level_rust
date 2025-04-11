use crate::{
    cell::Cell,
    iter::{flatten, IteratorExt},
};

#[test]
fn test_cell() {
    let cell = Cell::new('a');

    println!("{}", cell.get());
    cell.set('c');
    println!("{}", cell.get());
}

#[test]
fn empty() {
    assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0);
}

#[test]
fn empty_wide() {
    assert_eq!(flatten(vec![Vec::<()>::new(), vec![], vec![]]).count(), 0);
}

#[test]
fn one() {
    assert_eq!(flatten(std::iter::once(vec!["a"])).count(), 1);
}

#[test]
fn two() {
    assert_eq!(flatten(std::iter::once(vec!["a", "b"])).count(), 2);
}

#[test]
fn two_wide() {
    assert_eq!(flatten(vec![vec!["a"], vec!["b"]]).count(), 2);
}

#[test]
fn reverse() {
    assert_eq!(
        flatten(std::iter::once(vec!["a", "b"]))
            .rev()
            .collect::<Vec<_>>(),
        vec!["b", "a"]
    );
}

#[test]
fn reverse_wide() {
    assert_eq!(
        flatten(vec![vec!["a"], vec!["b"]])
            .rev()
            .collect::<Vec<_>>(),
        vec!["b", "a"]
    );
}

#[test]
fn both_ends() {
    let mut iter = flatten(vec![vec!["a1", "a2", "a3"], vec!["b1", "b2", "b3"]]);
    assert_eq!(iter.next(), Some("a1"));
    assert_eq!(iter.next_back(), Some("b3"));
    assert_eq!(iter.next(), Some("a2"));
    assert_eq!(iter.next_back(), Some("b2"));
    assert_eq!(iter.next(), Some("a3"));
    assert_eq!(iter.next_back(), Some("b1"));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);
}

#[test]
fn inf() {
    let mut iter = flatten((0..).map(|i| 0..i));
    // 0 => 0..0 => empty
    // 1 => 0..1 => [0]
    // 2 => 0..2 => [0, 1]
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
}

#[test]
fn deep() {
    assert_eq!(flatten(flatten(vec![vec![vec![0, 1]]])).count(), 2);
}

#[test]
fn ext() {
    assert_eq!(vec![vec![0, 1]].into_iter().our_flatten().count(), 2);
}
