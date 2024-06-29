use std::error::Error;
use std::fs::File;

use crate::rct_entry::RCTEntry;

// Try to read the CSV file with both delimiters, ';' and ','
pub fn read_csv(file: &File) -> Result<Vec<RCTEntry>, Box<dyn Error>> {
  if let Ok(entries) = try_read_csv(file, b';') {
    return Ok(entries);
  }
  let entries = try_read_csv(file, b',')?;
  Ok(entries)
}

fn try_read_csv(file: &File, delimiter: u8) -> Result<Vec<RCTEntry>, Box<dyn Error>> {
  let mut reader = csv::ReaderBuilder::new()
    .has_headers(true)
    .delimiter(delimiter)
    .from_reader(file);

  let mut entries: Vec<RCTEntry> = Vec::new();

  for result in reader.records() {
    let record = result?;
    let entry = RCTEntry::from_record(record)?;
    entries.push(entry);
  }

  Ok(entries)
}
