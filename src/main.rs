use clap::{arg, App, value_parser, Command, AppSettings};
use smas;

fn add_common_args(app: App) -> App {
    app.arg(
        arg!(-s <matrix_path> "The path to a stoichiometric matrix file in the Matrix Market array format.")
            .required(false)
    )
        .arg(
            arg!(-o <out_path> "The path to the output (printed to stdout by default).")
                .required(false)
        )
        .arg(
            arg!(-e <epsilon> "Values below epsilon are considered equal to 0.")
                .required(false)
                .default_value("1e-3")
                .value_parser(value_parser!(f64))
        )
        .arg(
            arg!(-p <float_precision> "The precision of floating point numbers in the output")
                .required(false)
                .default_value("5")
                .value_parser(value_parser!(u8))
        )
        .arg(
            arg!(-f <float_format>)
                .help("Adjust the formatting of floating point numbers in the output.")
                .required(false)
                .default_value("scientific")
                .value_parser(["scientific", "decimal"])
        )
}

fn main() {
    let mut solve_command = Command::new("solve")
        .about("Solve for a reaction vector given an accumulation vector")
        .arg(
            arg!(<accumulation_path> "The path to a stoichiometric accumulation vector file in the Matrix Market array format.")
                .required(false)
        )
        .arg(
            arg!(-a <accumulation_string> "Optionally, provide the input accumulation vector via stdin. \
                The vector should be enclosed in quotes and whitespace delimited, \
                e.g. \"0.0 1e5 0.5 0.3 0.0 ...\"")
                .required(false)
        );

    let mut validate_command = Command::new("validate")
        .about("A set of utilities designed to help validate computed results using ground truth data")
        .arg(
            arg!(-r <reactions_path> "The path to a stoichiometric reaction vector file in the Matrix Market array format. \
                        If provided, smas will compare the vector to the computed solution.")
                .required(false)
        );

    solve_command = add_common_args(solve_command);
    validate_command = add_common_args(validate_command);

    let matches = App::new("smas")
        .version("0.1.0")
        .author("Jack Roddy <jack.w.roddy@gmail.com>")
        .about("A simple tool to help with finding a solution to a particular stoichiometric matrix equation")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .set_term_width(80)
        .subcommand(solve_command)
        .subcommand(validate_command)
        .get_matches();

    match matches.subcommand_name() {
        Some("solve") => {
            let matches = matches.subcommand_matches("solve").unwrap();
            let accumulation_path = matches.get_one::<String>("accumulation_path");
            let accumulation_string = matches.get_one::<String>("accumulation_string");
            let matrix_path = matches.get_one::<String>("matrix_path");
            let epsilon = *matches.get_one::<f64>("epsilon").unwrap();
            let float_format = if let Some(format) = matches.get_one::<String>("float_format") {
                match format.as_str() {
                    "scientific" => smas::io::FloatFormat::Scientific,
                    "decimal" => smas::io::FloatFormat::Decimal,
                    _ => unreachable!()
                }
            } else {
                smas::io::FloatFormat::Scientific
            };

            let float_precision = *matches.get_one::<u8>("float_precision").unwrap();
            let a_vector = match accumulation_path {
                Some(path) => smas::io::load_vector(path)
                    .expect("failed to load accumulation vector file"),
                None =>
                    match accumulation_string {
                        Some(vector_string) => smas::io::parse_vector(vector_string),
                        None => panic!()
                    }
            };

            let s_matrix = match matrix_path {
                Some(path) => smas::io::load_matrix(path)
                    .expect("failed to load custom stoichiometric matrix file"),
                None => smas::util::default_s_matrix()
            };

            let results_vector = smas::solve::solve(a_vector, s_matrix);
            smas::util::print_matrix(&results_vector);
        }
        Some("validate") => {
            let validate_args = matches.subcommand_matches("validate").unwrap();
        }
        _ => unreachable!()
    }

    // if let Some(reaction_vector_computed) = reaction_vector_computed {
    //     let results: String = match reaction_vector_truth {
    //         Some(reaction_vector_truth) => format_comparison_results(
    //             &reaction_vector_computed,
    //             &reaction_vector_truth,
    //             float_format,
    //             float_precision as usize,
    //         ),
    //         None => format_accumulation_results(
    //             &reaction_vector_computed,
    //             float_format,
    //             float_precision as usize,
    //         )
    //     };
    //     println!("{}", results);
    // } else {
    //     println!("failed to produce a reaction vector");
    // }
}