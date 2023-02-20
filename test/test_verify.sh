#!/usr/bin/env bash

set -e

echo -e "\033[34;40;3mTesting - identity --verify\033[0m"

mkdir -p ~/.config/

echo -e "\033[34;40;3m- No config file, creates default\033[0m"
set +e
identity --verify &> output.txt
check_result=$?
set -e

if [[ $check_result -ne 0 ]]; then
  cat output.txt
  echo -e "\033[91;40mWrong exit code, wanted 0 but got $check_result\033[0m"
  exit 1
fi

if ! stat ~/.config/identity.toml &> /dev/null ; then
  echo -e "\033[91;40mFile was not created\033[0m"
  exit 1
fi

# Create some garbage
cat > ~/.config/identity.toml << END
nothing = "valid"
END

echo -e "\033[34;40;3m- Rejects garbage\033[0m"
set +e
identity --verify &> output.txt
check_result=$?
grep -qe "Invalid config file content" output.txt
content_check_result=$?
set -e

if [[ $check_result -ne 1 ]]; then
  cat output.txt
  echo -e "\033[91;40mWrong exit code, wanted 1 but got $check_result\033[0m"
  exit 1
fi

if [[ $content_check_result -ne 0 ]]; then
  cat output.txt
  echo -e "\033[91;40mWrong content\033[0m"
  exit 1
fi

# Create an identity with duplicate ids
cat > ~/.config/identity.toml << END
version = "1.0"

[[identity]]
id = "tester"
email = "tester@example.com"

[[identity]]
id = "tester"
email = "tester@example.com"
END

echo -e "\033[34;40;3m- Rejects duplicate ids\033[0m"
set +e
identity --verify &> output.txt
check_result=$?
grep -qe "Identities must have a unique id" output.txt
content_check_result=$?
set -e

if [[ $check_result -ne 1 ]]; then
  cat output.txt
  echo -e "\033[91;40mWrong exit code, wanted 1 but got $check_result\033[0m"
  exit 1
fi

if [[ $content_check_result -ne 0 ]]; then
  cat output.txt
  echo -e "\033[91;40mWrong content\033[0m"
  exit 1
fi
