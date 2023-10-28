// TODO

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parse UniProt SwissProt FASTA File");
    let file_size = std::fs::metadata("bench_data/uniprot_sprot.fasta").unwrap().len();

    group.throughput(criterion::Throughput::Bytes(file_size));

    group.bench_function("needletail_parse_fasta", |b| {
        b.iter(|| {
            let fasta_file = std::fs::File::open("bench_data/uniprot_sprot.fasta").unwrap();
            let mut bufreader = std::io::BufReader::new(fasta_file);

            let mut num_bases = 0;
            let mut fasta = needletail::parse_fastx_reader(black_box(seq.as_bytes())).expect("Unable to parse");
            while let Some(r) = fasta.next() {
                let seqrec = r.unwrap();
                num_bases += seqrec.num_bases();
            }
        })
    });

    // let seq = include_str!("../bench_data/uniprot_sprot.fasta");
    // let mut bufreader = std::io::BufReader::new(seq.as_bytes());
   
    group.bench_function("fffx_parse_fasta", |b| {
        b.iter(|| {
            let fasta_file = std::fs::File::open("bench_data/uniprot_sprot.fasta").unwrap();
            let mut bufreader = std::io::BufReader::new(fasta_file);

            let mut num_bases = 0;
            let fasta = fffx::fasta::Fasta::from_buffer(black_box(&mut bufreader));
            for seq in fasta {
                let seq = seq.unwrap();
                num_bases += seq.sequence.as_ref().unwrap().len();
            }
        })
    });

    group.finish();

    // ----------------------------------------------
    // Parse FASTQ
    // ----------------------------------------------

    let mut group = c.benchmark_group("Parse biofast-data-v1/M_abscessus_HiSeq.fq FASTQ");
    let file_size = std::fs::metadata("bench_data/biofast-data-v1/M_abscessus_HiSeq.fq").unwrap().len();
    group.throughput(file_size);

    group.bench_function("needletail_parse_fastq", |b| {
        b.iter(|| {
            let fasta_file = std::fs::File::open("bench_data/biofast-data-v1/M_abscessus_HiSeq.fq").unwrap();
            let mut bufreader = std::io::BufReader::new(fasta_file);

            let mut num_bases = 0;
            let mut fasta = needletail::parse_fastx_reader(black_box(seq.as_bytes())).expect("Unable to parse");
            while let Some(r) = fasta.next() {
                let seqrec = r.unwrap();
                num_bases += seqrec.num_bases();
            }
        })
    });

    // let seq = include_str!("../bench_data/uniprot_sprot.fasta");
    // let mut bufreader = std::io::BufReader::new(seq.as_bytes());
   
    group.bench_function("fffx_parse_fastq", |b| {
        b.iter(|| {
            let fasta_file = std::fs::File::open("bench_data/biofast-data-v1/M_abscessus_HiSeq.fq").unwrap();
            let mut bufreader = std::io::BufReader::new(fasta_file);
            
            let mut num_bases = 0;
            let fasta = fffx::fasta::Fasta::from_buffer(black_box(&mut bufreader));
            for seq in fasta {
                let seq = seq.unwrap();
                num_bases += seq.sequence.as_ref().unwrap().len();
            }
        })
    });

    group.finish();
}

criterion_group! {
    name=parse_fasta;
    config = Criterion::default().significance_level(0.05).measurement_time(std::time::Duration::from_secs(120));
    targets=criterion_benchmark
}

criterion_main!(parse_fasta);
