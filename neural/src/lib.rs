#![feature(new_range_api)]

pub mod contrastive;
pub mod data;
pub mod elu;
pub mod gat;
pub mod loss;
pub mod model;
pub mod node_process;

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
