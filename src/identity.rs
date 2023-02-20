use crate::config::{AccountConfig, IdentityConfig};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Identity<'a> {
    identity_config: &'a IdentityConfig,
    account_config: &'a AccountConfig,
}

impl<'a> Identity<'a> {
    pub fn from(identity_config: &'a IdentityConfig, account_config: &'a AccountConfig) -> Self {
        Identity {
            identity_config,
            account_config,
        }
    }

    pub fn id(&self) -> &str {
        self.identity_config.id.as_str()
    }

    pub fn email(&self) -> Option<&String> {
        self.identity_config.email.as_ref()
    }

    pub fn user(&self) -> Option<&String> {
        self.account_config
            .user
            .as_ref()
            .or_else(|| self.identity_config.user.as_ref())
    }

    pub fn match_url(&self) -> Option<&String> {
        self.account_config.match_url.as_ref()
    }

    pub fn description(&self) -> Option<&String> {
        self.account_config
            .description
            .as_ref()
            .or_else(|| self.identity_config.description.as_ref())
    }

    pub fn token(&self) -> Option<&String> {
        self.account_config.token.as_ref()
    }
}

impl<'a> Display for Identity<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, [user={}, email={}, desc={}]",
            self.id(),
            self.user().unwrap_or(&"no username".to_string()),
            self.email().unwrap_or(&"no email".to_string()),
            self.description().unwrap_or(&"no description".to_string())
        )
    }
}
