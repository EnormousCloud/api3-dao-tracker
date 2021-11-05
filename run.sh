set -xe
export STATIC_DIR=$(pwd)/client/dist

[[ "$1" == "client" ]] && {
    shift
    cd client
    trunk build
    cd -
}

[[ "$1" == "client-mainnet" ]] && {
    shift
    cd client
    trunk build --public-url=/dao/api3/tracker/ 
    cd -
}

[[ "$1" == "client-rinkeby" ]] && {
    shift
    cd client
    trunk build --public-url=/dao/api3/tracker-rinkeby/ 
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

[[ "$1" == "copy-cache-mainnet" ]] && {
    shift
    ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null root@enormous.cloud 'cd /opt; tar zcf cache.tar.gz ./cache'
    scp -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null root@enormous.cloud:/opt/cache.tar.gz ./
    tar xvzf ./cache.tar.gz 
    mv -f cache/* ./.cache/
    rm -rf cache.tar.gz cache/ || true
}
    
[[ "$1" == "copy-cache-rinkeby" ]] && {
    shift
    ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null root@rinkeby.enormous.cloud 'cd /opt; tar zcf cache.tar.gz ./cache'
    scp -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null root@rinkeby.enormous.cloud:/opt/cache.tar.gz ./
    tar xvzf ./cache.tar.gz 
    mv -f cache/* ./.cache/
    rm -rf cache.tar.gz cache/ || true
}
    
[[ "$1" == "run-local-mainnet" ]] && {
    shift
    export CACHE_DIR=$(pwd)/.cache
    mkdir -p $CACHE_DIR

	cd server
    cp -f .env.mainnet .env
    LOG_LEVEL=api3tracker=debug,info \
    RUST_BACKTRACE=full \
	RPC_ENDPOINT=http://20.56.124.174/rpc/mki2jdimfm7o-j7d9my6frt2ar0w5gvow \
        cargo run --release -- $@
}

[[ "$1" == "run-local-rinkeby" ]] && {
    shift
    export CACHE_DIR=$(pwd)/.cache
    mkdir -p $CACHE_DIR

	cd server
    cp -f .env.rinkeby .env
    LOG_LEVEL=api3tracker=debug,info \
    RUST_BACKTRACE=full \
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
    #export SSH_HOST="root@mainnet.enormous.cloud"
    export SSH_HOST="root@enormous.cloud"
    docker save api3tracker | bzip2 | ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null  $SSH_HOST 'bunzip2 | docker load'
    ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null $SSH_HOST 'cd /opt/api3tracker-mainnet; docker rm -f api3tracker-mainnet; docker-compose up -d'
}


[[ "$1" == "cache-mainnet" ]] && {
    export SSH_HOST="root@enormous.cloud"
    ssh $SSH_HOST 'docker exec -i api3tracker-mainnet /usr/src/app/api3tracker --dump unknown'
}

[[ "$1" == "publish-rinkeby" ]] && {
    export SSH_HOST="root@rinkeby.enormous.cloud"
    scp -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null "$(pwd)"/server/.env.rinkeby $SSH_HOST:/opt/.env.rinkeby
    docker save api3tracker | bzip2 | ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null$SSH_HOST 'bunzip2 | docker load'
    ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null$SSH_HOST '/opt/api3tracker-rinkeby.sh'
}

[[ "$1" == "cache-rinkeby" ]] && {
    export SSH_HOST="root@rinkeby.enormous.cloud"
    ssh $SSH_HOST 'docker exec -i api3tracker-rinkeby /usr/src/app/api3tracker --dump unknown'
}
