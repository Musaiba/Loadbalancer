//! # Module LoadBalancer
//!
//! Ce module implémente un LoadBalancer simple utilisant l'algorithme Round Robin pour distribuer les demandes entrantes vers une pool de serveurs. 
//! L'algorithme Round Robin sélectionne séquentiellement chaque serveur, assurant ainsi une distribution équitable des demandes.

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;

/// Représente le LoadBalancer avec sa configuration d'écoute et les serveurs disponibles.
pub struct LoadBalancer {
    /// Configuration pour écouter les connexions TCP. Exemple : "127.0.0.1:8080".
    listener_config: String, 
    /// Liste des adresses des serveurs (ex: "192.168.1.1:80") vers lesquels router les demandes.
    servers: Vec<String>,
}

impl LoadBalancer {
    /// Crée une nouvelle instance de `LoadBalancer`.
    ///
    /// # Arguments
    /// 
    /// * `bind_addr` - L'adresse sur laquelle le LoadBalancer doit écouter les connexions entrantes.
    /// * `servers` - Les serveurs disponibles pour router les demandes.
    pub fn new(bind_addr: &str, servers: Vec<String>) -> LoadBalancer {
        LoadBalancer {
            listener_config: bind_addr.to_string(), // Stocke la configuration pour créer un nouveau TcpListener
            servers,
        }
    }
   
    /// Démarre le LoadBalancer pour écouter sur l'adresse configurée et router les demandes vers les serveurs.
    ///
    /// Utilise `TcpListener` pour accepter les connexions entrantes et les distribue à l'aide de threads séparés pour chaque connexion.
    pub fn run(&self) {
        let listener = TcpListener::bind(&self.listener_config).expect("Failed to bind address");

        let servers = Arc::new(Mutex::new(self.servers.clone()));

        for stream in listener.incoming() {
            let stream = stream.expect("Failed to accept connection");
            let servers_clone = Arc::clone(&servers);

            thread::spawn(move || {
                let  lb = LoadBalancer {
                    listener_config: String::new(), 
                    servers: Vec::new(), 
                };
                lb.handle_client(stream, servers_clone);
            });
        }
    }
    /// Traite une connexion client, en routant la demande vers un serveur sélectionné selon l'algorithme Round Robin.
    ///
    /// # Arguments
    ///
    /// * `client_stream` - Le flux TCP associé à la connexion client.
    /// * `servers` - Une référence partagée à la liste des serveurs.
    ///
    /// La méthode établit une connexion au serveur suivant dans la liste, transfère la demande du client, puis lit et retourne la réponse du serveur au client.
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
    /// Permet de cloner une instance de `LoadBalancer`, notamment sa configuration et la liste des serveurs.
    fn clone(&self) -> Self {
        LoadBalancer {
            listener_config: self.listener_config.clone(),
            servers: self.servers.clone(),
        }
    }
}

