# LAB-sql-injection


Start docker
```bash
docker build -t client-mysql-image .
docker run -d --name client-mysql-container -p 3306:3306 client-mysql-image
```


Vous pouvez récupérer les différends utilisateur en fournissant l'id.
http://127.0.0.1:8080/user/1

Le but de ce lab et de parvenir à récupéré tout les utilisateurs grâce à une seul requète get.