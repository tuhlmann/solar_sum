# solar_sum

This is a simple personal project to calculate the total Wh produced by a solar panel system of manufacturer RCT. 

RCT can export a log file in CSV format, 
which contains the power produced by the system in 5 minute intervals. 
This project reads the CSV file and calculates the total Wh produced by the system.

## Installation

This is a Rust project, so you need to have Rust installed on your system.
Go to the [Rust website](https://www.rust-lang.org/tools/install) and follow the instructions to install Rust.

To run it, type:

```bash
cargo run -- -f <file>
```

Where `<file>` is the path to the CSV file or a directory containing the CSV files.
If you give it a directory, it will read all the CSV files in the directory and calculate the total sum for all the individual files.
Please note that the application will not traverse subdirectories.

## Important

At the moment, the application expects the column so sum up at position 4 (the 4th column) in the CSV file.


The sum is printed to the console in Wh, KWh and MWh.

If there is demand, I'll release binary versions for this application, please let me know if you need it.