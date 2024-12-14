# eio-okta-client - Okta Client Library and CLI

This crate wraps the abstract API provided by the `okta-data` crate in a concrete implementation, and provides a generic CLI for any specific API Endpoint supported by the `okta-api` crate, in addition to providing a unified CLI for those APIs.

Consider the following [example](./examples/get-user.rs), which produces a fully-functional CLI for the [`getUser` API](https://developer.okta.com/docs/api/openapi/okta-management/management/tag/User/#tag/User/operation/getUser):

```rust
use clap::Parser;
use okta_api::v1::users::GetUser;
use okta_client::command::Standalone;

fn main() {
  let json = Standalone::<GetUser>::parse().resource_json().unwrap();

  println!("{json}");
}
```

If you build run the resulting executable with no arguments, you get something like this:

```text
error: the following required arguments were not provided:
  --authorization <SSWS-TOKEN>
  --user-id <ID or LOGIN>

Usage: get-user --authorization <SSWS-TOKEN> --user-id <ID or LOGIN>

For more information, try '--help'.
```

And passing `--help` gives you something like this:

```text
Usage: get-user [OPTIONS] --authorization <SSWS-TOKEN> --user-id <ID or LOGIN>

Options:
      --ureq-agent-https-only <BOOL>                       [env: UREQ_AGENT_HTTPS_ONLY=] [default: false] [possible values: true, false]
      --ureq-agent-max-idle-connections <NUMBER>           [env: UREQ_AGENT_MAX_IDLE_CONNECTIONS=] [default: 100]
      --ureq-agent-max-idle-connections-per-host <NUMBER>  [env: UREQ_AGENT_MAX_IDLE_CONNECTIONS_PER_HOST=] [default: 1]
      --ureq-agent-no-delay <BOOL>                         [env: UREQ_AGENT_NO_DELAY=] [default: true] [possible values: true, false]
      --ureq-agent-redirects <NUMBER>                      [env: UREQ_AGENT_REDIRECTS=] [default: 5]
      --ureq-agent-timeout <DURATION>                      [env: UREQ_AGENT_TIMEOUT=]
      --ureq-agent-timeout-connect <DURATION>              [env: UREQ_AGENT_TIMEOUT_CONNECT=] [default: 30s]
      --ureq-agent-timeout-read <DURATION>                 [env: UREQ_AGENT_TIMEOUT_READ=]
      --ureq-agent-timeout-write <DURATION>                [env: UREQ_AGENT_TIMEOUT_WRITE=]
      --ureq-agent-user-agent <STRING>                     [env: UREQ_AGENT_USER_AGENT=]
      --authorization <SSWS-TOKEN>                         [env: OKTA_AUTHORIZATION=]
      --auto-paginate                                      automatically fetch additional results when available?
      --okta-service-authority <HOST[:PORT]>               [env: OKTA_SERVICE_AUTHORITY=] [default: suse.okta.com]
      --okta-service-scheme <SCHEME>                       [env: OKTA_SERVICE_SCHEME=] [default: https]
      --user-id <ID or LOGIN>                              ID of an existing Okta user
      --expand <EXPAND>                                    An optional parameter to include metadata in the `_embedded` attribute [possible values: blocks]
      --pretty                                             pretty-print JSON output?
  -h, --help                                               Print help
```

## License

```text
SPDX-License-Identifier: MIT OR Apache-2.0
```

`eio-okta-client` is available under [your choice](https://fossa.com/blog/dual-licensing-models-explained/) of *either* the [MIT License](https://colstrom.mit-license.org) *or* the [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0).

See [`LICENSE-MIT`](../LICENSE-MIT) and [`LICENSE-APACHE`](../LICENSE-APACHE) at the root of the repository for the full text of each.

Both are written in fancy lawyer-speak. If you prefer more down-to-earth language, consider the following:

- tl;drLegal has simple visual summaries available: [`MIT`](https://www.tldrlegal.com/license/mit-license) or [`Apache-2.0`](https://www.tldrlegal.com/license/apache-license-2-0-apache-2-0)
- FOSSA has more in-depth overviews available: [`MIT`](https://fossa.com/blog/open-source-licenses-101-mit-license/) or [`Apache-2.0`](https://fossa.com/blog/open-source-licenses-101-apache-license-2-0/)
