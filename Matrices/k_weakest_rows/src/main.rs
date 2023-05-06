fn main() {
    let i = k_weakest_rows(
        vec![
            vec![1,1,0,0,0],
            vec![1,1,1,1,0],
            vec![1,0,0,0,0],
            vec![1,1,0,0,0],
            vec![1,1,1,1,1]]
        ,
        3
    );
}

// k_weakest_rows by uptu
pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    let mut indices: Vec<usize> =   // Create a vector `indices`
        // Populate it with the values between 0 and the length in the matrix.
        // For instance, if `mat` has 5 rows, indices has values [0, 1, 2, 3, 4].
        (0..mat.len())
            .collect(); // Which are then collected into a vector.

    // Now sort the `indices` by the sum of the contents in `mat`.
    // This uses a closure on the matrix index `i` to sum all of the contents of each row into an i32, which is then used to sort the vector.
    indices.sort_by_key(|i| mat[*i].iter().sum::<i32>());

    // In order to get the weakest `k` values in `indices` without using a for loop, we can use the following `take()` and `map()` functions, which take ownership of the first `k` elements in `indices`, then parse them into `i32`s (they're currently `usize`).
    let weakest_k: Vec<i32> = indices.iter()
        .take(k as usize)   // Take ownership of the first `k` elements in `indices`
        .map(|&i| i as i32) // Parse them into `i32`s (they're currently `usize`)
        .collect();         // Then collect into a vector to return.
    println!("{:?}", weakest_k);
    weakest_k
}