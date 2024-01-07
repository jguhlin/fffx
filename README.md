# Yet another fasta/q/x parser

Well fuzzed though....

You should use [needletail](https://crates.io/crates/needletail) though.

## Benchmark
I don't know if this is correct or not, but it's fast enough. NOTE: This does not include I/O at all as that is outside the scope of this library (for now) and thus uniprot is included as a raw string in the binary.
``` 
Running benches/parse_fasta.rs (target/release/deps/parse_fasta-1e449f5527386a9a) 
Parse UniProt SwissProt FASTA File/parse_fasta 
  time: [54.788 ns 54.922 ns 55.069 ns] 
  thrpt: [4773329 GiB/s 4786111 GiB/s 4797786 GiB/s] 

Found 11 outliers among 100 measurements (11.00%)
  4 (4.00%) low mild 4 (4.00%) high mild
  3 (3.00%) high severe
```

## Changelog
### 0.1.4 UNRELEASED
Add zlib-ng as a feature

### 0.1.3
Pin bytelines to 2.2.2 to remove the tokio/futures dependency as it is not used here.

### 0.1.2
Change zlib-ng for flate2 to just zlib.
