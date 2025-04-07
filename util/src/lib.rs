use std::collections::HashMap;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy)]
pub struct MarkSpan {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy)]
pub struct Mark {
    pub a: MarkSpan,
    pub b: MarkSpan,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Pair {
    pub a: String,
    pub b: String,
    pub marks: Vec<Mark>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Dataset {
    /// The `usize` here is the pair index, a unique value for each pair of items in the dataset
    /// The `Pair` is specific information from the index
    /// I didn't use a `Vec` here because it would be very sparse
    pub pairs: HashMap<usize, Pair>,
}

#[macro_export]
macro_rules! str_error {
    ($t: ty) => {
        impl serde::Serialize for $t {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::ser::Serializer,
            {
                serializer.serialize_str(self.to_string().as_ref())
            }
        }
    };
}

#[derive(Debug, thiserror::Error)]
pub enum DatasetError {
    #[error("No dataset loaded")]
    NoDataset,
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Tree(#[from] ast::TreeParseError),
    #[error("Dataset is empty or mixes languages")]
    InvalidComposition,
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}

str_error!(DatasetError);

pub fn find_paired_indices_from_pair_index(mut k: usize, n: usize) -> (usize, usize) {
    let mut remaining = n - 1;
    let mut i = 0;
    while k >= remaining {
        i += 1;
        k -= remaining;
        remaining -= 1;
    }

    (i, i + k + 1)
}
// Oops this doesn't work, it overflows the stack
// TCO results in an infinite hang using `tailcall`, and `become` isn't in Rust yet
// // YOU CAN DO BETTER! THANKS JOHN FOR O(log(n))
// // Special binary search implementation that tries to guess values of k at certain i values,
// // homing in on correct one
// fn binary_search_index(k: usize, n: usize, l: usize, r: usize) -> PairedIndices {
//     let x = (r - l) / 2 + l;
//     let val = n * (n - 1) / 2 - x * (x - 1) / 2;
//     if val <= k {
//         if k - val < x - 1 {
//             return PairedIndices { i: n - x, j: val };
//         }
//         return binary_search_index(k, n, l, x + 1);
//     }
//     binary_search_index(k, n, x + 1, r)
// }

// fn find_paired_indices_from_pair_index(k: usize, n: usize) -> PairedIndices {
//     let PairedIndices { i, j } = binary_search_index(k, n, 0, n);
//     PairedIndices {
//         i,
//         j: k - j + i + 1,
//     }
// }

#[macro_export]
macro_rules! arr_vec_to_view {
    ($a:expr) => {
        ::util::view(&$a).as_slice()
    };
}

pub fn view<T, D: ndarray::Dimension>(
    arr: &[ndarray::ArrayBase<ndarray::OwnedRepr<T>, D>],
) -> Vec<ndarray::ArrayView<T, D>> {
    arr.iter().map(|a| a.view()).collect()
}
