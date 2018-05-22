fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    #[test]
    fn empty_strands() {
        assert_eq!(2, 3);
    }
}
