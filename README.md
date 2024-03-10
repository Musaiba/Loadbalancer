# Loadbalancer
**Ce projet est realisé par  Mariam CISSE et  Mike Arthur NYOGA - 4SI4 ESGI**

## **Description**

Ce projet consiste en mettre en place un equilibreur de charge en Rust . Ainsi tout au long de ce projet nous allons utiliser la logique de l'algorithme de Round Robin pour mettre en place notre LoadBalancer .

## Fonctionnalités

- Distribution des requêtes entrantes en utilisant l'algorithme Round Robin.
- Configuration dynamique des adresses des serveurs via la ligne de commande.
- Journaux d'événements détaillés pour le suivi des opérations et le débogage.

## **Prérequis**:
	
  1. Telecharger et Installer Rust à partir de [la page officielle Rust](https://www.rust-lang.org/tools/install). . 
  2. S'assurer d'avoir les Outils de developpement C++ de Visual Studio
  3. Python 3.x installé
  4. Bibliothèques Python requises : Flask.  

## **Installation des dependances** :
Pour installer la  dépendances Python requises, exécutez la commande suivante  :
		```pip install Flask```

## **Utilisation**
  1.  Lancez les serveurs python via les commandes :
         
      ```python3 server1.py```
      ```python3 server2.py ```
        
        Ne pas oubliez de modifier les adresses Ip des serveurs 
      
   2.  Puis renseigner les adresses Ip de nos 2 serveurs sur notre LoadBalancer
           
           ```let lb = LoadBalancer::new("Votre_IP:8080", vec!["IP_Serveur1:8000", "IP_Serveur1:8001"].iter().map(|s| s.to_string()).collect())```
   4. Faire ```cargo test``` pour lancer les tests et tester le loadbalancer
    
   3. Faire ```cargo run``` pour lancer l'execution de notre LoadBalancer
   
   
   4. Acceder via votre navigateur à ```http://Votre_IP:8080``` vous serrez redirigé vers le serveur disponible grâce au Loadbalancer


## **Documentation**
 Pour Consulter la documentation de notre projet faire un :

            ```cargo doc --open```
            
## **Remerciements**
 Nous tenons à remercier tous ceux qui ont contribué à la réalisation de ce projet, y compris les enseignants de l'ESGI, les ressources en ligne, et la communauté Rust pour leur support et leurs conseils précieux.
