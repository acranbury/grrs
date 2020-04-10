use std::process::Command; // Run programs
use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use tempfile::NamedTempFile;
use std::io::{Write};


#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
	let mut cmd = Command::cargo_bin("grrs")?;
	cmd.arg("foobar")
		.arg("test/file/doesnt/exist");
	cmd.assert()
		.failure()
		.stderr(predicate::str::contains("No such file or directory"));

	Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
	let mut file = NamedTempFile::new()?;
	writeln!(file, "A test\nActual content\nMore content\nAnother test")?;

	let mut cmd = Command::cargo_bin("grrs")?;
	cmd.arg("test")
		.arg(file.path());
	cmd.assert()
		.success()
		.stdout(predicate::str::contains("A test\nAnother test"));

	Ok(())
}

#[test]
fn empty_string_as_pattern() -> Result<(), Box<dyn std::error::Error>> {
	let mut file = NamedTempFile::new()?;
	writeln!(file, "A test\nActual content\nMore content\nAnother test")?;

	let mut cmd = Command::cargo_bin("grrs")?;
	cmd.arg("")
		.arg(file.path());
	cmd.assert()
		.success()
		.stdout(predicate::str::contains("A test\nActual content\nMore content\nAnother test"));

	Ok(())
}