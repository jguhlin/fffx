//! Yet another rust fasta/fastq parser, with a focus on speed and memory usage. Well fuzzed and performant-focused.
//!
//! The basic ones to use are fasta, fastq submodules.
//!
//! The ones under zerocopy:: use slightly less memory at the cost of discontiguous streams.

pub mod fasta;
pub mod fastq;
pub mod utils;

pub use fasta::Fasta;
pub use fastq::Fastq;

#[derive(PartialEq, Eq, Clone, Debug, Default)]
pub struct Sequence {
    pub sequence: Option<Vec<u8>>,
    pub scores: Option<Vec<u8>>,
    pub header: Option<String>,
    pub id: Option<String>,
    /// Primarily used downstream, but when used for random access this is the offset from the start of the sequence
    pub offset: usize,
}

#[cfg(test)]
mod test {
    // Using this, count the total number of bases
    use super::*;
    use std::io::BufReader;

    #[test]
    fn fasta_test() {
        let seq = include_str!("../bench_data/uniprot_sprot.fasta");
        let mut bufreader = BufReader::new(seq.as_bytes());
        let mut num_bases = 0;
        let fasta = Fasta::from_buffer(&mut bufreader);
        for seq in fasta {
            let seq = seq.unwrap();
            num_bases += seq.sequence.as_ref().unwrap().len();
        }
        assert_eq!(num_bases, 559_023_700);
    }

}
