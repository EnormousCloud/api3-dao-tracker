build:
	docker build -t api3tracker .

run:
	chmod 0666 ${HOME}/.ethereum/geth.ipc
	docker run -d \
		--name api3tracker-mainnet \
		-p 8000:8000 \
		--mount type=bind,source=${HOME}/.ethereum/geth.ipc,target=/ethereum/geth.ipc \
		--mount type=bind,source=$(shell pwd)/server/.env.mainnet,target=/usr/src/app/.env \
		-e RPC_ENDPOINT=/ethereum/geth.ipc \
		api3tracker

