use std::{net::TcpListener, io::Read};

    pub struct Server {
        address: String,
    }

    impl Server {
        pub fn new(address: String) -> Self {
            Self { address }
        }
        pub fn run(self) {
            println!("Listening on {}", self.address);
            let listener = TcpListener::bind(&self.address).unwrap();
            loop {
                match listener.accept() {
                    Ok((mut stream, address)) => {
                        println!("Successful connection: {}", address);
                        let mut buffer = [0; 1024];
                        match stream.read(&mut buffer) {
                            Ok(size) => {
                                println!("Received a request: {}", String::from_utf8_lossy(&buffer))
                            },
                            Err(e) => println!("Failed to read from connection: {}", e),
                        };
                    },
                    Err(e) => {
                        println!("Failed to establish a connection: {}", e)
                    },
                };
            }
        }
    }
