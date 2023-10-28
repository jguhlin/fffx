// TODO

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let seq = include_str!("../bench_data/uniprot_sprot.fasta");
    let mut bufreader = std::io::BufReader::new(seq.as_bytes());

    let mut group = c.benchmark_group("Parse UniProt SwissProt FASTA File");

    group.bench_function("needletail_parse_fasta", |b| {
        b.iter(|| {
            let mut fasta = needletail::parse_fastx_reader(black_box(seq.as_bytes())).expect("Unable to parse");
            while let Some(r) = fasta.next() {
                let _ = r.unwrap();
            }
        })
    });

    let seq = include_str!("../bench_data/uniprot_sprot.fasta");
    let mut bufreader = std::io::BufReader::new(seq.as_bytes());
    
    group.throughput(criterion::Throughput::Bytes(seq.len() as u64));
    group.bench_function("fffx_parse_fasta", |b| {
        b.iter(|| {
            let fasta = fffx::fasta::Fasta::from_buffer(black_box(&mut bufreader));
            for seq in fasta {
                let _ = seq.unwrap();
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
