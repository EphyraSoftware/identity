#!/usr/bin/env bash

docker build -t identity-test .
docker run identity-test
