use crate::util;

use std::io::{BufRead, BufReader};
use std::fs::File;
use std::path::Path;
use std::str::FromStr;

use nalgebra as na;

/// This is a simple internal struct that describes the shape of a matrix and its data.
struct MatrixData {
    /// The number of rows in the matrix
    nrows: usize,
    /// The number of columns in the matrix
    ncols: usize,
    /// A flat vector of floats that contains the values of the matrix in row major order
    values: Vec<f64>,
}

/// This is an enum used to parametrize the float format in formatting/output functions.
pub enum FloatFormat {
    /// Format floats in scientific notation, e.g. 1.0e-3
    Scientific,
    /// Format floats in decimal notation, e.g. 0.001
    Decimal,
}

/// This parses a whitespace delimited string of floats into an nalgebra::DVector<f64>
///
/// # Arguments
/// * `vector_string` - the whitespace delimited string of floats that describes the vector
///
pub fn parse_vector(vector_string: &String) -> na::DVector<f64> {
    let vector: Vec<f64> = vector_string.split_whitespace()
        .map(|s| f64::from_str(s).expect("failed to parse a float from vector string"))
        .collect();
    na::DVector::from_vec(vector)
}

/// This parses a whitespace delimited string of floats into an nalgebra::DMatrix<f64>
///
/// # Arguments
/// * `matrix_string` - the whitespace delimited string of floats that describes the matrix
/// * `ncols` - the number of columns in the matrix
/// * `nrows` - the number of rows in the matrix
///
pub fn parse_matrix(matrix_string: &String, nrows: usize, ncols: usize) -> na::DMatrix<f64> {
    let vector: Vec<f64> = matrix_string.split_whitespace()
        .map(|s| f64::from_str(s).expect("failed to parse a float from vector string"))
        .collect();
    // ** from_vec() expects the data presented in column major order
    // ** so, we swap the row and column arguments then transpose
    na::DMatrix::from_vec(
        ncols,
        nrows,
        vector,
    ).transpose()
}

/// This reads a Matrix Market array formatted file and returns a nalgebra::DVector<F64>.
///
/// # Arguments
/// * `path` - The path to the file.
///
pub fn load_vector<R: AsRef<Path>>(path: R) -> Option<na::DVector<f64>> {
    let data = read_matrix_file(path)?;

    Some(na::DVector::from_vec(
        data.values
    ))
}

/// This reads a Matrix Market array formatted file and returns a nalgebra::DMatrix<F64>.
///
/// # Arguments
/// * `path` - the path to the file.
///
pub fn load_matrix<R: AsRef<Path>>(path: R) -> Option<na::DMatrix<f64>> {
    let data = read_matrix_file(path)?;

    // ** from_vec() expects the data presented in column major order
    // ** so, we swap the row and column arguments then transpose
    Some(na::DMatrix::from_vec(
        data.ncols,
        data.nrows,
        data.values,
    ).transpose())
}

/// This reads a Matrix Market array formatted file and returns a MatrixData struct.
fn read_matrix_file<R: AsRef<Path>>(path: R) -> Option<MatrixData> {
    let mat_file = File::open(path).unwrap();
    let mut mat_lines = BufReader::new(mat_file).lines();

    let mut mat_data: Vec<f64> = vec!();

    let mut rows: usize = 0;
    let mut cols: usize = 0;

    while let Some(Ok(line)) = mat_lines.next() {
        if !line.starts_with('%') {
            let split: Vec<&str> = line.split_whitespace().collect();
            rows = usize::from_str(split[0]).expect("failed to parse row count");
            cols = usize::from_str(split[1]).expect("failed to parse column count");
            break;
        }
    }

    let total: usize = rows * cols;

    for line in mat_lines {
        if let Ok(line) = line {
            let split: Vec<&str> = line.split_whitespace().collect();
            for entry in split {
                mat_data.push(f64::from_str(entry).unwrap());
            }
        }
    }

    if mat_data.len() != total {
        return None;
    }

    Some(MatrixData {
        ncols: cols,
        nrows: rows,
        values: mat_data,
    })
}

/// This formats a nalgebra::DVector<f64> as a flat, whitespace delimited string.
///
/// # Arguments
/// * `vector` - the vector to be formatted
/// * `float_format` - how to format the floats: scientific or decimal
/// * `float_precision` - how many positions the floats have past the decimal point
///
pub fn format_vector_flat(
    vector: &na::DVector<f64>,
    float_format: FloatFormat,
    float_precision: usize,
) -> String {
    let mut result_string = String::new();
    for (i, row) in vector.row_iter().enumerate() {
        let val: f64 = row[0];
        match float_format {
            FloatFormat::Decimal => {
                result_string.push_str(&format!("{val:.float_precision$}"))
            }
            FloatFormat::Scientific => {
                result_string.push_str(&format!("{val:.float_precision$e}"));
            }
        }
        if i < vector.nrows() - 1 {
            result_string.push_str(" ");
        }
    }

    result_string
}

/// This formats a nalgebra::DVector<f64> as a String in the Matrix Market array format.
///
/// # Arguments
/// * `vector` - the vector to be formatted
/// * `float_format` - how to format the floats: scientific or decimal
/// * `float_precision` - how many positions the floats have past the decimal point
/// * `header` - the header text at the beginning of the string
///
pub fn format_vector_mm_array(
    vector: &na::DVector<f64>,
    float_format: FloatFormat,
    float_precision: usize,
    header: &str,
) -> String {
    let mut result_string = String::new();
    let n_rows = vector.nrows();
    result_string.push_str(&format!("% {}\n", header));
    result_string.push_str(&format!("{} 1 {}\n", n_rows, n_rows));
    for (i, row) in vector.row_iter().enumerate() {
        let val: f64 = row[0];
        match float_format {
            FloatFormat::Decimal => {
                result_string.push_str(&format!("  {val:.float_precision$}"))
            }
            FloatFormat::Scientific => {
                result_string.push_str(&format!("  {val:.float_precision$e}"));
            }
        }
        if i < vector.nrows() - 1 {
            result_string.push_str("\n");
        }
    }

    result_string
}

// This formats the results for ground truth comparison
// TODO: needs some reworking
pub fn format_comparison_results(
    reactions_computed: &na::DVector<f64>,
    reactions_true: &na::DVector<f64>,
    float_format: FloatFormat,
    float_precision: usize,
    epsilon: f64,
) -> String {
    let mut result_string = String::new();
    // let reactions_delta = reactions_computed - reactions_true;

    result_string.push_str("% computed \t true \t |delta| \t |delta|<=epsilon\n");

    let n_rows = reactions_computed.nrows();
    for i in 0..n_rows {
        let val_computed = reactions_computed.get(i)
            .expect(&format!("failed to retrieve computed value at index: {i}"));
        let val_true = reactions_true.get(i)
            .expect(&format!("failed to retrieve true value at index: {i}"));
        let val_delta = (val_computed - val_true).abs();

        match float_format {
            FloatFormat::Decimal => {
                result_string.push_str(&format!(
                    "  {:.float_precision$}\t{:.float_precision$}\t{:.float_precision$}\t{}",
                    val_computed, val_true, val_delta, util::epsilon_eq(*val_true, *val_computed, epsilon)
                ))
            }
            FloatFormat::Scientific => {
                result_string.push_str(&format!(
                    "  {:.float_precision$e}\t{:.float_precision$e}\t{:.float_precision$e}\t{}",
                    val_computed, val_true, val_delta, util::epsilon_eq(*val_true, *val_computed, epsilon)
                ));
            }
        }
        if i < n_rows - 1 {
            result_string.push_str("\n");
        }
    }
    result_string
}

#[cfg(test)]
mod tests {
    use crate::*;
    use nalgebra as na;

    #[test]
    fn test_parse_vector() {
        let vec_string = String::from("\
            5.4416e-07 13086 13186 13798 1.0884e-06 1.423e-05 1.4392e-05 1.8305e-06 2.1326e-05 \
            2420.6 7014.5 1.8217e+05 1.7251e+05 2.5834e-06 2.5908e-06 5.3759e-07 4.4686e-15 \
            1.8068e-08 4.4686e-15 4.4686e-15 2.7206e-07 2.7213e-07 7.9655e-10 3.039e+05 \
            3.0447e+05 3.6711e+05 8901.3 2.7438e+05\
        ");
        let vec: na::DVector<f64> = io::parse_vector(&vec_string);

        let vec_static: na::SVector<f64, 28> = na::SVector::from_row_slice(matrices::R_STD_015);
        assert!(vec == vec_static)
    }
    #[test]
    fn test_parse_matrix() {
        let matrix_string = String::from("\
            0	2	-2	0	0	0	0	0	0	0	0	0	0	0	0	1	0	0	0	0	0	0	0	0	0	0	0	0 \
            0	0	2	-2	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0 \
            0	0	0	0	0	0	0	0	0	0	0	0	0	0	1	-3	0	0	0	0	0	0	0	0	0	0	0	0 \
            0	0	0	0	0	2	-2	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0 \
            2	0	-2	0	-2	0	0	0	0	0	0	0	0	1	0	2	0	0	0	0	-1	1	0	0	0	0	0	0 \
            0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	1 \
            0	0	0	0	0	0	0	2	-2	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	1	0 \
            0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	-1	-1	0	-1 \
            -2	0	2	0	2	0	0	0	0	0	0	0	0	-1	0	-2	0	0	0	0	1	-1	0	0	0	0	0	0 \
            0	0	0	0	0	0	2	-2	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0 \
            0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	2	-4	0	0	0	0	0	0	0	0 \
            0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	-2	4	0	0	0	0	0	0	0	0 \
            0	0	0	0	0	0	0	0	0	0	-2	0	0	0	0	0	0	1	0	0	0	0	0	0	0	0	0	0 \
            0	0	0	0	0	0	0	0	0	0	2	0	0	0	0	0	0	-1	0	0	0	0	0	0	0	0	0	0 \
            1	-1	0	0	0	0	0	0	0	0	0	0	0	0	0	2	0	0	0	0	0	0	0	0	0	0	0	0 \
            0	0	0	0	0	0	0	0	0	0	2	-2	0	0	0	0	0	0	0	0	0	0	0	0	0	1	0	0 \
            0	0	0	0	0	0	0	0	0	-2	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0 \
            -1	0	0	0	0	0	0	0	0	0	0	0	0	-1	0	0	0	0	0	0	0	0	0	0	0	0	0	0 \
            0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	-1	0 \
            0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	1	0 \
            0	0	0	0	0	0	0	0	0	2	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0 \
            0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	4	0	4	4	-3	0	0	0	0	0	0	0 \
            0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	-5	0	-4	-4	3	0	0	0	0	0	0	0 \
            0	0	0	0	0	-2	2	0	-2	2	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0 \
            0	0	0	0	0	0	0	0	0	0	0	2	-2	0	0	0	0	0	0	0	0	0	1	0	0	0	0	0 \
            0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	1	0	0	0	0 \
            0	-2	0	0	0	-2	0	-2	-2	0	0	0	-2	0	0	-1	1	0	0	0	0	0	0	1	0	0	0	0 \
            0	2	0	0	0	2	0	2	2	0	0	0	2	0	0	1	-1	0	0	0	0	0	0	-1	0	0	0	0 \
            0	0	0	0	0	0	0	0	0	0	0	0	0	-1	0	-3	0	0	0	0	0	0	1	0	0	0	-1	0 \
            0	0	0	0	0	0	0	0	0	0	0	0	0	1	0	3	0	0	0	0	0	0	-1	0	0	0	1	0 \
            0	0	0	0	0	0	-2	0	0	0	0	0	2	0	0	0	0	0	0	0	0	1	0	0	1	0	0	0 \
            0	0	0	0	0	0	0	0	0	0	0	0	0	1	-1	0	0	0	0	0	0	0	0	0	0	0	0	0 \
            0	0	0	2	-2	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0 \
            0	0	0	0	2	-2	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	-1	-1	-1	0	0	0	0 \
            0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	-1	-1	1	0	0	0	0	0	0	0	0	0 \
            0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	1	1	-1	0	0	0	0	0	0	0	0	0 \
            0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0 \
            0	0	0	0	0	0	0	0	0	2	-2	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0 \
            0	0	0	0	0	0	0	0	2	-2	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0	0
        ");
        let matrix: na::DMatrix<f64> = io::parse_matrix(&matrix_string, 39, 28);

        let matrix_static: na::SMatrix<f64, 39, 28> = na::SMatrix::from_row_slice(matrices::S_MAT);

        assert!(matrix == matrix_static)
    }

    #[test]
    fn test_load_vector() {
        let vec_path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "resources/rstd015.txt");
        let vec: na::DVector<f64> = io::load_vector(vec_path).unwrap();

        let vec_static: na::SVector<f64, 28> = na::SVector::from_row_slice(matrices::R_STD_015);
        assert!(vec == vec_static)
    }

    #[test]
    fn test_load_matrix() {
        let smat_path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "resources/smat.txt");
        let smat: na::DMatrix<f64> = io::load_matrix(smat_path).unwrap();

        let smat_static: na::SMatrix<f64, 39, 28> = na::SMatrix::from_row_slice(matrices::S_MAT);

        assert!(smat == smat_static)
    }
}
