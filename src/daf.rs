//! Definition of types and traits for handling Double Precision Array Files

use crate::byteorder::ByteOrder;

/// The length (in bytes) of the ID word in the DAF File Record
const ID_WORD_LENGTH: usize = 8;

/// The length (in bytes) of the internal name or description in the DAF File Record
const DESCRIPTION_LENGTH: usize = 60;

/// The length (in bytes) of the FTP validation string in the DAF File Record
const FTP_STRING_LENGTH: usize = 28;

/// String indicating that the file is little endian
const LITTLE_ENDIAN_STRING: &str = "LTL-IEEE";

/// String indicating that the file is big endian
const BIG_ENDIAN_STRING: &str = "BIG-IEEE";

#[derive(Debug, Clone, Copy)]
pub struct FileRecord {
    /// An identification word (`DAF/xxxx')
    pub id_word: [u8; ID_WORD_LENGTH],

    /// The number of double precision components in each array summary
    pub n_double: i32,

    /// The number of integer components in each array summary
    pub n_integer: i32,

    /// The internal name or description of the array file
    pub description: [u8; DESCRIPTION_LENGTH],

    /// The record number of the initial summary record in the file
    pub forward: i32,

    /// The record number of the final summary record in the file
    pub backward: i32,

    /// The first free address in the file
    ///
    /// This is the address at which the first element of the next array to be added to the file
    /// will be stored..
    pub first_free: i32,

    /// The indicatation of the numeric binary format of the DAF
    pub byte_ordering: ByteOrder,

    /// FTP validation string
    ftp_string: [u8; FTP_STRING_LENGTH],
}

impl FileRecord {
    /// Size of a single summary within a summary record in the DAF file
    pub fn single_summary_size(&self) -> i32 {
        return self.n_double + (self.n_integer + 1) / 2;
    }

    /// Number of characters in a single name in a name record in the DAF file
    pub fn n_character(&self) -> i32 {
        return 8 * self.single_summary_size();
    }

    /// Number of summaries within a single summary record
    pub fn summaries_per_record(&self) -> i32 {
        return 125 / self.single_summary_size();
    }
}

/// Validate the values for the number of double and integer components in the array summaries
fn valid_nd_ni(nd: i32, ni: i32) -> bool {
    let check_1 = (nd + (ni + 1) / 2) <= 125;
    let check_2 = (0 <= nd) && (nd <= 124);
    let check_3 = (2 <= ni) && (ni <= 250);
    return check_1 && check_2 && check_3;
}

impl TryFrom<&[u8]> for FileRecord {
    type Error = &'static str;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 1024 {
            return Err("byte buffer too short to parse file record");
        }

        let mut id_word = [0; ID_WORD_LENGTH];
        let mut description = [0; DESCRIPTION_LENGTH];
        let mut fmt_string = [0; 8];
        let mut ftp_string = [0; FTP_STRING_LENGTH];

        id_word.copy_from_slice(&bytes[0..8]);
        description.copy_from_slice(&bytes[16..76]);
        fmt_string.copy_from_slice(&bytes[88..96]);
        ftp_string.copy_from_slice(&bytes[699..727]);

        let byte_ordering = if fmt_string == LITTLE_ENDIAN_STRING.as_bytes() {
            ByteOrder::LittleEndian
        } else if fmt_string == BIG_ENDIAN_STRING.as_bytes() {
            ByteOrder::BigEndian
        } else {
            return Err("invalid binary format string");
        };

        let n_double = byte_ordering.i32_from_bytes(&bytes[8..12]);
        let n_integer = byte_ordering.i32_from_bytes(&bytes[12..16]);
        let forward = byte_ordering.i32_from_bytes(&bytes[76..80]);
        let backward = byte_ordering.i32_from_bytes(&bytes[80..84]);
        let first_free = byte_ordering.i32_from_bytes(&bytes[84..88]);

        return Ok(FileRecord {
            id_word,
            n_double,
            n_integer,
            description,
            forward,
            backward,
            first_free,
            byte_ordering,
            ftp_string,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::{FileRecord, DESCRIPTION_LENGTH, FTP_STRING_LENGTH, ID_WORD_LENGTH};
    use rand::prelude::*;

    fn random_file_record(nd: i32, ni: i32, f: i32, b: i32, ff: i32) -> FileRecord {
        let mut id_word = [0; ID_WORD_LENGTH];
        let mut description = [0; DESCRIPTION_LENGTH];
        let mut ftp_string = [0; FTP_STRING_LENGTH];

        rand::thread_rng().fill_bytes(&mut id_word);
        rand::thread_rng().fill_bytes(&mut description);
        rand::thread_rng().fill_bytes(&mut ftp_string);

        return FileRecord {
            id_word: id_word,
            n_double: nd,
            n_integer: ni,
            description: description,
            forward: f,
            backward: b,
            first_free: ff,
            byte_ordering: crate::byteorder::ByteOrder::LittleEndian,
            ftp_string: ftp_string,
        };
    }

    #[test]
    fn n_character() {
        let fr = random_file_record(2, 6, 1, 1, 10);
        assert_eq!(40, fr.n_character());
    }

    #[test]
    fn single_summary_size() {
        let fr = random_file_record(2, 6, 1, 1, 10);
        assert_eq!(5, fr.single_summary_size());
    }

    #[test]
    fn summaries_per_record() {
        let fr = random_file_record(1, 3, 1, 1, 10);
        assert_eq!(41, fr.summaries_per_record());
    }
}
