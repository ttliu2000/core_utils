pub fn pair_elements<T: Clone>(vec: Vec<T>) -> Vec<(T, T)> {
    let mut pairs = Vec::new();

    let mut iter = vec.into_iter();

    while let (Some(first), Some(second)) = (iter.next(), iter.next()) {
        pairs.push((first, second));
    }

    pairs
}
