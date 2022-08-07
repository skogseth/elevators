mod get;
mod send;

#[cfg(test)]
mod tests {
    use std::io::prelude::*;
    use std::net::SocketAddr;
    use std::net::{TcpListener, TcpStream};

    #[test]
    fn read_and_write() {
        const HOST: [u8; 4] = [127, 0, 0, 1];
        const PORT: u16 = 10000;

        let handle = std::thread::spawn(|| {
            let addr = SocketAddr::from((HOST, PORT));
            let listener = TcpListener::bind(addr).expect("failed to create listener");

            let mut stream = match listener.accept() {
                Ok((socket, _addr)) => socket,
                Err(e) => panic!("failed to accept stream (listener) => error: {e}"),
            };

            let mut buffer: [u8; 5] = [0; 5];
            if let Err(e) = stream.read(&mut buffer) {
                panic!("failed to read buffer (listener) {buffer:?} => error: {e}");
            }
            assert_eq!(buffer, [1, 2, 3, 4, 5]);

            buffer = [5, 4, 3, 2, 1];
            if let Err(e) = stream.write(&buffer) {
                panic!("failed to write buffer (listener) {buffer:?} => error: {e}");
            }
        });

        let addr = SocketAddr::from((HOST, PORT));
        let mut stream = match TcpStream::connect(addr) {
            Ok(s) => s,
            Err(_) => panic!("failed to connect to addr {addr}"),
        };

        let mut buffer: [u8; 5] = [1, 2, 3, 4, 5];
        if let Err(e) = stream.write(&buffer) {
            panic!("failed to write buffer {buffer:?} => error: {e}");
        }

        if let Err(e) = stream.read(&mut buffer) {
            panic!("failed to read to buffer {buffer:?} => error: {e}");
        }
        assert_eq!(buffer, [5, 4, 3, 2, 1]);

        if let Err(_) = handle.join() {
            panic!("failed to join listener thread");
        }
    }
}