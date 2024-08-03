use bio::io::fasta::{self};
use serde::Serialize;

use crate::primer::Base;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Hash)]
pub enum NotBase {
    N,
}

impl TryFrom<char> for NotBase {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'N' => Ok(NotBase::N),
            other => Err(format!("Expected N, got {other}")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Hash)]
pub enum FastaBase {
    Base(Base),
    NotBase(NotBase),
}

pub trait Parser {
    fn parse_into(fasta_file_content: &str) -> Vec<Vec<FastaBase>>;
}

pub struct FastaParser {}
impl Parser for FastaParser {
    fn parse_into(fasta_file_content: &str) -> Vec<Vec<FastaBase>> {
        let reader = fasta::Reader::new(fasta_file_content.as_bytes());
        reader
            .records()
            .into_iter()
            .map(|record| {
                record
                    .unwrap()
                    .seq()
                    .iter()
                    .map(|&b| b as char)
                    .map(|base| match base {
                        'N' => Some(FastaBase::NotBase(NotBase::N)),
                        'A' => Some(FastaBase::Base(Base::A)),
                        'T' => Some(FastaBase::Base(Base::T)),
                        'G' => Some(FastaBase::Base(Base::G)),
                        'C' => Some(FastaBase::Base(Base::C)),
                        _ => None,
                    })
                    .map(|maybe_base| maybe_base.unwrap())
                    .collect::<Vec<FastaBase>>()
            })
            .collect::<Vec<Vec<FastaBase>>>()
    }
}

#[cfg(test)]
mod tests {
    // TODO: add tests
    fn sequence_output() {}
}
