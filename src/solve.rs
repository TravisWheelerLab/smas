use nalgebra as na;
pub const SVD_EPSILON: f64 = 1e-9;

/// This function solves the linear equation Ax = B, where A is a stoichiometric matrix and B is an
/// accumulation vector. The return value is the solution vector x.
///
/// The solution is computed using the Moore-Penrose inverse (i.e. pseudoinverse) of A.
///
/// # Arguments
/// * `acc_vector` - the accumulation vector, B; (m x n)
/// * `s_matrix` - the stoichiometric matrix, A: (m x 1)
///
pub fn solve(acc_vector: na::DVector<f64>, s_matrix: na::DMatrix<f64>) -> na::DVector<f64> {
    let s_pseudo_inverse = s_matrix.pseudo_inverse(SVD_EPSILON)
        .expect("failed to compute pseudo-inverse of stoichiometric matrix");

    s_pseudo_inverse * acc_vector
}

#[cfg(test)]
mod tests {
    use crate::*;
    use nalgebra as na;

    #[test]
    fn test_solve() {
        let s_matrix = util::default_s_matrix();
        let acc_vector: na::DVector<f64> = na::DVector::from_row_slice(matrices::A_STD_015);
        let r_vector_truth: na::DVector<f64> = na::DVector::from_row_slice(matrices::R_STD_015);
        let r_vector = solve::solve(acc_vector, s_matrix);
        for (c, t) in r_vector.row_iter().zip(r_vector_truth.row_iter()) {
            assert!(util::epsilon_eq(c[0], t[0], 1e-4));
        }
    }
}
