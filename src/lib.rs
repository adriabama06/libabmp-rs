pub mod abitmap;
pub mod get;
pub mod create;
pub mod file_read;
pub mod file_write;
pub mod read;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn hello_world() -> &'static str {
    "Hello, World!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn hello_world_test() {
        assert_eq!(hello_world(), "Hello, World!");
    }
}
