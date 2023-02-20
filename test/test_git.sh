#!/usr/bin/env bash

set -e

echo -e "\033[34;40;3mTesting - identity git\033[0m"

mkdir -p ~/.config/
touch ~/.config/identity.toml

cat > ~/.config/identity.toml << END
version = "1.0"

[[identity]]
id = "tester"
email = "tester@example.com"

[[identity.account]]
service = "git"
user = "tester"
match_url = "https://github.com/tester/*"

[[identity]]
id = "tinkerer"
email = "tinkerer@example.com"

[[identity.account]]
service = "git"
user = "tinkerer"
match_url = "https://github.com/tinkerer/*"

[[identity]]
id = "duplicator1"
email = "duplicator1@example.com"

[[identity.account]]
service = "git"
user = "duplicator1"
match_url = "https://github.com/duplicator/*"

[[identity]]
id = "duplicator2"
email = "duplicator2@example.com"

[[identity.account]]
service = "git"
user = "duplicator2"
match_url = "https://github.com/duplicator/*"

[[identity]]
id = "bad-credentials"
email = "bad-credentials@example.com"

[[identity.account]]
service = "git"
user = "bad-credentials"
match_url = "https://github.com/bad-credentials/*"
token = "the-wrong-token"

END

git config --global init.defaultBranch main

trap "popd > /dev/null" EXIT
pushd "$(mktemp -d)" > /dev/null || exit

git init --quiet tester_project && cd tester_project || exit
git config user.name "tester"
git config user.email "tester@example.com"
git remote add origin https://github.com/tester/project.git

echo -e "\033[34;40;3m- Missing credentials helper\033[0m"
set +e
identity git --check &> output.txt
check_result=$?
grep -qe "No credentials helper configured" output.txt
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

# Enable any credential helper
git config --global credential.helper cache

echo -e "\033[34;40;3m- Pre-commit hook not installed\033[0m"
set +e
identity git --check &> output.txt
check_result=$?
grep -qe "Pre-commit hook not found" output.txt
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

echo -e "#!/usr/local/env bash\n" > .git/hooks/pre-commit

echo -e "\033[34;40;3m- Pre-commit hook missing identity check\033[0m"
set +e
identity git --check &> output.txt
check_result=$?
grep -qe "Pre-commit hook does not contain an identity check" output.txt
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

echo -e "\033[34;40;3m- Will not overwrite existing pre-commit hook\033[0m"
set +e
identity git --install &> output.txt
check_result=$?
grep -qe "Pre-commit hook already exists, run with --force to overwrite" output.txt
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

echo -e "\033[34;40;3m- Force install pre-commit hook\033[0m"
set +e
identity git --install --force &> output.txt
check_result=$?
grep -qe "Hook installed at" output.txt
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

echo -e "\033[34;40;3m- Wrong username\033[0m"
set +e
git config user.name "not-the-tester"
identity git --check &> output.txt
check_result=$?
grep -qe "Username mismatch - expected=tester != actual=not-the-tester" output.txt
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

# Reset the username
git config user.name "tester"

echo -e "\033[34;40;3m- Wrong email\033[0m"
set +e
git config user.email "not-the-tester@example.com"
identity git --check &> output.txt
check_result=$?
grep -qe "Email mismatch - expected=tester@example.com != actual=not-the-tester@example.com" output.txt
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

# Reset the email
git config user.email "tester@example.com"

echo -e "\033[34;40;3m- Check pass\033[0m"
set +e
identity git --check &> output.txt
check_result=$?
grep -qe "Everything looks good!" output.txt
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

echo -e "\033[34;40;3m- Check on commit\033[0m"
set +e
git config user.email "not-the-tester@example.com"
echo "a change" > test.txt
git commit -am "initial commit" &> output.txt
check_result=$?
grep -qe "Email mismatch - expected=tester@example.com != actual=not-the-tester@example.com" output.txt
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

# Reset the email
git config user.email "tester@example.com"

cd ..
git init --quiet tinkerer_project && cd tinkerer_project || exit
git config user.name "tinkerer"
git config user.email "tinkerer@example.com"
git remote add origin https://github.com/tinkerer/project.git
identity git --install > /dev/null

echo -e "\033[34;40;3m- Check alternate identity\033[0m"
set +e
identity git --check &> output.txt
check_result=$?
grep -qe "Everything looks good!" output.txt
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

cd ..
git init --quiet duplicator_project && cd duplicator_project || exit
git config user.name "duplicator1"
git config user.email "duplicator1@example.com"
git remote add origin https://github.com/duplicator/project.git
identity git --install > /dev/null

echo -e "\033[34;40;3m- Reject duplicate matched identities\033[0m"
set +e
identity git --check &> output.txt
check_result=$?
grep -qe "Multiple identities found for URL" output.txt
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

cd ..
git init --quiet credentials_project && cd credentials_project || exit
git config user.name "bad-credentials"
git config user.email "bad-credentials@example.com"
git remote add origin https://github.com/bad-credentials/credentials_project.git
identity git --install > /dev/null

echo -e "protocol=https\nhost=github.com\nusername=bad-credentials\npassword=the-right-token\n\n" | git credential approve

echo -e "\033[34;40;3m- Reject mismatched credentials\033[0m"
set +e
identity git --check &> output.txt
check_result=$?
grep -qe "The token in your identity.toml does not match the token Git is configured to use" output.txt
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

sed -i 's/the-wrong-token/the-right-token/' ~/.config/identity.toml

echo -e "\033[34;40;3m- Accepts correct credentials\033[0m"
set +e
identity git --check &> output.txt
check_result=$?
grep -qe "Everything looks good!" output.txt
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
