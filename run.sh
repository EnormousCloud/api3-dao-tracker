set -xe
export STATIC_DIR=$(pwd)/client/dist

[[ "$1" == "client" ]] && {
    shift
    cd client
    trunk-wcrbrm build
    cd -
}

[[ "$1" == "client-mainnet" ]] && {
    shift
    cd client
    trunk-wcrbrm build --public-url=/dao/api3/tracker/ 
    cd -
}

[[ "$1" == "client-rinkeby" ]] && {
    shift
    cd client
    trunk-wcrbrm build --public-url=/dao/api3/tracker-rinkeby/ 
    cd -
}

[[ "$1" == "build" ]] && {
    shift
    docker build -t api3tracker .
}

[[ "$1" == "stop" ]] && {
    shift
	docker rm -f api3tracker-mainnet
}
    
[[ "$1" == "run-local-mainnet" ]] && {
    shift
    export CACHE_DIR=$(pwd)/.cache
    mkdir -p $CACHE_DIR

	cd server
    cp -f .env.mainnet .env
    LOG_LEVEL=api3tracker=debug,info \
    RUST_BACKTRACE=1 \
	RPC_ENDPOINT=$HOME/.ethereum/geth.ipc \
        cargo run --release -- $@
}

[[ "$1" == "run-local-rinkeby" ]] && {
    shift
    export CACHE_DIR=$(pwd)/.cache
    mkdir -p $CACHE_DIR

	cd server
    cp -f .env.rinkeby .env
    LOG_LEVEL=api3tracker=debug,info \
    RUST_BACKTRACE=1 \
	RPC_ENDPOINT=$HOME/.ethereum/rinkeby/geth.ipc \
        cargo run --release -- $@
}

[[ "$1" == "run" ]] && {
    shift
    export CACHE_DIR=$(pwd)/.cache
    mkdir -p $CACHE_DIR

    chmod 0666 $HOME/.ethereum/geth.ipc
    docker run -d \
        --name api3tracker-mainnet \
        -p 8000:8000 \
        -e RPC_ENDPOINT=/ethereum/geth.ipc \
        -v $CACHE_DIR:/cache \
        --mount type=bind,source=$HOME/.ethereum/geth.ipc,target=/ethereum/geth.ipc \
        --mount type=bind,source="$(pwd)"/server/.env.mainnet,target=/usr/src/app/.env \
        api3tracker
    docker logs -f api3tracker-mainnet
}

[[ "$1" == "run-rinkeby" ]] && {
    shift
    export CACHE_DIR=$(pwd)/.cache
    mkdir -p $CACHE_DIR

	chmod 0666 ${HOME}/.ethereum/rinkeby/geth.ipc
	docker run -d \
		--name api3tracker-rinkeby \
		-p 8000:8000 \
		-e RPC_ENDPOINT=/ethereum/geth.ipc \
        -v $CACHE_DIR:/cache \
		--mount type=bind,source=${HOME}/.ethereum/rinkeby/geth.ipc,target=/ethereum/geth.ipc \
		--mount type=bind,source=$(pwd)/server/.env.rinkeby,target=/usr/src/app/.env \
		api3tracker
	docker logs -f api3tracker-rinkeby
}

[[ "$1" == "publish-mainnet" ]] && {
    export SSH_HOST="root@enormous.cloud"
    docker save api3tracker | bzip2 | ssh $SSH_HOST 'bunzip2 | docker load'
    ssh $SSH_HOST 'cd /opt/api3tracker-mainnet; docker rm -f api3tracker-mainnet; docker-compose up -d'
}

[[ "$1" == "publish-rinkeby" ]] && {
    export SSH_HOST="root@rinkeby.enormous.cloud"
    scp "$(pwd)"/server/.env.rinkeby $SSH_HOST:/opt/.env.rinkeby
    docker save api3tracker | bzip2 | ssh $SSH_HOST 'bunzip2 | docker load'
    ssh $SSH_HOST '/opt/api3tracker-rinkeby.sh'
}