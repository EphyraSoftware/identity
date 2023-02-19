#!/usr/bin/env bash

set -e

docker build -t identity-test .
docker run identity-test
