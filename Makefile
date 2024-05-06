start:
	docker compose up --build -d

stop:
	docker compose stop

restart: stop start

destroy:
	docker compose down

rebuild: stop destroy start

ps:
	docker compose ps

# If you have the tree command installed
tree:
	tree -I 'target'