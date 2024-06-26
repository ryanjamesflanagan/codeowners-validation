use codeowners_validation::parser::parse_codeowners_file;
use codeowners_validation::validators::validator::{run_validator, ValidatorArgs};
use std::env;
use std::io;

fn main() -> io::Result<()> {
    let validator_args = match env::var("INPUT_CHECKS") {
        Ok(args_str) => ValidatorArgs::from_env(&args_str),
        Err(_) => ValidatorArgs::default(),
    };

    let codeowners_file_path = ".github/CODEOWNERS";

    // Parse the CODEOWNERS file
    let (rules, invalid_lines) = match parse_codeowners_file(codeowners_file_path) {
        Ok((rules, invalid_lines)) => (rules, invalid_lines),
        Err(e) => {
            eprintln!("Error parsing CODEOWNERS file: {}", e);
            return Err(e);
        }
    };

    // Check for invalid lines in the CODEOWNERS file
    if !invalid_lines.is_empty() {
        eprintln!("Invalid lines found in the CODEOWNERS file:");
        for invalid_line in invalid_lines {
            eprintln!(
                "Line {}: {}",
                invalid_line.line_number, invalid_line.content
            );
        }
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid lines found in the CODEOWNERS file",
        ));
    }

    let failed_rules = run_validator(&validator_args, &rules);

    if !failed_rules.is_empty() {
        eprintln!("The following rules failed validation:");
        eprintln!();
        for (validator, rule) in &failed_rules {
            eprintln!("Validator: {}", validator);
            eprintln!("  Pattern: {}", rule.pattern);
            eprintln!("    Rule: {}", rule.original_path);
            eprintln!("    Owners: {:?}", rule.owners);
            eprintln!();
        }
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Some rules failed validation",
        ));
    }

    Ok(())
}
