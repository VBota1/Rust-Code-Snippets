use std::fs::OpenOptions;
use std::io::Write;

fn write_to_file (data_to_write: String, file_name: String) -> Result<String,String> {

    match OpenOptions::new().write(true).create(true).open(file_name.clone()) {
        Ok(mut file) => {
            match file.write_all( data_to_write.as_bytes() ) {
                Ok(_) => { },
                Err(error) => {
                    return Err(format!(" \"{}\" occurred while writing to \"{}\"", error.to_string(), file_name.clone()));
                }
            };
        },
        Err(error) => {
            return Err(format!(" \"{}\" occurred while creating file \"{}\"", error.to_string(), file_name.clone()));
        }
    };

    return Ok(format!("Data was written to \"{}\"",file_name));
}
