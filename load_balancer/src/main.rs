/// Point d'entrée principal du LoadBalancer.
///
/// Configure et lance le LoadBalancer avec les adresses spécifiées pour écouter
/// et les serveurs backend. Utilise `clap` pour le parsing des arguments de ligne de commande.

mod load_balancer;

use clap::{App, Arg};
use load_balancer::LoadBalancer;
use log::info;

fn main() {
    // Code d'initialisation et de lancement
    env_logger::init(); // Initialise le système de logging

    let default_bind_addr = "192.168.1.137:8080";
    let default_servers = vec!["192.168.1.137:8000", "192.168.1.137:8001"];
    
    // Créez une chaîne contenant les adresses des serveurs, séparées par des virgules
    let server_list = default_servers.join(",");

    let matches = App::new("LoadBalancer")
        .version("0.1.0")
        .author("Mariam CISSE & Mike Arthur NYOGA")
        .about("Distribue les demandes entrantes en utilisant l'algorithme Round Robin")
        .arg(Arg::with_name("bind_addr")
             .short("b")
             .long("bind")
             .value_name("BIND_ADDR")
             .help("Définit l'adresse d'écoute pour le LoadBalancer")
             .takes_value(true)
             .default_value(default_bind_addr))
        .arg(Arg::with_name("servers")
             .short("s")
             .long("servers")
             .value_name("SERVERS")
             .help("Liste des adresses des serveurs à équilibrer, séparées par une virgule")
             .takes_value(true)
             .default_value(&server_list)) // Utilisez la référence à la chaîne créée précédemment
        .get_matches();

    let bind_addr = matches.value_of("bind_addr").unwrap_or(default_bind_addr);
    let servers: Vec<String> = matches.value_of("servers")
        .map(|s| s.split(',').map(String::from).collect())
        .unwrap_or_else(|| default_servers.iter().map(|&s| s.into()).collect());

    let lb = LoadBalancer::new(bind_addr, servers);
    info!("Démarrage du LoadBalancer à l'adresse {}", bind_addr);
    lb.run();
}

