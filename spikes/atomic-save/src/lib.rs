use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Fingerprint(String);

#[derive(Debug)]
pub enum SaveError {
    ExternalChange,
    Io(std::io::Error),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecoveryState {
    DiskUnchanged,
    DiskChanged,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecoveryOffer {
    pub draft: String,
    pub disk_state: RecoveryState,
}

#[derive(Debug, Deserialize, Serialize)]
struct RecoveryRecord {
    version: u8,
    document_path: String,
    base_fingerprint: String,
    draft: String,
}

impl fmt::Display for SaveError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExternalChange => write!(formatter, "document changed on disk"),
            Self::Io(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for SaveError {}

impl From<std::io::Error> for SaveError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

pub fn fingerprint(path: &Path) -> Result<Fingerprint, SaveError> {
    let bytes = fs::read(path)?;
    let digest = Sha256::digest(bytes);
    let encoded = digest.iter().map(|byte| format!("{byte:02x}")).collect();
    Ok(Fingerprint(encoded))
}

pub fn save_if_unchanged(
    path: &Path,
    expected: &Fingerprint,
    text: &str,
) -> Result<Fingerprint, SaveError> {
    if &fingerprint(path)? != expected {
        return Err(SaveError::ExternalChange);
    }

    let parent = path.parent().ok_or_else(|| {
        SaveError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "document has no parent directory",
        ))
    })?;
    let permissions = fs::metadata(path)?.permissions();
    let temporary_path = temporary_path_for(path)?;
    let mut cleanup = TemporaryFile::new(temporary_path.clone());

    let mut temporary = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&temporary_path)?;
    temporary.write_all(text.as_bytes())?;
    temporary.set_permissions(permissions)?;
    temporary.sync_all()?;
    drop(temporary);

    fs::rename(&temporary_path, path)?;
    cleanup.persisted = true;
    File::open(parent)?.sync_all()?;

    fingerprint(path)
}

pub fn write_recovery(
    recovery_directory: &Path,
    document: &Path,
    base: &Fingerprint,
    draft: &str,
) -> Result<(), SaveError> {
    fs::create_dir_all(recovery_directory)?;
    let recovery_path = recovery_path(recovery_directory, document);
    let temporary_path = temporary_path_for(&recovery_path)?;
    let mut cleanup = TemporaryFile::new(temporary_path.clone());
    let record = RecoveryRecord {
        version: 1,
        document_path: document.to_string_lossy().into_owned(),
        base_fingerprint: base.0.clone(),
        draft: draft.to_owned(),
    };
    let encoded = serde_json::to_vec(&record).map_err(|error| {
        SaveError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, error))
    })?;

    let mut temporary = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&temporary_path)?;
    temporary.write_all(&encoded)?;
    temporary.sync_all()?;
    drop(temporary);

    fs::rename(&temporary_path, &recovery_path)?;
    cleanup.persisted = true;
    File::open(recovery_directory)?.sync_all()?;
    Ok(())
}

pub fn load_recovery(
    recovery_directory: &Path,
    document: &Path,
) -> Result<Option<RecoveryOffer>, SaveError> {
    let recovery_path = recovery_path(recovery_directory, document);
    let encoded = match fs::read(&recovery_path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.into()),
    };
    let record: RecoveryRecord = serde_json::from_slice(&encoded).map_err(|error| {
        SaveError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, error))
    })?;
    if record.version != 1 || record.document_path != document.to_string_lossy() {
        return Err(SaveError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "recovery record does not match the document",
        )));
    }

    let disk_text = fs::read_to_string(document)?;
    if disk_text == record.draft {
        return Ok(None);
    }
    let disk_state = if fingerprint(document)?.0 == record.base_fingerprint {
        RecoveryState::DiskUnchanged
    } else {
        RecoveryState::DiskChanged
    };

    Ok(Some(RecoveryOffer {
        draft: record.draft,
        disk_state,
    }))
}

fn recovery_path(recovery_directory: &Path, document: &Path) -> PathBuf {
    let digest = Sha256::digest(document.to_string_lossy().as_bytes());
    let encoded: String = digest.iter().map(|byte| format!("{byte:02x}")).collect();
    recovery_directory.join(format!("{encoded}.json"))
}

fn temporary_path_for(path: &Path) -> Result<PathBuf, SaveError> {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            SaveError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "document name is not valid UTF-8",
            ))
        })?;
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| SaveError::Io(std::io::Error::other(error)))?
        .as_nanos();
    Ok(path.with_file_name(format!(
        ".{file_name}.minimark-{}-{nonce}.tmp",
        std::process::id()
    )))
}

struct TemporaryFile {
    path: PathBuf,
    persisted: bool,
}

impl TemporaryFile {
    fn new(path: PathBuf) -> Self {
        Self {
            path,
            persisted: false,
        }
    }
}

impl Drop for TemporaryFile {
    fn drop(&mut self) {
        if !self.persisted {
            let _ = fs::remove_file(&self.path);
        }
    }
}
