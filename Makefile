.PHONY: init mysql.start mysql.stop mysql.clean

init:
	cargo install diesel_cli
	diesel setup

mysql.start:
	docker run --name aworld-data -e MYSQL_ROOT_PASSWORD=ourworld -p 3306:3306 -d mysql:5.7.27

mysql.stop:
	docker stop aworld-data

mysql.clean:
	docker rm aworld-data
