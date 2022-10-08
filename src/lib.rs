//! Yet another rust fasta/fastq parser, with a focus on speed and memory usage. Well fuzzed and performant-focused.
//! 
//! The basic ones to use are fasta, fastq submodules.
//! 
//! The ones under zerocopy:: use slightly less memory at the cost of discontiguous streams.

pub mod fasta;
pub mod fastq;
pub mod utils;

#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct Sequence {
    pub sequence: Option<Vec<u8>>,
    pub scores: Option<Vec<u8>>,
    pub header: Option<String>,
    pub id: Option<String>,
    /// Primarily used downstream, but when used for random access this is the offset from the start of the sequence
    pub offset: usize,
}