use wasm_bindgen::prelude::*;

use crate::solve;
use crate::io;
use crate::util;


#[wasm_bindgen]
pub fn set_panic_hook() {
    // this enables useful error messages in the javascript console
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
/// The default solve function exposed in the wasm API. This uses the default stoichiometric matrix.
///
/// # Arguments
/// * `vector_string` - The accumulation vector, B; (m x n); formatted as a whitespace delimted string
///
pub fn solve_default(vector_string: String) -> String {
    let acc_vector = io::parse_vector(&vector_string);
    let result = solve::solve(acc_vector, util::default_s_matrix());
    io::format_vector_flat(&result, io::FloatFormat::Decimal, 5)
}