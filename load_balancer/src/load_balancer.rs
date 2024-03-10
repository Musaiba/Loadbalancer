//! # Module LoadBalancer
//!
//! Ce module implémente un LoadBalancer simple utilisant l'algorithme Round Robin pour distribuer les demandes entrantes vers une pool de serveurs. 
//! L'algorithme Round Robin sélectionne séquentiellement chaque serveur, assurant ainsi une distribution équitable des demandes.
//! ## Fonctionnalités
//! - Configuration dynamique via la ligne de commande pour l'adresse d'écoute et les serveurs cibles.
//! - Distribution des demandes entrantes en utilisant l'algorithme Round Robin.
//! - Gestion des connexions TCP avec multithreading pour une meilleure performance et robustesse.


use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use log::{info, warn, error};

/// Structure principale du LoadBalancer.
///
/// Ce LoadBalancer distribue les requêtes entrantes entre une pool de serveurs
/// en utilisant l'algorithme Round Robin pour une distribution équitable.

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
    ///
    /// # Exemple
    ///
    /// ```
    /// let lb = LoadBalancer::new("127.0.0.1:8080", vec!["127.0.0.1:8000", "127.0.0.1:8001"]);
    /// ```
    pub fn new(bind_addr: &str, servers: Vec<String>) -> LoadBalancer {
        LoadBalancer {
            listener_config: bind_addr.to_string(), // Stocke la configuration pour créer un nouveau TcpListener
            servers,
        }
    }
   
    /// Démarre le LoadBalancer pour écouter sur l'adresse configurée et router les demandes vers les serveurs.
    ///
    /// Initialise le logger et écoute sur l'adresse configurée, distribuant les requêtes
    /// entrantes aux serveurs backend selon l'algorithme Round Robin.
    pub fn run(&self) {
        let listener = match TcpListener::bind(&self.listener_config) {
            Ok(listener) => {
                info!("LoadBalancer running on {}", &self.listener_config);
                listener
            },
            Err(e) => {
                error!("Failed to bind to {}: {}", &self.listener_config, e);
                return;
            }
        };

        let servers = Arc::new(Mutex::new(self.servers.clone()));

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let servers_clone = Arc::clone(&servers);
                    thread::spawn(move || {
                        Self::handle_client(stream, servers_clone);
                    });
                },
                Err(e) => error!("Failed to accept connection: {}", e),
            }
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
    fn handle_client(mut client_stream: TcpStream, servers: Arc<Mutex<Vec<String>>>) {
        let mut rr_index = 0;

        loop {
            let server_addr = {
                let servers = servers.lock().unwrap();
                servers[rr_index % servers.len()].clone() // Cycle through servers using modulo
            };

            match TcpStream::connect(&server_addr) {
                Ok(mut server_stream) => {
                    let mut buffer = [0; 512];
                    match client_stream.read(&mut buffer) {
                        Ok(bytes_read) => {
                            if let Err(e) = server_stream.write_all(&buffer[..bytes_read]) {
                                warn!("Failed to send data to server: {}", e);
                                continue;
                            }

                            let mut server_response = Vec::new();
                            if let Err(e) = server_stream.read_to_end(&mut server_response) {
                                warn!("Failed to read response from server: {}", e);
                                continue;
                            }

                            if let Err(e) = client_stream.write_all(&server_response) {
                                warn!("Failed to send server response to client: {}", e);
                            }
                        },
                        Err(e) => warn!("Failed to read data from client: {}", e),
                    }

                    break;
                },
                Err(_) => {
                    rr_index = (rr_index + 1) % servers.lock().unwrap().len();
                    continue;
                }
            }
        }
    }
}
