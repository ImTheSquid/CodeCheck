#!/usr/bin/env sh

# docker rm "$(docker stop "$(docker ps -a -q --filter ancestor=mongo:latest --format="{{.ID}}")")"

cargo leptos build

if [ $? -ne 0 ]; then
  exit 1
fi

cargo clippy

if [ $? -ne 0 ]; then
  exit 2
fi

MONGO_ID=$(docker run -p 27017:27017 -d mongo:latest --noauth)

docker buildx build --load -t "codecheck:latest" --build-arg MONGO_URI=mongodb://172.17.0.2:27017 --target webserver .

docker stop "$MONGO_ID"
docker rm "$MONGO_ID"
