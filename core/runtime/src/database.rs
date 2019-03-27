use crate::FutRuntimeResult;

/// Specify the category of data stored, and users can store the data in a decentralized manner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataCategory {
    // Block
    Block,
    // Already of "SignedTransaction" in the block.
    Transaction,
    // Already of "Receipt" in the block.
    Receipt,
    // State of the world
    State,
    // "SignedTransaction" in the transaction pool
    TransactionPool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DatabaseError {
    NotFound,
    InvalidData,
    Internal(String),
}

pub trait Database: Send + Sync {
    fn get(&self, c: DataCategory, key: &[u8]) -> FutRuntimeResult<Vec<u8>, DatabaseError>;

    fn get_batch(
        &self,
        c: DataCategory,
        keys: &[Vec<u8>],
    ) -> FutRuntimeResult<Vec<Option<Vec<u8>>>, DatabaseError>;

    fn insert(
        &self,
        c: DataCategory,
        key: &[u8],
        value: &[u8],
    ) -> FutRuntimeResult<(), DatabaseError>;

    fn insert_batch(
        &self,
        c: DataCategory,
        keys: &[Vec<u8>],
        values: &[Vec<u8>],
    ) -> FutRuntimeResult<(), DatabaseError>;

    fn contains(&self, c: DataCategory, key: &[u8]) -> FutRuntimeResult<bool, DatabaseError>;

    fn remove(&self, c: DataCategory, key: &[u8]) -> FutRuntimeResult<(), DatabaseError>;

    fn remove_batch(
        &self,
        c: DataCategory,
        keys: &[Vec<u8>],
    ) -> FutRuntimeResult<(), DatabaseError>;
}
