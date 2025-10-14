#!/bin/sh
cd "$(dirname "$0")"
docker build --platform linux/amd64 -f docker/Dockerfile.neural -t codecheck/train:latest .

if [ "${1:no}" = "push" ]; then
    docker image tag codecheck/train:latest registry.jackhogan.me/codecheck/train:latest
    docker image push registry.jackhogan.me/codecheck/train:latest
fi
