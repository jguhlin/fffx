# Yet another fasta/q/x parser

Well fuzzed though....

## Benchmark
Use needletail! This is more a fun experiment to me.

## Changelog
### 0.1.4 UNRELEASED
Add zlib-ng as a feature

### 0.1.3
Pin bytelines to 2.2.2 to remove the tokio/futures dependency as it is not used here.

### 0.1.2
Change zlib-ng for flate2 to just zlib.
