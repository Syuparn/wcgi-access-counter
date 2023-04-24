use std::fs;

pub fn increment_counter(file_path: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let file_str = fs::read_to_string(file_path)?;
    let mut cnt_access = file_str.parse::<i64>()?;
    cnt_access += 1;
    fs::write(file_path, format!("{cnt_access}"))?;
    Ok(cnt_access)
}

#[cfg(test)]
mod tests {
    use crate::counter::increment_counter;
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use tempfile::{tempdir, TempDir};

    #[test]
    fn first_access() {
        // HACK: dir and file should be received here, otherwise they die at the end of prepare_tempfile
        let (_dir, _file, file_path) = prepare_tempfile("counter_zero.txt", "0");
        let actual = increment_counter(&file_path).unwrap();
        assert_eq!(actual, 1);

        let actual_recorded = fs::read_to_string(file_path).unwrap();
        assert_eq!(actual_recorded, "1");
    }

    #[test]
    fn second_access() {
        // HACK: dir and file should be received here, otherwise they die at the end of prepare_tempfile
        let (_dir, _file, file_path) = prepare_tempfile("counter_zero.txt", "1");
        let actual = increment_counter(&file_path).unwrap();
        assert_eq!(actual, 2);

        let actual_recorded = fs::read_to_string(file_path).unwrap();
        assert_eq!(actual_recorded, "2");
    }

    #[test]
    fn popular_site_access() {
        // HACK: dir and file should be received here, otherwise they die at the end of prepare_tempfile
        let (_dir, _file, file_path) = prepare_tempfile("counter_zero.txt", "999999");
        let actual = increment_counter(&file_path).unwrap();
        assert_eq!(actual, 1000000);

        let actual_recorded = fs::read_to_string(file_path).unwrap();
        assert_eq!(actual_recorded, "1000000");
    }

    fn prepare_tempfile(file_name: &str, content: &str) -> (TempDir, File, String) {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join(&file_name);
        let mut file = File::create(&file_path).unwrap();
        write!(file, "{content}").unwrap();

        (dir, file, file_path.to_str().unwrap().to_string())
    }
}
