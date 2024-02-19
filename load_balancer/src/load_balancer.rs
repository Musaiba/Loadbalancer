use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct LoadBalancer {
    listener_config: String, // Suppose que la configuration de TcpListener est une chaîne pour l'exemple
    servers: Vec<String>,
}

impl LoadBalancer {
    pub fn new(bind_addr: &str, servers: Vec<String>) -> LoadBalancer {
        LoadBalancer {
            listener_config: bind_addr.to_string(), // Stocke la configuration pour créer un nouveau TcpListener
            servers,
        }
    }

    pub fn run(&self) {
        let listener = TcpListener::bind(&self.listener_config).expect("Failed to bind address");

        let servers = Arc::new(Mutex::new(self.servers.clone()));

        for stream in listener.incoming() {
            let stream = stream.expect("Failed to accept connection");
            let servers_clone = Arc::clone(&servers);

            thread::spawn(move || {
                let  lb = LoadBalancer {
                    listener_config: String::new(), // Configuration factice car TcpListener n'est pas clonable
                    servers: Vec::new(), // Vecteur vide car nous n'avons pas besoin des serveurs ici
                };
                lb.handle_client(stream, servers_clone);
            });
        }
    }

    fn handle_client(&self, mut client_stream: TcpStream, servers: Arc<Mutex<Vec<String>>>) {
        let mut rr_index = 0;

        loop {
            let server_addr = {
                let servers = servers.lock().unwrap();
                servers[rr_index].clone()
            };

            match TcpStream::connect(&server_addr) {
                Ok(mut server_stream) => {
                    let mut buffer = [0; 512];
                    let bytes_read = client_stream.read(&mut buffer).unwrap();
                    server_stream.write_all(&buffer[..bytes_read]).unwrap();

                    let mut server_response = Vec::new();
                    server_stream.read_to_end(&mut server_response).unwrap();
                    client_stream.write_all(&server_response).unwrap();

                    break;
                }
                Err(_) => {
                    rr_index = (rr_index + 1) % servers.lock().unwrap().len();
                    continue;
                }
            }
        }
    }
}

impl Clone for LoadBalancer {
    fn clone(&self) -> Self {
        LoadBalancer {
            listener_config: self.listener_config.clone(),
            servers: self.servers.clone(),
        }
    }
}

