pub mod get;
pub mod send;
pub mod types;

#[cfg(test)]
mod tests {
    use std::net::SocketAddr;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::{TcpListener, TcpStream};

    #[tokio::test]
    async fn read_and_write() {
        const HOST: [u8; 4] = [127, 0, 0, 1];
        const PORT: u16 = 7878;

        let handle = tokio::spawn(async move {
            let addr = SocketAddr::from((HOST, PORT));
            let mut stream = match TcpStream::connect(addr).await {
                Ok(s) => s,
                Err(_) => panic!("failed to connect to addr {addr}"),
            };

            let mut buffer: [u8; 5] = [1, 2, 3, 4, 5];
            if let Err(e) = stream.write_all(&buffer).await {
                panic!("failed to write buffer {buffer:?} => error: {e}");
            }

            if let Err(e) = stream.read(&mut buffer).await {
                panic!("failed to read to buffer {buffer:?} => error: {e}");
            }
            assert_eq!(buffer, [5, 4, 3, 2, 1]);
        });

        let addr = SocketAddr::from((HOST, PORT));
        let listener = TcpListener::bind(addr)
            .await
            .expect("failed to create listener");

        let mut stream = match listener.accept().await {
            Ok((socket, _addr)) => socket,
            Err(e) => panic!("failed to accept stream (listener) => error: {e}"),
        };

        let mut buffer: [u8; 5] = [0; 5];
        if let Err(e) = stream.read(&mut buffer).await {
            panic!("failed to read buffer (listener) {buffer:?} => error: {e}");
        }
        assert_eq!(buffer, [1, 2, 3, 4, 5]);

        buffer = [5, 4, 3, 2, 1];
        if let Err(e) = stream.write_all(&buffer).await {
            panic!("failed to write buffer (listener) {buffer:?} => error: {e}");
        }

        if let Err(e) = handle.await {
            panic!("failed to await handle => error: {e}");
        }
    }
}
