
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct RCTEntry {
  pub date: String,
  pub pdc_a: String,
  pub pdc_b: String,
  pub pdc_sum: f64,
  pub pac: String,
  pub pload: String,
  pub pgrid_feed: String,
  pub pgrid_load: String,
  pub pgrid: String,
}

impl RCTEntry {
  pub fn from_record(record: csv::StringRecord) -> Result<Self, csv::Error> {
    let rct_entry: RCTEntry = record.deserialize(None)?;
    Ok(rct_entry)
  }
}
