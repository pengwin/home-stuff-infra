.PHONY: deploy lambda-zip

lambda-zip:
ifeq ($(OS),Windows_NT)
	wsl /bin/bash -c 'source $$HOME/.cargo/env && make -C lambdas/rust-lambda zip'
else
	make -C lambdas/rust-lambda zip
endif

run-local-stack:
	docker compose up -d
	
deploy-local-stack: lambda-zip
	cdktf deploy home-stuff-local --auto-approve