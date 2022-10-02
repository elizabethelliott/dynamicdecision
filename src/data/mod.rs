use std::{path::Path, fs::File, io::Write, fs::create_dir_all};

use crate::views::ExperimentData;

pub mod partipant_data;

pub fn write_data_file(id: u32, experiment_data: Box<&dyn ExperimentData>) {
    let file_name = format!("output/{}/{}.csv", id, experiment_data.name());
    let write_path = Path::new(file_name.as_str());

    let parent_folder = write_path.parent().unwrap();

    if !parent_folder.exists() {
        create_dir_all(parent_folder).unwrap();
    }

    let mut file = File::create(write_path).expect("Could not open data file for writing");

    file.write_all(experiment_data.headers().as_bytes()).expect("Could not write CSV header to file");
    file.write("\n".as_bytes()).expect("Could not write to file");
    file.write_all(experiment_data.data().to_csv().as_bytes()).expect("Could not write CSV data");
    file.flush().expect("Could not flush the file to the disk");
}