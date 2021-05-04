use cs_to_twisty::Solve;

use std::env;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let filename = env::args()
        .nth(1)
        .unwrap_or_else(|| error_exit("Please provide a file name.", 1));

    let contents = std::fs::read_to_string(filename)
        .unwrap_or_else(|e| error_exit(&format!("Error reading file: {}", e), 2));

    let data = json::parse(&contents)
        .unwrap_or_else(|e| error_exit(&format!("Error parsing json: {}", e), 3));

    let mut outfile = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("output.txt")
        .unwrap_or_else(|e| error_exit(&format!("Error opening output file: {}", e), 4));

    let mut solves_left = data["session1"].members().len();
    eprintln!("Writing {} solves.", solves_left);

    for solve in data["session1"].members() {
        let solve = Solve::parse(solve).unwrap();
        writeln!(&mut outfile, "{}", solve.to_twisty_string())
            .unwrap_or_else(|e| error_exit(&format!("Error writing to output file: {}", e), 5));

        solves_left -= 1;
        eprint!("\rSolves left: {} ", solves_left)
    }
    eprintln!("\nDone!");
}

fn error_exit(msg: &str, code: i32) -> ! {
    eprintln!("{}", msg);
    std::process::exit(code);
}
