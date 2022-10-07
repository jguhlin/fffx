use simdutf8::basic::from_utf8;

#[derive(PartialEq, Eq, Debug)]
pub enum FileFormat {
    FASTA,
    FASTQ,
    SFASTA,  // TODO
    GFA,     // TODO
}

#[allow(dead_code)]
pub fn detect_file_format(buffer: &[u8]) -> Result<FileFormat, &'static str> {
    let buffer = from_utf8(&buffer).expect("Unable to parse file as UTF-8");
    if buffer.starts_with(">") {
        Ok(FileFormat::FASTA)
    } else if buffer.starts_with("@") {
        Ok(FileFormat::FASTQ)
    } else if buffer.starts_with("sfasta") {
        Ok(FileFormat::SFASTA)
    } else {
        Err("Unknown file format")
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum CompressionType {
    GZIP,
    BZIP2,
    XZ,
    RAR,
    ZSTD,
    LZ4,
    LZMA,
    NONE,
}

/// Return the compression type of a file
#[allow(dead_code)]
pub fn detect_compression_format(buffer: &[u8]) -> Result<CompressionType, &'static str> {
    Ok(match buffer {
        [0x1F, 0x8B, ..] => CompressionType::GZIP,
        [0x42, 0x5A, ..] => CompressionType::BZIP2,
        [0xFD, b'7', b'z', b'X', b'Z', 0x00] => CompressionType::XZ,
        [0x28, 0xB5, 0x2F, 0xFD, ..] => CompressionType::LZMA,
        [0x5D, 0x00, ..] => CompressionType::LZMA,
        [0x1F, 0x9D, ..] => CompressionType::LZMA,
        [0x37, 0x7A, 0xBC, 0xAF, 0x27, 0x1C] => CompressionType::ZSTD,
        [0x04, 0x22, 0x4D, 0x18, ..] => CompressionType::LZ4,
        [0x08, 0x22, 0x4D, 0x18, ..] => CompressionType::LZ4,
        [0x52, 0x61, 0x72, 0x21, 0x1A, 0x07] => CompressionType::RAR,
        _ => CompressionType::NONE,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_file_format() {
        assert_eq!(
            detect_file_format(b">seq1\nACTGCATCAGCATCGATCGACTACTGACAC"), 
            Ok(FileFormat::FASTA)
        );
        assert_eq!(
            detect_file_format(b"@seq1\nACTGCATCAGCATCGATCGACTACTGACAC"), 
            Ok(FileFormat::FASTQ)
        );
        assert_eq!(
            detect_file_format(b"sfasta......................................................"), 
            Ok(FileFormat::SFASTA)
        );
        assert_eq!(
            detect_file_format(b"seq1\nACTGCATCAGCATCGATCGACTACTGACAC"), 
            Err("Unknown file format")
        );
    }

    #[test]
    fn test_detect_compression() {
        assert_eq!(
            detect_compression_format(&[0x1F, 0x8B, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03]),
            Ok(CompressionType::GZIP)
        );
        assert_eq!(
            detect_compression_format(&[0x42, 0x5A, 0x68, 0x39, 0x31, 0x41, 0x59, 0x26, 0x53, 0x59]),
            Ok(CompressionType::BZIP2)
        );
        assert_eq!(
            detect_compression_format(&[0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00]),
            Ok(CompressionType::XZ)
        );
        assert_eq!(
            detect_compression_format(&[0x28, 0xB5, 0x2F, 0xFD, 0x37]),
            Ok(CompressionType::LZMA)
        );
        assert_eq!(
            detect_compression_format(&[0x5D, 0x00, 0x00, 0x80, 0x00, 0x00]),
            Ok(CompressionType::LZMA)
        );
        assert_eq!(
            detect_compression_format(&[0x1F, 0x9D, 0x90, 0x00, 0x00, 0x00]),
            Ok(CompressionType::LZMA)
        );
        assert_eq!(
            detect_compression_format(&[0x37, 0x7A, 0xBC, 0xAF, 0x27, 0x1C]),
            Ok(CompressionType::ZSTD)
        );
        assert_eq!(
            detect_compression_format(&[0x04, 0x22, 0x4D, 0x18, 0x04, 0x40, 0x40, 0x80, 0x80, 0x00]),
            Ok(CompressionType::LZ4)
        );
        assert_eq!(
            detect_compression_format(&[0x08, 0x22, 0x4D, 0x18, 0x04, 0x40, 0x40, 0x80, 0x80, 0x00]),
            Ok(CompressionType::LZ4)
        );

    }

}
