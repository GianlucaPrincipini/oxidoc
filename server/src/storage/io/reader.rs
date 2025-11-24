// Questo modulo fornisce la struct FileReader per leggere dati da un file su disco.
// È pensato per essere semplice e sicuro: controlla che il file esista prima di aprirlo e fornisce un metodo per leggere i dati in un buffer.
//
// In Rust, lavorare con i file richiede di gestire i possibili errori (ad esempio, file non trovati o permessi insufficienti).
// Questo esempio mostra come incapsulare la logica di apertura e lettura di un file in una struct dedicata.

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// FileReader è una struct che incapsula un file aperto in sola lettura.
///
/// # Scopo
/// Permette di leggere dati da un file in modo semplice e sicuro.
///
/// # Esempio d'uso
/// ```rust
/// let path = std::path::Path::new("file.txt");
/// let mut reader = FileReader::new(&path);
/// let mut buffer = [0; 10];
/// let bytes_read = reader.read(&mut buffer).unwrap();
/// ```
///
/// # Dettagli
/// - Il file viene aperto solo se esiste già. Se il file non esiste, il programma va in panic (si interrompe con un messaggio di errore).
/// - Il metodo `read` permette di leggere una quantità arbitraria di byte dal file in un buffer fornito dall'utente.
/// - La struct gestisce internamente il file, quindi non devi preoccuparti di chiuderlo: Rust lo chiude automaticamente quando la struct esce dallo scope.
pub struct FileReader {
    /// Il campo `file` contiene il file aperto in sola lettura.
    file: File,
}

impl FileReader {
    /// Crea una nuova istanza di FileReader aprendo il file specificato dal percorso.
    ///
    /// # Argomenti
    /// * `path` - Un riferimento a un oggetto Path che rappresenta il percorso del file da aprire.
    ///
    /// # Panics
    /// Se il file non esiste, la funzione va in panic e mostra un messaggio di errore.
    ///
    /// # Nota per chi viene da Scala
    /// In Rust, il "panic" è simile a un'eccezione non gestita: interrompe il programma. In produzione, è meglio gestire gli errori con Result.
    fn new(path: &Path) -> FileReader {
        if !path.exists() {
            panic!("File does not exist: {:?}", path);
        }
        FileReader {file: File::open(path).unwrap()}
    }
    
    /// Legge dati dal file nel buffer fornito.
    ///
    /// # Argomenti
    /// * `buffer` - Un riferimento mutabile a un array di byte dove verranno scritti i dati letti dal file.
    ///
    /// # Ritorna
    /// Restituisce un Result che contiene il numero di byte effettivamente letti, oppure un errore I/O.
    ///
    /// # Nota per chi viene da Scala
    /// In Rust, il metodo non restituisce direttamente i dati, ma li scrive nel buffer che passi come argomento. Questo è più efficiente e ti permette di riutilizzare la memoria.
    fn read(&mut self, buffer: &mut [u8]) ->std::io::Result<usize> {
        self.file.read(buffer)
    } 
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;

    #[test]
    fn creates_file_reader_for_existing_file() {
        let path = Path::new("test_file.txt");
        let mut file = File::create(&path).unwrap();
        writeln!(file, "Test content").unwrap();

        let file_reader = FileReader::new(&path);
        assert!(file_reader.file.metadata().is_ok());

        fs::remove_file(&path).unwrap();
    }

    #[test]
    #[should_panic(expected = "File does not exist")]
    fn panics_when_file_does_not_exist() {
        let path = Path::new("non_existent_file.txt");
        FileReader::new(&path);
    }

    #[test]
    fn reads_data_into_buffer() {
        let path = Path::new("test_file.txt");
        let mut file = File::create(&path).unwrap();
        writeln!(file, "Test content").unwrap();
    
        let mut file_reader = FileReader::new(&path);
        let mut buffer = [0; 12];
        let bytes_read = file_reader.read(&mut buffer).unwrap();
    
        assert_eq!(bytes_read, 12);
        assert_eq!(&buffer[..], b"Test content");
    
        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn handles_empty_file() {
        let path = Path::new("empty_file.txt");
        File::create(&path).unwrap();

        let mut file_reader = FileReader::new(&path);
        let mut buffer = [0; 10];
        let bytes_read = file_reader.read(&mut buffer).unwrap();

        assert_eq!(bytes_read, 0);

        fs::remove_file(&path).unwrap();
    }
}