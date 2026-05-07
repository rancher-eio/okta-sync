use octocrab::Octocrab;
use secrecy::SecretString;

#[derive(Debug, clap::Args)]
pub(crate) struct Token {
  #[arg(
    alias("token"),
    env("GITHUB_TOKEN"),
    help("GitHub Access Token"),
    hide_env_values(true),
    id("github.token"),
    long("github-token"),
    value_name("TOKEN")
  )]
  value: SecretString,
}

impl From<Token> for SecretString {
  fn from(token: Token) -> Self {
    token.value
  }
}

impl TryFrom<Token> for Octocrab {
  type Error = octocrab::Error;
  fn try_from(token: Token) -> Result<Self, Self::Error> {
    Octocrab::builder().personal_token(token).build()
  }
}
