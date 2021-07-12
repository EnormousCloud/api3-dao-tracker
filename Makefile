build:
	docker build -t api3tracker .

# for the case if the local instance is running (geth --syncmode=light)
run:
	chmod 0666 ${HOME}/.ethereum/geth.ipc
	docker run -d \
		--name api3tracker-mainnet \
		-p 8000:8000 \
		--mount type=bind,source=${HOME}/.ethereum/geth.ipc,target=/ethereum/geth.ipc \
		--mount type=bind,source=$(shell pwd)/server/.env.mainnet,target=/usr/src/app/.env \
		-e RPC_ENDPOINT=/ethereum/geth.ipc \
		api3tracker
	docker logs -f api3tracker-mainnet

    
# for the case if the local instance is running (geth --rinkeby --syncmode=light)
run-rinkeby:
	chmod 0666 ${HOME}/.ethereum/rinkeby/geth.ipc
	docker run -d \
		--name api3tracker-rinkeby \
		-p 8000:8000 \
		--mount type=bind,source=${HOME}/.ethereum/rinkeby/geth.ipc,target=/ethereum/geth.ipc \
		--mount type=bind,source=$(shell pwd)/server/.env.rinkeby,target=/usr/src/app/.env \
		-e RPC_ENDPOINT=/ethereum/geth.ipc \
		api3tracker
	docker logs -f api3tracker-rinkeby

stop:
	docker rm -f api3tracker-mainnet
	docker rm -f api3tracker-rinkeby

