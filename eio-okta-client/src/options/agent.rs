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
  #[arg(help_heading = "Ureq Options")]
  #[arg(help = "Enforce the client to only perform HTTPS requests")]
  #[arg(long_help = "Enforce the client to only perform HTTPS requests.

This setting also makes the client refuse HTTPS to HTTP redirects.")]
  pub https_only: bool,
  #[arg(default_value = "100")]
  #[arg(env = "UREQ_AGENT_MAX_IDLE_CONNECTIONS")]
  #[arg(id = "ureq_agent_max_idle_connections")]
  #[arg(long = "ureq-agent-max-idle-connections")]
  #[arg(value_name = "NUMBER")]
  #[arg(help_heading = "Ureq Options")]
  #[arg(help = "Sets the maximum number of connections allowed in the connection pool")]
  #[arg(long_help = "Sets the maximum number of connections allowed in the connection pool.

Setting this to zero would disable connection pooling.")]
  pub max_idle_connections: usize,
  #[arg(default_value = "1")]
  #[arg(env = "UREQ_AGENT_MAX_IDLE_CONNECTIONS_PER_HOST")]
  #[arg(id = "ureq_agent_max_idle_connections_per_host")]
  #[arg(long = "ureq-agent-max-idle-connections-per-host")]
  #[arg(value_name = "NUMBER")]
  #[arg(help_heading = "Ureq Options")]
  #[arg(help = "Sets the maximum number of connections per host to keep in the connection pool")]
  #[arg(
    long_help = "Sets the maximum number of connections per host to keep in the connection pool.

Setting this to zero would disable connection pooling."
  )]
  pub max_idle_connections_per_host: usize,
  #[arg(action = ArgAction::Set)]
  #[arg(default_value = "true")]
  #[arg(env = "UREQ_AGENT_NO_DELAY")]
  #[arg(id = "ureq_agent_no_delay")]
  #[arg(long = "ureq-agent-no-delay")]
  #[arg(value_name = "BOOL")]
  #[arg(help_heading = "Ureq Options")]
  #[arg(help = "Whether no_delay will be set on the tcp socket")]
  #[arg(long_help = "Whether no_delay will be set on the tcp socket.

Setting this to true disables Nagle's algorithm.")]
  pub no_delay: bool,
  #[arg(default_value = "5")]
  #[arg(env = "UREQ_AGENT_REDIRECTS")]
  #[arg(id = "ureq_agent_redirects")]
  #[arg(long = "ureq-agent-redirects")]
  #[arg(value_name = "NUMBER")]
  #[arg(help_heading = "Ureq Options")]
  #[arg(help = "How many redirects to follow")]
  #[arg(long_help = "How many redirects to follow.

Set to 0 to avoid redirects and instead get a response object with the 3xx status code.

If the redirect count hits this limit (and it's > 0), TooManyRedirects is returned.

WARNING: for 307 and 308 redirects, this value is ignored for methods that have a body. You must handle 307 redirects yourself when sending a PUT, POST, PATCH, or DELETE request.")]
  pub redirects: u32,
  #[arg(env = "UREQ_AGENT_TIMEOUT")]
  #[arg(id = "ureq_agent_timeout")]
  #[arg(long = "ureq-agent-timeout")]
  #[arg(value_parser = humantime::parse_duration)]
  #[arg(value_name = "DURATION")]
  #[serde(with = "humantime_serde")]
  #[arg(help_heading = "Ureq Options")]
  #[arg(help = "Timeout for the overall request")]
  #[arg(
    long_help = "Timeout for the overall request, including DNS resolution, connection time, redirects, and reading the response body.

Slow DNS resolution may cause a request to exceed the timeout, because the DNS request cannot be interrupted with the available APIs.

This takes precedence over timeout_read and timeout_write, but not timeout_connect."
  )]
  pub timeout: Option<Duration>,
  #[arg(default_value = "30s")]
  #[arg(env = "UREQ_AGENT_TIMEOUT_CONNECT")]
  #[arg(id = "ureq_agent_timeout_connect")]
  #[arg(long = "ureq-agent-timeout-connect")]
  #[arg(value_parser = humantime::parse_duration)]
  #[arg(value_name = "DURATION")]
  #[serde(with = "humantime_serde")]
  #[arg(help_heading = "Ureq Options")]
  #[arg(help = "Timeout for the socket connection to be successful")]
  #[arg(long_help = "Timeout for the socket connection to be successful.

If both this and timeout are both set, this takes precedence.")]
  pub timeout_connect: Duration,
  #[arg(env = "UREQ_AGENT_TIMEOUT_READ")]
  #[arg(id = "ureq_agent_timeout_read")]
  #[arg(long = "ureq-agent-timeout-read")]
  #[arg(value_parser = humantime::parse_duration)]
  #[arg(value_name = "DURATION")]
  #[serde(with = "humantime_serde")]
  #[arg(help_heading = "Ureq Options")]
  #[arg(help = "Timeout for the individual reads of the socket")]
  #[arg(long_help = "Timeout for the individual reads of the socket.

If both this and timeout are both set, timeout takes precedence.

The default is no timeout. In other words, requests may block forever on reads by default.")]
  pub timeout_read: Option<Duration>,
  #[arg(env = "UREQ_AGENT_TIMEOUT_WRITE")]
  #[arg(id = "ureq_agent_timeout_write")]
  #[arg(long = "ureq-agent-timeout-write")]
  #[arg(value_parser = humantime::parse_duration)]
  #[arg(value_name = "DURATION")]
  #[serde(with = "humantime_serde")]
  #[arg(help_heading = "Ureq Options")]
  #[arg(help = "Timeout for the individual writes to the socket")]
  #[arg(long_help = "Timeout for the individual writes to the socket.

If both this and timeout are both set, timeout takes precedence.

The default is no timeout. In other words, requests may block forever on writes by default.")]
  pub timeout_write: Option<Duration>,
  #[arg(env = "UREQ_AGENT_USER_AGENT")]
  #[arg(id = "ureq_agent_user_agent")]
  #[arg(long = "ureq-agent-user-agent")]
  #[arg(value_name = "STRING")]
  #[arg(help_heading = "Ureq Options")]
  #[arg(help = "The user-agent header to associate with all requests from this agent by default")]
  #[arg(
    long_help = "The user-agent header to associate with all requests from this agent by default.

Defaults to ureq/[VERSION]. You can override the user-agent on an individual request by setting the User-Agent header when constructing the request."
  )]
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
