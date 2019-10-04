.PHONY: mysql.start mysql.stop mysql.clean

all: mysql.start

init:
	@if [ which diesel > /dev/null 2>&1 ];then\
		cargo install diesel_cli;\
	fi
	@diesel setup
	@echo "Diesel has been installed!"

mysql.start:
	docker run --name aworld-data -e MYSQL_ROOT_PASSWORD=ourworld -p 43306:3306 -d mysql:5.7.27

mysql.stop:
	docker stop aworld-data

mysql.clean:
	docker kill aworld-data
	docker rm aworld-data
