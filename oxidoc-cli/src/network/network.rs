use std::io::{Error, Read, Write};

pub fn write_message<W: Write>(mut stream: W, message: &[u8]) -> Result<(), Error> {
    let message_length = message.len() as u64;
    stream.write_all(&message_length.to_be_bytes())?;
    stream.write_all(message)
}
pub fn read_message<R: Read>(mut stream: R) -> Result<Vec<u8>, Error> {
    let mut len_buffer = [0u8; 8]; 
    stream.read_exact(&mut len_buffer)?;
    let mut buffer = vec![0u8; u64::from_be_bytes(len_buffer) as usize];
    stream.read_exact(&mut buffer)?;
    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, ErrorKind};

    #[test]
    fn write_and_read_message_success() {
        let message = b"hello world!";
        let mut buffer = Vec::new();
        {
            // Simula la scrittura su uno stream in memoria
            let mut cursor = Cursor::new(&mut buffer);
            write_message(&mut cursor, message).expect("write_message should succeed");
        }
        {
            // Simula la lettura dallo stesso stream in memoria
            let mut cursor = Cursor::new(&buffer);
            let result = read_message(&mut cursor).expect("read_message should succeed");
            assert_eq!(result, message);
        }
    }

    #[test]
    fn read_message_returns_error_on_short_length() {
        // Buffer troppo corto per contenere la lunghezza
        let buffer = vec![0u8, 0, 0];
        let mut cursor = Cursor::new(&buffer);
        let err = read_message(&mut cursor).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
    }

    #[test]
    fn read_message_returns_error_on_short_payload() {
        // Lunghezza dichiarata 10, ma solo 5 byte disponibili
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&10u32.to_be_bytes());
        buffer.extend_from_slice(b"12345");
        let mut cursor = Cursor::new(&buffer);
        let err = read_message(&mut cursor).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::UnexpectedEof);
    }

    #[test]
    fn write_message_writes_length_and_payload() {
        let message = b"abc";
        let mut buffer = Vec::new();
        {
            let mut cursor = Cursor::new(&mut buffer);
            write_message(&mut cursor, message).unwrap();
        }
        // 3 character length message payload buffer
        let mut expected = Vec::new();
        expected.extend_from_slice(&3u64.to_be_bytes());
        expected.extend_from_slice(message);
        assert_eq!(buffer, expected);
    }
}
