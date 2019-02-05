extern crate csv;
#[macro_use]
extern crate serde;

#[derive(Deserialize)]
struct Record {
    year: u16,
    make: String,
    model: String,
    description: String,
}

fn main() {
    let mut reader =
        csv::Reader::from_path("./in.csv").expect("Expected in.csv is working directory");

    reader.deserialize()
          .into_iter()
          .for_each(|x: Result<Record, csv::Error>| match x {
              Ok(rec) => {
                  println!(
                      "In {}, {} built the {} model. It is a {}.",
                      rec.year, rec.make, rec.model, rec.description
                  );
              }
              Err(e) => eprintln!("Error: {}", e),
          });
}
