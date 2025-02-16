use crate::Result;
use std::fs::File;
use std::path::Path;
use tempfile::NamedTempFile;
use zip::ZipWriter;

pub struct Archiver;

impl Archiver {
	pub fn create_archive(paths: &[impl AsRef<Path>]) -> Result<NamedTempFile> {
		let mut temp_file = NamedTempFile::new()?;
		let mut zip = ZipWriter::new(&mut temp_file);

		for path in paths {
			let path = path.as_ref();
			if path.is_dir() {
				Self::add_directory(&mut zip, path, path)?;
			} else {
				Self::add_file(&mut zip, path, path)?;
			}
		}

		zip.finish()?;
		temp_file.as_file_mut().sync_all()?;
		Ok(temp_file)
	}

	fn add_directory<W: std::io::Write + std::io::Seek>(
		zip: &mut ZipWriter<W>,
		path: &Path,
		base: &Path,
	) -> Result<()> {
		for entry in std::fs::read_dir(path)? {
			let entry = entry?;
			let entry_path = entry.path();

			if entry_path.is_dir() {
				Self::add_directory(zip, &entry_path, base)?;
			} else {
				Self::add_file(zip, &entry_path, base)?;
			}
		}
		Ok(())
	}

	fn add_file<W: std::io::Write + std::io::Seek>(zip: &mut ZipWriter<W>, path: &Path, base: &Path) -> Result<()> {
		let relative_path = path.strip_prefix(base)?;
		zip.start_file(relative_path.to_str().unwrap(), Default::default())?;

		let mut file = std::fs::File::open(path)?;
		std::io::copy(&mut file, zip)?;
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::io::Read;
	use tempfile::tempdir;

	#[test]
	fn test_create_archive() {
		let temp_dir = tempdir().unwrap();
		let file_path = temp_dir.path().join("test.txt");
		std::fs::write(&file_path, "test content").unwrap();

		let archive = Archiver::create_archive(&[file_path]).unwrap();
		let mut zip = zip::ZipArchive::new(archive.as_file()).unwrap();

		assert_eq!(zip.len(), 1);
		let mut file = zip.by_index(0).unwrap();
		let mut content = String::new();
		file.read_to_string(&mut content).unwrap();
		assert_eq!(content, "test content");
	}
}
