# identity

A tool for managing your identity from the command line.

If you have more than one identity, such as work and personal, on the same machine then `identity` can help. It currently
supports Git and Cargo. The usage of each is described below. If you prefer, there is a set of functional tests provided with
the [source code](https://github.com/EphyraSoftware/identity/tree/main/test).

### Configuring identities

The `identity` CLI looks for a configuration file at `~/.config/identity.toml`. You can create or upgrade your identity file
using `identity --verify`.

```toml
version = "1.0"

[[identity]]
id = "personal"
email = "your-email@example.com"

[[identity.account]]
service = "git"
user = "my-username"
match_url = "https://github.com/my-username/*"
description = "my personal github"

[[identity.account]]
service = "cargo"
user = "my-username"
token = "a-token"

[[identity]]
id = "work"
email = "your-email@company.com"

[[identity.account]]
service = "git"
user = "company-username"
match_url = "https://github.com/company-username/*"
description = "my work github"
```

This configures two identities, `personal` and `work`. The `personal` identity has a GitHub account and a Cargo (crates.io) account.
The `work` identity just has a GitHub account.

### Git

Your username and email address are the first thing to keep separate. Git gives you several options for configuring these
and you can use `identity` to find out what's currently being used

```shell
identity whoami --service git
```

_Note_: You can omit the `--service git` argument and you will be prompted instead.

This will output something like

```text
user.name  = ThetaSinner
user.email = your-email@example.com
```

Knowing is one thing, but preventing commits with the wrong user information is the goal. While in a Git repository run

```shell
identity git install
```

which will install a pre-commit hook to verify your identity on every commit. The origin URL will be matched against the identities
in your configuration file, and if the current Git identity isn't the same as the matched on, the commit will be prevented.

If you're already using pre-commit hooks then you can add the check manually by putting `identity git hook --pre-commit` into `.git/hooks/pre-commit`.

To check that a repository is currently configured to use `identity` and that the identity is configured correctly you can run

```shell
identity git --check
```

### Cargo

Cargo doesn't have accounts in the same sense. You have a token which can be used to publish crates and this is your identity
as far as `identity` is concerned.

To check your current identity, use

```shell
identity whoami --service cargo
```

Or to switch to a new identity

```shell
identity switch --service cargo
```

which will prompt you for an identity to switch to.

### Other service

I plan to add more services as I need them, feel free to open an issue or a PR on GitHub if you'd like something else supported.
