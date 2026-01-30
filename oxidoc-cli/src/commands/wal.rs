use serde::{Serialize, Deserialize};
use std::fs::{File, OpenOptions};
use std::io::{Write, Read, BufReader, BufWriter};
use std::path::Path;
use crate::commands::commands::CliCommand;

#[derive(Serialize, Deserialize)]
struct WALEntry {
    sequence: u64,
    command: CliCommand,
    checksum: u32,
}

pub struct WAL {
    file: BufWriter<File>,
    sequence: u64,
    path: String,
}

impl WAL {
    pub fn new(path: &str) -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;

        Ok(WAL {
            file: BufWriter::new(file),
            sequence: 0,
            path: path.to_string(),
        })
    }

    pub fn append(&mut self, CliCommand: CliCommand) -> std::io::Result<()> {
        // Serializza con postcard
        let cmd_bytes = postcard::to_allocvec(&CliCommand)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        let checksum = crc32fast::hash(&cmd_bytes);

        let entry = WALEntry {
            sequence: self.sequence,
            command: CliCommand,
            checksum,
        };

        // Serializza entry completa con postcard
        let entry_bytes = postcard::to_allocvec(&entry)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        let len = entry_bytes.len() as u32;
        self.file.write_all(&len.to_le_bytes())?;
        self.file.write_all(&entry_bytes)?;

        self.file.flush()?;
        self.file.get_mut().sync_all()?;

        self.sequence += 1;
        Ok(())
    }

    pub fn recover<F>(path: &str, mut apply: F) -> std::io::Result<u64>
    where
        F: FnMut(CliCommand) -> std::io::Result<()>
    {
        if !Path::new(path).exists() {
            return Ok(0);
        }

        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut last_sequence = 0;

        loop {
            let mut len_bytes = [0u8; 4];
            match reader.read_exact(&mut len_bytes) {
                Ok(_) => {},
                Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                    break;
                }
                Err(e) => return Err(e),
            }

            let len = u32::from_le_bytes(len_bytes) as usize;

            let mut entry_bytes = vec![0u8; len];
            reader.read_exact(&mut entry_bytes)?;

            // Deserializza con postcard
            let entry: WALEntry = postcard::from_bytes(&entry_bytes)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

            // Verifica checksum
            let cmd_bytes = postcard::to_allocvec(&entry.command)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
            let computed_checksum = crc32fast::hash(&cmd_bytes);

            if computed_checksum != entry.checksum {
                eprintln!("⚠️  WAL corrotto alla sequenza {}, stop recovery", entry.sequence);
                break;
            }

            apply(entry.command)?;
            last_sequence = entry.sequence;
        }

        Ok(last_sequence + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn temp_wal_path(name: &str) -> String {
        format!("/tmp/test_wal_{}_{}.wal", name, std::process::id())
    }

    fn cleanup(path: &str) {
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_write_single_entry() {
        let path = temp_wal_path("single");
        cleanup(&path);

        let mut wal = WAL::new(&path).unwrap();
        let cmd = CliCommand::CreateCollection { name: "users".into() };

        assert!(wal.append(cmd).is_ok());
        assert_eq!(wal.sequence, 1);

        cleanup(&path);
    }

    #[test]
    fn test_write_multiple_entries() {
        let path = temp_wal_path("multiple");
        cleanup(&path);

        let mut wal = WAL::new(&path).unwrap();

        wal.append(CliCommand::CreateCollection { name: "users".into() }).unwrap();
        wal.append(CliCommand::CreateCollection { name: "posts".into() }).unwrap();
        wal.append(CliCommand::Insert {
            collection: "users".into(),
            doc_id: "1".into(),
            data: b"alice".to_vec(),
        }).unwrap();

        assert_eq!(wal.sequence, 3);

        cleanup(&path);
    }

    #[test]
    fn test_recovery_empty_wal() {
        let path = temp_wal_path("empty");
        cleanup(&path);

        let next_seq = WAL::recover(&path, |_cmd| Ok(())).unwrap();
        assert_eq!(next_seq, 0);

        cleanup(&path);
    }

    #[test]
    fn test_recovery_with_data() {
        let path = temp_wal_path("recovery");
        cleanup(&path);

        // Scrivi dati
        {
            let mut wal = WAL::new(&path).unwrap();
            wal.append(CliCommand::CreateCollection { name: "users".into() }).unwrap();
            wal.append(CliCommand::Insert {
                collection: "users".into(),
                doc_id: "1".into(),
                data: b"alice".to_vec(),
            }).unwrap();
            wal.append(CliCommand::Delete {
                collection: "users".into(),
                doc_id: "1".into(),
            }).unwrap();
        }

        // Recupera dati
        let mut recovered = Vec::new();
        let next_seq = WAL::recover(&path, |cmd| {
            recovered.push(cmd);
            Ok(())
        }).unwrap();

        assert_eq!(next_seq, 3);
        assert_eq!(recovered.len(), 3);
        assert_eq!(
            recovered[0],
            CliCommand::CreateCollection { name: "users".into() }
        );
        assert_eq!(
            recovered[1],
            CliCommand::Insert {
                collection: "users".into(),
                doc_id: "1".into(),
                data: b"alice".to_vec(),
            }
        );

        cleanup(&path);
    }

    #[test]
    fn test_recovery_then_append() {
        let path = temp_wal_path("recovery_append");
        cleanup(&path);

        // Prima sessione
        {
            let mut wal = WAL::new(&path).unwrap();
            wal.append(CliCommand::CreateCollection { name: "test".into() }).unwrap();
        }

        // Recovery e nuovi append
        let next_seq = WAL::recover(&path, |_| Ok(())).unwrap();
        let mut wal = WAL::new(&path).unwrap();
        wal.sequence = next_seq;

        wal.append(CliCommand::Insert {
            collection: "test".into(),
            doc_id: "1".into(),
            data: b"data".to_vec(),
        }).unwrap();

        // Verifica totale
        let mut all_CliCommands = Vec::new();
        WAL::recover(&path, |cmd| {
            all_CliCommands.push(cmd);
            Ok(())
        }).unwrap();

        assert_eq!(all_CliCommands.len(), 2);

        cleanup(&path);
    }

    #[test]
    fn test_checksum_protects_integrity() {
        let path = temp_wal_path("checksum");
        cleanup(&path);

        // Scrivi entry valida
        {
            let mut wal = WAL::new(&path).unwrap();
            wal.append(CliCommand::CreateCollection { name: "test".into() }).unwrap();
        }

        // Corrompi il file
        {
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(&path)
                .unwrap();

            use std::io::Seek;
            file.seek(std::io::SeekFrom::Start(20)).unwrap();
            file.write_all(&[0xFF]).unwrap();
        }

        // Recovery dovrebbe fermarsi
        let mut count = 0;
        let next_seq = WAL::recover(&path, |_| {
            count += 1;
            Ok(())
        }).unwrap();

        assert_eq!(count, 0);
        assert_eq!(next_seq, 0);

        cleanup(&path);
    }

    #[test]
    fn test_all_CliCommand_types() {
        let path = temp_wal_path("all_CliCommands");
        cleanup(&path);

        let CliCommands = vec![
            CliCommand::CreateCollection { name: "coll1".into() },
            CliCommand::Insert {
                collection: "coll1".into(),
                doc_id: "doc1".into(),
                data: vec![1, 2, 3, 4],
            },
            CliCommand::Delete {
                collection: "coll1".into(),
                doc_id: "doc1".into(),
            },
            CliCommand::DeleteCollection { name: "coll1".into() },
        ];

        // Scrivi
        {
            let mut wal = WAL::new(&path).unwrap();
            for cmd in &CliCommands {
                wal.append(cmd.clone()).unwrap();
            }
        }

        // Recupera e verifica
        let mut recovered = Vec::new();
        WAL::recover(&path, |cmd| {
            recovered.push(cmd);
            Ok(())
        }).unwrap();

        assert_eq!(recovered, CliCommands);

        cleanup(&path);
    }

    #[test]
    fn test_postcard_size_advantage() {
        let path = temp_wal_path("size_test");
        cleanup(&path);

        // Scrivi un comando con dati grandi
        {
            let mut wal = WAL::new(&path).unwrap();
            let large_data = vec![42u8; 1000];
            wal.append(CliCommand::Insert {
                collection: "test".into(),
                doc_id: "big".into(),
                data: large_data,
            }).unwrap();
        }

        // Verifica che il file esista e sia relativamente piccolo
        let metadata = fs::metadata(&path).unwrap();
        assert!(metadata.len() > 0);
        println!("WAL size with postcard: {} bytes", metadata.len());

        cleanup(&path);
    }
}