# eio-okta-sync - a VERY specific way to turn Okta Users and Groups into GitHub Members and Teams

## Overview

`eio-okta-sync` takes Users and Groups from Okta, and produces Memberships for GitHub.

## Why does this exist?

For those times where you want Okta to be the Source of Truth for who should be
a member of your org, but you also want to use a GitOps-style workflow.

## How does it work?

1. It reads Users and Groups from Okta, and creates a snapshot of the current state.
2. You provide a configuration file that maps groups to teams and orgs, and users to roles.
3. The snapshot and mapping config are used to produce a set of Managed Resources in YAML format.
4. You commit these to a repository, and whatever GitOps process you have in place takes over.

## Requirements

1. an [Okta](https://www.okta.com) account with an API token with sufficient privileges to read the relevant users and groups.
2. a [Kubernetes](https://kubernetes.io) cluster running [Crossplane](https://www.crossplane.io) and [this GitHub Provider](https://marketplace.upbound.io/providers/coopnorge/provider-github/) or a compatible variant.
3. some [GitOps](https://www.gitops.tech) system such as [Flux](https://fluxcd.io).

## How do I obtain this majestic tool?

```fish
cargo install eio-okta-sync
```

## How do I use it?

The examples below assume you are using [`pass`](https://www.passwordstore.org) for secret storage. This is not required, and is simply for illustrative purposes. Adjust the commands accordingly if you are doing something else.

### Checking for Available Updates

As of `v0.2.0`, there is a built-in way to check for available updates. If an update is available, it can be installed the same as installing the program in the first place.

```fish
eio-okta-sync check-version
```

```json
{
  "name": "eio-okta-sync",
  "versions": {
    "current": "0.2.0",
    "latest": "0.2.0"
  }
}
```

### Creating Resources from Okta

This is the most common workflow, where Okta is the Source of Truth.

#### Create Okta Snapshot

Environment

- `OKTA_AUTHORIZATION` should be a valid Okta SSWS Token.
- `OKTA_SERVICE_AUTHORITY` should be your Okta Endpoint.

This example assume your Okta Endpoint is `suse.okta.com`, which is _almost certainly incorrect_ unless you work for [SUSE](https://www.suse.com). Set yours appropriately.

For `fish`:

```fish
set --export OKTA_SERVICE_AUTHORITY "suse.okta.com"
set --export OKTA_AUTHORIZATION (pass okta/api-key)
```

For vintage shells like `bash`:

```shell
export OKTA_SERVICE_AUTHORITY="suse.okta.com"
export OKTA_AUTHORIZATION="$(pass okta/api-key)"
```

Generate a snapshot:

```fish
eio-okta-sync snapshot
```

```text
collecting users...
request: https://suse.okta.com/api/v1/users?limit=200
request: https://suse.okta.com/api/v1/users?limit=200&after=200udfj87h3lcZ9CLf357
collecting groups...
request: https://suse.okta.com/api/v1/groups?
collecting members for group 00g195mbu8efWZW3R358...
request: https://suse.okta.com/api/v1/groups/00g195mbu8efWZW3R358/users?limit=200
collecting members for group 00g15m9kp9ilv7cLi358...
request: https://suse.okta.com/api/v1/groups/00g15m9kp9ilv7cLi358/users?limit=200
request: https://suse.okta.com/api/v1/groups/00g15m9kp9ilv7cLi358/users?limit=200&after=00uj5wovq9j3cAjim357
collecting members for group 00g166tf934YjQGLF358...
request: https://suse.okta.com/api/v1/groups/00g166tf934YjQGLF358/users?limit=200

[OK] snapshot saved to snapshot.yaml
```

#### Create a Mappings Config (first time only)

You'll need a config that describes how you want to map things.

You can get an example of this with:

```fish
eio-okta-sync generate --help
```

Or you can generate one interactively, based on your Okta Snapshot:

```fish
eio-okta-sync mappings
```

#### Generate Resources from Okta Snapshot

```fish
eio-okta-sync generate
```

```text
validating expectations for okta group with id: 00g195mbu8efWZW3R358 ...
validating expectations for okta group with id: 00g15m9kp9ilv7cLi358 ...
validating expectations for okta group with id: 00g166tf934YjQGLF358 ...
Skipping Provisioned User: provisioned-user@company.local
Skipping PasswordExpired User: password-expired@company.local
Skipping Suspended User: suspended@company.local

[OK] resources saved to resources.yaml
```

### Creating Resources from GitHub

This workflow uses GitHub as the Source of Truth instead.

This is particularly useful when _onboarding_ an existing Org.

These examples assume your GitHub Org is `rancher`, which is _almost certainly incorrect_ for most people. Set yours appropriately.

#### Create GitHub Snapshot

Environment:

- `GITHUB_TOKEN` should be a valid Github Access Token.

```fish
eio-okta-sync members --org rancher
```

```text
[OK] members saved to members.yaml
```

#### Generate Resources from GitHub Snapshot

```fish
eio-okta-sync current --org rancher
```

```text
[OK] resources saved to current.yaml
```

## License

```text
SPDX-License-Identifier: MIT OR Apache-2.0
```

`eio-okta-data` is available under [your choice](https://fossa.com/blog/dual-licensing-models-explained/) of _either_ the [MIT License](https://colstrom.mit-license.org) _or_ the [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0).

See [`LICENSE-MIT`](../LICENSE-MIT) and [`LICENSE-APACHE`](../LICENSE-APACHE) at the root of the repository for the full text of each.

Both are written in fancy lawyer-speak. If you prefer more down-to-earth language, consider the following:

- tl;drLegal has simple visual summaries available: [`MIT`](https://www.tldrlegal.com/license/mit-license) or [`Apache-2.0`](https://www.tldrlegal.com/license/apache-license-2-0-apache-2-0)
- FOSSA has more in-depth overviews available: [`MIT`](https://fossa.com/blog/open-source-licenses-101-mit-license/) or [`Apache-2.0`](https://fossa.com/blog/open-source-licenses-101-apache-license-2-0/)
