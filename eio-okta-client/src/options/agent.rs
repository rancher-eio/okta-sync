use clap::{ArgAction, Args};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use ureq::{Agent, AgentBuilder};

#[derive(Debug, Clone, Args, Serialize, Deserialize)]
#[group(skip)]
#[remain::sorted]
pub struct UreqAgent {
  #[arg(action = ArgAction::Set)]
  #[arg(default_value = "false")]
  #[arg(env = "UREQ_AGENT_HTTPS_ONLY")]
  #[arg(id = "ureq_agent_https_only")]
  #[arg(long = "ureq-agent-https-only")]
  #[arg(value_name = "BOOL")]
  pub https_only: bool,
  #[arg(default_value = "100")]
  #[arg(env = "UREQ_AGENT_MAX_IDLE_CONNECTIONS")]
  #[arg(id = "ureq_agent_max_idle_connections")]
  #[arg(long = "ureq-agent-max-idle-connections")]
  #[arg(value_name = "NUMBER")]
  pub max_idle_connections: usize,
  #[arg(default_value = "1")]
  #[arg(env = "UREQ_AGENT_MAX_IDLE_CONNECTIONS_PER_HOST")]
  #[arg(id = "ureq_agent_max_idle_connections_per_host")]
  #[arg(long = "ureq-agent-max-idle-connections-per-host")]
  #[arg(value_name = "NUMBER")]
  pub max_idle_connections_per_host: usize,
  #[arg(action = ArgAction::Set)]
  #[arg(default_value = "true")]
  #[arg(env = "UREQ_AGENT_NO_DELAY")]
  #[arg(id = "ureq_agent_no_delay")]
  #[arg(long = "ureq-agent-no-delay")]
  #[arg(value_name = "BOOL")]
  pub no_delay: bool,
  #[arg(default_value = "5")]
  #[arg(env = "UREQ_AGENT_REDIRECTS")]
  #[arg(id = "ureq_agent_redirects")]
  #[arg(long = "ureq-agent-redirects")]
  #[arg(value_name = "NUMBER")]
  pub redirects: u32,
  #[arg(env = "UREQ_AGENT_TIMEOUT")]
  #[arg(id = "ureq_agent_timeout")]
  #[arg(long = "ureq-agent-timeout")]
  #[arg(value_parser = humantime::parse_duration)]
  #[arg(value_name = "DURATION")]
  #[serde(with = "humantime_serde")]
  pub timeout: Option<Duration>,
  #[arg(default_value = "30s")]
  #[arg(env = "UREQ_AGENT_TIMEOUT_CONNECT")]
  #[arg(id = "ureq_agent_timeout_connect")]
  #[arg(long = "ureq-agent-timeout-connect")]
  #[arg(value_parser = humantime::parse_duration)]
  #[arg(value_name = "DURATION")]
  #[serde(with = "humantime_serde")]
  pub timeout_connect: Duration,
  #[arg(env = "UREQ_AGENT_TIMEOUT_READ")]
  #[arg(id = "ureq_agent_timeout_read")]
  #[arg(long = "ureq-agent-timeout-read")]
  #[arg(value_parser = humantime::parse_duration)]
  #[arg(value_name = "DURATION")]
  #[serde(with = "humantime_serde")]
  pub timeout_read: Option<Duration>,
  #[arg(env = "UREQ_AGENT_TIMEOUT_WRITE")]
  #[arg(id = "ureq_agent_timeout_write")]
  #[arg(long = "ureq-agent-timeout-write")]
  #[arg(value_parser = humantime::parse_duration)]
  #[arg(value_name = "DURATION")]
  #[serde(with = "humantime_serde")]
  pub timeout_write: Option<Duration>,
  #[arg(env = "UREQ_AGENT_USER_AGENT")]
  #[arg(id = "ureq_agent_user_agent")]
  #[arg(long = "ureq-agent-user-agent")]
  #[arg(value_name = "STRING")]
  pub user_agent: Option<String>,
}

impl From<UreqAgent> for AgentBuilder {
  fn from(options: UreqAgent) -> Self {
    let UreqAgent {
      https_only,
      max_idle_connections,
      max_idle_connections_per_host,
      no_delay,
      redirects,
      timeout,
      timeout_connect,
      timeout_read,
      timeout_write,
      user_agent,
    } = options;

    let builder = Self::new()
      .https_only(https_only)
      .max_idle_connections(max_idle_connections)
      .max_idle_connections_per_host(max_idle_connections_per_host)
      .no_delay(no_delay)
      .redirects(redirects)
      .timeout_connect(timeout_connect);

    let builder = match timeout {
      Some(timeout) => builder.timeout(timeout),
      None => builder,
    };

    let builder = match timeout_read {
      Some(timeout) => builder.timeout_read(timeout),
      None => builder,
    };

    let builder = match timeout_write {
      Some(timeout) => builder.timeout_write(timeout),
      None => builder,
    };

    match user_agent {
      Some(user_agent) => builder.user_agent(&user_agent),
      None => builder,
    }
  }
}

impl From<UreqAgent> for Agent {
  fn from(options: UreqAgent) -> Self {
    AgentBuilder::from(options).build()
  }
}
