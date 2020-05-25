use std::fs::{OpenOptions, File, remove_file};
use std::io::Result;
use crate::measurement::Measurement;

const QUEUE_FILE: &'static str = "queue.json";

pub fn get() -> Result<Vec<Measurement>>{
    let file = OpenOptions::new().create(false).read(true).open(QUEUE_FILE);
    if file.is_ok() {
        let queue: Vec<Measurement> = serde_json::from_reader(&file.unwrap())?;
        Ok(queue)
    } else {
        Ok(vec![])
    }
}

pub fn write(measurements: &Vec<Measurement>) -> Result<()> {
    let file = File::create(QUEUE_FILE)?;
    serde_json::to_writer(&file, &measurements)?;
    Ok(())
}

pub fn clear() -> Result<()> {
    remove_file(QUEUE_FILE)?;
    Ok(())
}
