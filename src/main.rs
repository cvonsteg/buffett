use structopt::StructOpt;
use std::io::Write;
use std::fs::File;

use buffett::parse_utils::file_to_stocks;

#[derive(StructOpt)]
struct Cli {
    // Helper struct for parsing CLI arguments
    input_file: String,
    output_file: String
}


fn main() {
    let args = Cli::from_args();
    let stocks = file_to_stocks(&args.input_file);
    let mut file = File::create(&args.output_file).expect("Failed to create output file");
    file.write_all("stock, open_close_delta, open_close_delta_perc, max_price_delta".as_bytes()).expect("Write failed");
    for stk in stocks {
        file.write_all("\n".as_bytes()).expect("Write error");
        file.write_all(stk.output_stats().as_bytes()).expect("Write failed");
    }
}
