//!##  Introduction detaille 

//! Dans ce module nous allons implementer la logique de notre LoadBalancer
//!
//! l'algorithme que nous avons decider d'implementer pour gerer le LoadBalancing est l'algorithme de Round Robin 
//!
//! Il s'agit d'un algorithme qui choisi sequentiellement chaque serveur dans une pool de serveur
//! elle se base sur un indice de rotation qui se charge de determiner quelle source utiliser pour chaque nouvelle demande  

//!## Structure 
//! Nous allons commencer par definir la structure de notre LoadBalancer 

//! ```
//! pub struct LoadBalancer { 
//!    listener_config: String, 
//!     servers: Vec<String>, 
//!                   }
//! ```
//! Cette structure  comprend  2 parametres a savoir un vecteur qui va stocker les serveurs auxquels notre LoadBalancer sera liés , 
//! puis un autre parametre listener_config  qui stocke la configuration du TcpListener associer a notre LoadBalancer 
//!
//! TcpListener ici represente une structure qui permet de creer un point liaison a partir duquel nos connexions Tcp entrantes peuvent etre gerer , 
//! en gros il nous permet de creer un serveur Tcp basique 

//!## Methode de la structure LoadBalancer

//! Par la suite nous allons implementer les differentes methodes de notre LoadBalancer a savoir :
//! ```
//!  pub fn new(bind_addr: &str, servers: Vec<String>)
//!```
//! qui nous permettra de creer une nouvelle instance de notre LoadBalancer
//! ```
//!  pub fn run(&self)
//!```
//! Methode dans laquelle on va implementer la logique d'execution de notre LoadBalancer 
//!# Analyse de la methode run()
//!  Deux  Objets sont declare dans cette methode , a savoir 
//! ```
//! let listener = TcpListener::bind(&self.listener_config).expect("Failed to bind address")
//!```
//! qui se charge d'ecouter  sur une adresse specifique au travers de la liaison faite a partir de la methode Bind() de la structure TcpListener
//! Puis nous avons l'objet 
//! ```
//!let servers = Arc::new(Mutex::new(self.servers.clone()));
//!```
//! qui va stocker une copie des serveurs , permettant ainsi au differents Threads d'acceder a une copie de nos serveur , sans toutefois  modifier la structure principale . 
//! Nos objets crees, nous allons par la suite ecouter chaque connection entrante  et nous allons  boucler sur chacune d'entres elles , afin de les traiter individuelement .
//! Ainsi a l'aide de la boucle for et de la methode incoming() nous allons iterer sur chaque connection entrantes
//! ```
//! for stream in listener.incoming()
//!```
//!plus precisement  la methode incomming() de la structure TcpStream , creer un iterateur , en attente de connexion ,  qui renvoi un valeur correspondant a la prochaine connexion entrante
//! A l'aide de la methode  except() on recupere cette connection entrante , et on pourra generer par la meme occasion une exception dans le cas l'acceptation ne se deroule pas comme prevu
//!
//! Par la suite nous allons creer un nouveau Threads qui se chargera de gerer chaque nouvelles  connection entrante , selon le modele de gestion definit dans la fonction handle_client()
//! ```
//!  thread::spawn()
//!```
//! Nous avons opter pour une gestion des connection entrante dans un Threads differentes , du Threads principale , pour eviter les probleme lies a la duree de vie 
//!# Analyse de la methode handle_client()
//! Cette methode se charge de gerer les differentes connexions entrantes  des utilisateurs et de determiner vers quels serveurs diriger les differentes requetes des clients 
//! 
//! Ainsi on commence par initialiser a 0 la variable mutable rr_index , cette variable nous permettra de suivre l'index des serveurs a utiliser lors de l'achemeniment des requete 
//! par notre LoadBalancer
//! Puis dans une boucle infinie   on recupere , le prochain serveur vers lequel nos requetes seront acheminés
//!  en s'assurant que notre Thread ai un accès exclussif a notre liste de serveur 
//! ```
//!   let server_addr = {  let servers = servers.lock().unwrap(); servers[rr_index].clone() };         
//!```
//!  Par la suite a partir d'un match() on realise une tentative de connexion vers le serveur en question en utilisant la methode connect() de la structure TcpStream
//!
//! Cette methode retourne un Result<TcpStream,Error> qui indique si la connexion a reussi ou pas. Dans le cas ou la 
//! connexion a reussi [Ok(mut server_stream)]  on definit les differents operations liées au  relais de données .
//! Ainsi dans un premier temps on definit un tampon de 512 octects pour stocker les donnees du client , puis on lit les données provenant du client (client_stream.read()) et on les stocke dans le buffer
//! ```
//!   let mut buffer = [0; 512];
//!   let bytes_read = client_stream.read(&mut buffer).unwrap();       
//!```
//! Ici bytes_read va stocker le nombre d'octect lu
//!
//! Puis ces données lu depuis le client sont ecrite directement sur le serveur server_stream
//! ```
//!   server_stream.write_all(&buffer[..bytes_read]).unwrap();         
//!```
//! Par la suite on crée un vecteur vide qui va contenir les reponses du serveur . 
//!
//! On lit la reponse complete du serveur dans  le vecteur , puis on ecrit cette reponse sur client_stream qui represente notre connexion client 
//! ```
//!   let mut server_response = Vec::new();
//!    server_stream.read_to_end(&mut server_response).unwrap();
//!     client_stream.write_all(&server_response).unwrap();      
//!```
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
                    listener_config: String::new(), 
                    servers: Vec::new(), 
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

