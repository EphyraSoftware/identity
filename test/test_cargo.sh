#!/usr/bin/env bash

set -e

echo -e "\033[34;40;3mTesting - identity git\033[0m"

mkdir -p ~/.cargo/
touch ~/.config/credentials

mkdir -p ~/.config/
touch ~/.config/identity.toml

cat > ~/.config/identity.toml << END
version = "1.0"

[[identity]]
id = "tester"
email = "tester@example.com"

[[identity.account]]
service = "cargo"
token = "abcd"

[[identity]]
id = "tinkerer"
email = "tinkerer@example.com"

[[identity.account]]
service = "cargo"
token = "efgh"

END

echo -e "\033[34;40;3m- Fails to switch if no credentials are configured\033[0m"
set +e
identity switch --service cargo --identity tester &> output.txt
check_result=$?
grep -qe "Cargo credentials file not found" output.txt
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

cat > ~/.cargo/credentials << END
[registry]
token = "efgh"

END

echo -e "\033[34;40;3m- Fails to switch if the currently configured token is not known\033[0m"
set +e
identity switch --service cargo --identity tester &> output.txt
check_result=$?
grep -qe "abcd" ~/.cargo/credentials
content_check_result=$?
set -e

if [[ $check_result -ne 0 ]]; then
  cat output.txt
  echo -e "\033[91;40mWrong exit code, wanted 0 but got $check_result\033[0m"
  exit 1
fi

if [[ $content_check_result -ne 0 ]]; then
  cat output.txt
  echo -e "\033[91;40mWrong content\033[0m"
  exit 1
fi

echo -e "\033[34;40;3m- Gets current identity based on current token\033[0m"
set +e
identity whoami --service cargo  &> output.txt
check_result=$?
grep -qe "tester" output.txt
content_check_result=$?
set -e

if [[ $check_result -ne 0 ]]; then
  cat output.txt
  echo -e "\033[91;40mWrong exit code, wanted 0 but got $check_result\033[0m"
  exit 1
fi

if [[ $content_check_result -ne 0 ]]; then
  cat output.txt
  echo -e "\033[91;40mWrong content\033[0m"
  exit 1
fi

identity switch --service cargo --identity tinkerer

echo -e "\033[34;40;3m- Gets current identity based on current token after a switch\033[0m"
set +e
identity whoami --service cargo  &> output.txt
check_result=$?
grep -qe "tinkerer" output.txt
content_check_result=$?
set -e

if [[ $check_result -ne 0 ]]; then
  cat output.txt
  echo -e "\033[91;40mWrong exit code, wanted 0 but got $check_result\033[0m"
  exit 1
fi

if [[ $content_check_result -ne 0 ]]; then
  cat output.txt
  echo -e "\033[91;40mWrong content\033[0m"
  exit 1
fi
