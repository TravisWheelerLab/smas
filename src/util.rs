use nalgebra as na;


/// Compare two floats (f64) to see if they are close enough to zero for practical purposes.
///
/// It takes the difference of the two floats, `a` and `b`, and returns true if the
/// absolute value of that difference is smaller than `epsilon`.
///
/// # Arguments
///
/// - `a` - the first float to compare
/// - `b` - the second float to compare
/// - `epsilon` - the value epsilon that the difference between `a` and `b` is compared to.
///
pub fn epsilon_eq(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
}

/// This returns the default stoichiometric matrix as a nalgebra::DMatrix<f64>.
pub fn default_s_matrix() -> na::DMatrix<f64> {
    na::DMatrix::from_row_slice(39, 28, crate::matrices::S_MAT)
}

/// This prints out the data in a nalgebra::Matrix<f64>
pub fn print_matrix<R, C, S>(matrix: &na::Matrix<f64, R, C, S>)
    where
        R: na::Dim,
        C: na::Dim,
        S: na::RawStorage<f64, R, C>
{
    for row in matrix.row_iter() {
        for val in row.iter() {
            print!("{:?}\t", val);
        }
        println!();
    }
}

