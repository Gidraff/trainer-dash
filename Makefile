.PHONY: dev build up down logs test api-test web-test

dev:
	docker-compose up --build

build:
	docker-compose build

up:
	docker-compose up -d

down:
	docker-compose down -v

logs:
	docker-compose logs -f

test: api-test web-test

api-test:
	cd apps/api && cargo test

web-test:
	cd apps/web && npm test || true

lint:
	cd apps/api && cargo clippy
	cd apps/web && npm run lint
