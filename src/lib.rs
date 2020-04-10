
use std::io::{BufRead};
use exitfailure::ExitFailure;

pub fn find_matches<R: std::io::BufRead>(reader: &mut R, pattern: &str, mut writer: impl std::io::Write) -> Result<(), ExitFailure> {
    for line in reader.lines() {
        let line = line?;
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    };

    Ok(())
}

#[cfg(test)]
mod test {
	use super::*;
	use std::io::BufReader;

	fn read_from_string(s: &str) -> &[u8] {
	    s.as_bytes()
	}

	#[test]
	fn find_a_match() {
	    let mut answer = Vec::new();
	    let string_data = read_from_string("lorem ipsum\ndolor sit amet");
	    let mut reader = BufReader::new(string_data);

	    let _ = find_matches(&mut reader, "lorem", &mut answer);
	    assert_eq!(answer, b"lorem ipsum\n");
	}
}