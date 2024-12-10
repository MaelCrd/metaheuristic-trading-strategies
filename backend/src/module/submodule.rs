pub fn function1() {
    println!("Function 1 in submodule1");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function1() {
        function1();
        assert_eq!(1 + 1, 2);
    }
}
