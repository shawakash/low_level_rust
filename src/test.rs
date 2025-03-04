#[cfg(test)]
mod test {
    mod cell {
        use crate::cell::Cell;

        #[test]
        fn test_cell() {
            let cell = Cell::new('a');

            println!("{}", cell.get());
            cell.set('c');
            println!("{}", cell.get());
        }
    }
}
