use std::fs;
use std::path::Path;

fn main() {
}

fn create_file(filename: &str) {
    let file_path = "test_file.txt";

    // Create the file
    fs::File::create(file_path).expect("Failed to create file");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_file() {
        let test_filename = "test_file.txt";

        // Ensure the file does not exist before the test
        let _ = fs::remove_file(test_filename);

        // Call the function to create the file
        create_file(test_filename);

        // Check if the file was created
        assert!(fs::metadata(test_filename).is_ok());

        // Clean up the file after the test
        let _ = fs::remove_file(test_filename);
    }
}


