#!/usr/bin/env bash

set -e

echo -e "\033[34;40;3mStarting tests\033[0m"

./test_verify.sh
./test_git.sh

echo -e "\033[34;40;3mFinished testing\033[0m"
