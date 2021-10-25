use structopt::StructOpt;

// Seach =for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct CLI {
    // The pattern to search for
    pattern: String,
    // The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = CLI::from_args();
    // Parse the command line arguments
    let pattern = std::env::args().nth(1).expect("No pattern given");
    // The path to the file to read
    let path = std::env::args().nth(2).expect("No path given");
    // Print the pattern and path
    println!("Hello, world!: {}", pattern);
    println!("Hello, world!: {}", path);
}
