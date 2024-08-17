#[derive(thiserror::Error, Debug)]
pub (crate) enum SlateDBError {
    #[error("IO error")]
    IoError(#[from] std::io::Error),

    #[error("Checksum mismatch")]
    ChecksumMismatch,

    #[error("Empty SSTable")]
    EmptySSTable,

    #[error("Empty block metadata")]
    EmptyBlockMeta,

    #[error("Empty block")]
    EmptyBlock,

    #[error("Object store error")]
    ObjectStoreError(#[from] object_store::Error),

    #[error("Manifest file already exists")]
    ManifestVersionExists,

    #[error("Invalid sst error")]
    InvalidFlatbuffer(#[from] flatbuffers::InvalidFlatbuffer),

    #[error("Invalid DB state error")]
    InvalidDBState,

    #[error("Invalid Compaction")]
    InvalidCompaction,

    #[error("Detected newer DB client")]
    Fenced,
}
