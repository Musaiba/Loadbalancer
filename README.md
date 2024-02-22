# Loadbalancer
**Ce projet est realisé par  Mariam CISSE et  Mike Arthur NYOGA - 4SI4 ESGI**

## **Description**

Ce projet consiste en mettre en place un equilibreur de charge en Rust . Ainsi tout au long de ce projet nous allons utiliser la logique de l'algorithme de Round Robin pour mettre en place notre LoadBalancer .

### **Prérequis**:
	
  1. Telecharger et Installer Rust  . 
  2. S'assurer d'avoir les Outils de developpement C++ de Visual Studio
  3. Python 3.x installé
  4. Bibliothèques Python requises : Flask.  

## **Installation des dependances** :
pour installer la  dépendances Python requises, exécutez la commande suivante  :
		```pip install Flask```

## **Utilisation**
  1.  Lancez les serveurs python via les commandes :
         
      ```python3 serveur1 ou serveur2 ```
        
        Ne pas oubliez de modifier les adresses Ip des serveurs 
      
   2.  Puis renseigner les adresses Ip de nos 2 serveurs sur notre LoadBalancer
           
           ```let lb = LoadBalancer::new("Votre_IP:8080", vec!["IP_Serveur1:8000", "IP_Serveur1:8001"].iter().map(|s| s.to_string()).collect())```
    
   3.  Puis faire un cargo run pour lancer l'execution de notre LoadBalancer


  ## **Documentation**

       Pour Consulter la documentation de notre projet faire un :

            ```cargo doc --open```