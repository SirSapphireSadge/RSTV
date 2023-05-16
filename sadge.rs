extern crate csv;
extern crate serde;

#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "Date")]
    date: String,
    #[serde(rename = "Value")]
    value: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sum = 0.0;
    let mut count = 0;

    let file = std::fs::File::open("data.csv")?;
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.deserialize::<Record>() {
        let record = result?;
        sum += record.value;
        count += 1;
    }

    let average = sum / count as f64;
    println!("Average value: {}", average);

    Ok(())
}
