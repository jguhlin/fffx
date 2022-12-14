#![no_main]
use libfuzzer_sys::fuzz_target;
extern crate fffx;

fuzz_target!(|data: &[u8]| {
    let mut buf = std::io::BufReader::new(data);
    let mut fastq = fffx::fastq::Fastq::from_buffer(&mut buf);
    while let Some(Ok(_)) = fastq.next() {
        true;
    }
});
