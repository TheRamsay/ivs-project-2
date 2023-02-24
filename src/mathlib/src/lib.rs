/// more documentation here
///
/// you can even write examples here, which can contain
/// asserts, which can be run as tests, so called `doctests`
///
/// afaik you can use *Markdown* __notation__ here
pub fn add(left: usize, right: usize) -> usize {

    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
