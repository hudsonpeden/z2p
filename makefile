init:
	./scripts/init_db.sh
init_no_docker:
	SKIP_DOCKER=true ./scripts/init_db.sh    
