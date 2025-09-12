#!/bin/sh
cd "$(dirname "$0")"
docker build --platform linux/amd64 -f docker/Dockerfile.neural -t codecheck/train:latest .
