//! Basic wrappers for external crate types

// Wrapper for ntools VTK format variants
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum CliVtkFormat {
    Xml,
    LegacyAscii,
    LegacyBinary,
}

impl From<CliVtkFormat> for ntools::weights::vtk::VtkFormat {
    fn from(format: CliVtkFormat) -> Self {
        match format {
            CliVtkFormat::Xml => ntools::weights::vtk::VtkFormat::Xml,
            CliVtkFormat::LegacyAscii => ntools::weights::vtk::VtkFormat::LegacyAscii,
            CliVtkFormat::LegacyBinary => ntools::weights::vtk::VtkFormat::LegacyBinary,
        }
    }
}

// Wrapper for byte order used by vtkio
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum CliByteOrder {
    BigEndian,
    LittleEndian,
}

impl From<CliByteOrder> for vtkio::model::ByteOrder {
    fn from(endian: CliByteOrder) -> Self {
        match endian {
            CliByteOrder::LittleEndian => vtkio::model::ByteOrder::LittleEndian,
            CliByteOrder::BigEndian => vtkio::model::ByteOrder::BigEndian,
        }
    }
}

// Wrapper for compression strategy used by vtkio
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum CliCompressor {
    LZ4,
    ZLib,
    LZMA,
    None,
}

impl From<CliCompressor> for vtkio::xml::Compressor {
    fn from(compressor: CliCompressor) -> Self {
        match compressor {
            CliCompressor::LZMA => vtkio::xml::Compressor::LZMA,
            CliCompressor::LZ4 => vtkio::xml::Compressor::LZ4,
            CliCompressor::ZLib => vtkio::xml::Compressor::ZLib,
            CliCompressor::None => vtkio::xml::Compressor::None,
        }
    }
}
