use crate::PersistentStorageError;

use super::common::{LogCommand, LogEntry, LogIndex, PersistentStorage, ServerId, TermIndex};
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufReader, BufWriter, Seek, Write};
use std::marker::PhantomData;
use std::mem;
use std::path::Path;

use bincode::Options;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
struct Election {
    current_term: TermIndex,
    voted_for: Option<(TermIndex, ServerId)>,
}

type WALBincodeOptions = bincode::config::WithOtherEndian<
    bincode::config::WithOtherIntEncoding<
        bincode::config::WithOtherTrailing<
            bincode::config::WithOtherLimit<
                bincode::config::DefaultOptions,
                bincode::config::Bounded,
            >,
            bincode::config::RejectTrailing,
        >,
        bincode::config::VarintEncoding,
    >,
    bincode::config::LittleEndian,
>;
#[inline]
fn get_election_bincode() -> WALBincodeOptions {
    bincode::DefaultOptions::new()
        .with_limit(mem::size_of::<Election>().try_into().unwrap())
        .reject_trailing_bytes()
        .with_varint_encoding()
        .with_little_endian()
}

/// WAL, should only be used from one thread
#[derive(Debug)]
pub struct DefaultPersistentStorage<C: LogCommand> {
    election: Election,
    election_writer: BufWriter<File>,
    /// This is a placeholder so we can use the generic type parameters C:LogCommand
    /// We will need this type param later when we implement the WAL
    placeholder: PhantomData<C>,
}
impl<C: LogCommand> DefaultPersistentStorage<C> {
    pub fn new(log_path: &Path) -> Self {
        // TODO: Currently we only care about persisting election data, will eventually need WAL
        let (election, election_writer) = Self::open_election_file(log_path);

        DefaultPersistentStorage {
            election,
            election_writer,
            placeholder: PhantomData,
        }
    }

    fn open_election_file(log_path: &Path) -> (Election, BufWriter<File>) {
        let file_size: usize = mem::size_of::<Election>();
        let election_file_exists = log_path.join("election").exists();
        let (reader, mut writer) = File::options()
            .create(true)
            .read(true)
            .write(true)
            .open(log_path.join("election"))
            .and_then(|f| f.set_len(file_size as u64).map(|_| f))
            .and_then(|f| {
                f.try_clone()
                    .map(|f_cloned| (BufReader::new(f), BufWriter::new(f_cloned)))
            })
            .expect("OPEN ELEC FILE: Could not open election file and set file size!");

        if election_file_exists {
            let header = get_election_bincode()
                .deserialize_from(reader)
                .expect("OPEN ELEC FILE: Could not deserialize election file!");
            (header, writer)
        } else {
            let election = Election {
                current_term: TermIndex(0),
                voted_for: None,
            };
            Self::write_election_state(&election, &mut writer)
                .expect("OPEN ELEC FILE: Could not write initial state to election file!");
            writer
                .flush()
                .expect("OPEN ELEC FILE: Could not fsync header to WAL!");
            (election, writer)
        }
    }

    fn write_election_state(
        election: &Election,
        election_writer: &mut BufWriter<File>,
    ) -> Result<(), PersistentStorageError> {
        election_writer
            .rewind()
            .map_err(|_| PersistentStorageError::IoError)?;
        get_election_bincode()
            .serialize_into(election_writer, election)
            .map_err(|_| PersistentStorageError::IoError)?;
        Ok(())
    }
}

impl<C: LogCommand> PersistentStorage<C> for DefaultPersistentStorage<C> {
    fn voted_for(&self) -> Option<ServerId> {
        self.election
            .voted_for
            .and_then(|(last_vote_term, server_id)| {
                if last_vote_term == self.election.current_term {
                    Some(server_id)
                } else {
                    None
                }
            })
    }

    fn update_term(&mut self, term: TermIndex) -> &mut Self {
        self.election.current_term = term;
        self
    }

    fn record_vote(&mut self, voted_for: ServerId) -> &mut Self {
        self.election.voted_for = Some((self.current_term(), voted_for));
        self
    }

    fn sync(&mut self) -> Result<(), PersistentStorageError> {
        Self::write_election_state(&self.election, &mut self.election_writer)?;
        match self.election_writer.flush() {
            Ok(_) => Ok(()),
            Err(_) => Err(PersistentStorageError::IoError),
        }
    }

    fn current_term(&self) -> TermIndex {
        self.election.current_term
    }

    fn last_entry_index(&self) -> Option<LogIndex> {
        todo!()
    }

    /// Checks if there is a log entry with matching log index & log term
    fn has_entry(self, _index: LogIndex, _term: TermIndex) -> bool {
        todo!()
    }
    /// Appends new entries to log, first deleting any conflicting entries (same index but different terms)
    fn append(&mut self, _entries: Vec<LogEntry<C>>) -> &mut Self {
        todo!()
    }
}
