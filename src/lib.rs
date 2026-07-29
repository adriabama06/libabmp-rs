pub mod abitmap;
pub mod get;
pub mod create;
pub mod file_read;
pub mod file_write;
pub mod read;
pub mod write;

pub fn add(left: u64, right: u64) -> u64 {
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
