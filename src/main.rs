use std::error::Error;
use std::fs::File;

use clap::Parser;

use crate::cli::Args;
use crate::csv_handler::read_csv;

mod rct_entry;
mod csv_handler;
mod cli;

struct FileEntry {
  name: String,
  file: File,
}

impl FileEntry {
  pub fn new(name: String, file: File) -> Self {
    FileEntry {
      name,
      file,
    }
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  let args = Args::parse();

  println!("Reading file: {}", args.file);

  let mut file_entries: Vec<FileEntry> = vec![];
  let file = File::open(&args.file)?;
  // If we received a single file we add this to the list, otherwise we traverse the given folder one level down
  if file.metadata()?.is_file() {
    file_entries.push(FileEntry::new(args.file, file));
  } else {
    for entry in std::fs::read_dir(&args.file)? {
      let entry = entry?;
      let path = entry.path();
      if path.is_file() {
        file_entries.push(FileEntry::new(entry.file_name().to_string_lossy().to_string(), File::open(path)?));
      }
    }
  }

  file_entries.sort_by_key(|e| e.name.clone());
  
  let mut sum_total_wh = 0.0;
  
  for file_entry in file_entries {
    
    let entries = read_csv(&file_entry.file)?;
    let sum_wh: f64 = entries.iter().map(|entry| entry.pdc_sum / 12.0).sum();

    // for entry in entries {
    //   println!("PDC Sum: {} = {} ; {}", entry.pdcSum, entry.pdcSum, entry.pdcSum / 12.0);
    // }

    sum_total_wh += sum_wh;
    println!("Processing file: {} => {}", file_entry.name, sum_wh);
  }

  let sum_total_wh_str = format!("{:.2}", sum_total_wh);
  
  let sum_total_kwh = sum_total_wh / 1000.0;
  let sum_total_kwh_str = format!("{:.2}", sum_total_kwh);
  
  let sum_total_mwh = sum_total_kwh / 1000.0;
  let sum_total_mwh_str = format!("{:.2}", sum_total_mwh);
  
  println!("\n\n\nSum Total Wh: {}, KWh: {}, MWh: {}", sum_total_wh_str, sum_total_kwh_str, sum_total_mwh_str);

  

  Ok(())
}