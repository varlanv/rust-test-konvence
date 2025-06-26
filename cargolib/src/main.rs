fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    /// should qwe
    #[test]
    fn one_result_case_insensitive() {
        let a = "\
        "
        .to_string();

        assert_eq!(a, "");
    }
}
