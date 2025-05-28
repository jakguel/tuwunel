# Tuwunel <sup>🎔</sup>

[![CI/CD](https://github.com/matrix-construct/tuwunel/actions/workflows/main.yml/badge.svg?branch=main)](https://github.com/matrix-construct/tuwunel/actions/workflows/main.yml)
![GitHub Repo stars](https://img.shields.io/github/stars/matrix-construct/tuwunel?style=flat&color=%23fcba03&link=https%3A%2F%2Fgithub.com%2Fmatrix-construct%2Ftuwunel)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/matrix-construct/tuwunel?style=flat&color=%2303fcb1&link=https%3A%2F%2Fgithub.com%2Fmatrix-construct%2Ftuwunel%2Fpulse%2Fmonthly)
![GitHub Created At](https://img.shields.io/github/created-at/matrix-construct/tuwunel)
![GitHub License](https://img.shields.io/github/license/matrix-construct/tuwunel)

<!-- ANCHOR: catchphrase -->

## High Performance Matrix Homeserver in Rust!

<!-- ANCHOR_END: catchphrase -->

<!-- ANCHOR: body -->

Tuwunel is a featureful [Matrix](https://matrix.org/) federation homeserver you can use instead of Synapse
with your favorite Matrix [client](https://matrix.org/ecosystem/clients/),
[bridge](https://matrix.org/ecosystem/bridges/) or
[bot](https://matrix.org/ecosystem/integrations/). The server is written entirely in Rust as a scalable,
low-cost, lightweight, community-driven alternative covering all but the most niche use-cases.

This project is the official successor to [conduwuit](https://github.com/girlbossceo/conduwuit), which
was a featureful and high-performance fork of [Conduit](https://gitlab.com/famedly/conduit), all
community-lead alternatives to Synapse implementing the compatible
[Matrix Specification](https://spec.matrix.org/latest/).

Tuwunel is operated by enterprise users with a vested interest in sponsoring its continued
development. It is now maintained by full-time staff.

### Getting Started

1. Download Tuwunel
	- [GitHub Releases](https://github.com/matrix-construct/tuwunel/releases)
	- [Sourcecode](https://github.com/matrix-construct/tuwunel/) `git clone https://github.com/matrix-construct/tuwunel.git`
    - [DockerHub](https://hub.docker.com/r/jevolk/tuwunel) or `docker pull jevolk/tuwunel:latest`
    - [GHCR](https://github.com/matrix-construct/tuwunel/pkgs/container/tuwunel) or `docker pull ghcr.io/matrix-construct/tuwunel:latest`
    - Deb and RPM packages are available and this will be updated with a link.
    - Arch Package is expected very soon and this will be updated.
    - Nix Package has not yet been updated but expect this soon.

2. [Configure](https://github.com/matrix-construct/tuwunel/blob/main/docs/configuration.md) by copying and editing the `tuwunel-example.toml`. **Most users deploy via docker or a distribution package and should follow the [appropriate guide](https://github.com/matrix-construct/tuwunel/tree/main/docs/deploying) instead**. This is just a summary for the impatient. See the full [documentation](https://github.com/matrix-construct/tuwunel/blob/main/docs/).

	- Setting the `server_name` and `database_path` is required.
	- The `registration_token` is the easiest method to enable `allow_registration = true`.

	👉 Avoid using a sub-domain for your `server_name`. You can always delegate `example.com` to `matrix.example.com` by setting up a [`.well-known`](https://github.com/spantaleev/matrix-docker-ansible-deploy/blob/master/docs/configuring-well-known.md) file later, but you can never change your `server_name`.

3. Setup TLS certificates. Most users enjoy the [Caddy](https://caddyserver.com/) reverse-proxy which automates their certificate renewal. Advanced users can load their own TLS certificates using the configuration and Tuwunel can be deployed without a reverse proxy. Example `/etc/caddy/Caddyfile` configuration with [Element-web](https://github.com/element-hq/element-web/releases) unzipped in `/var/www/element`:
    ```
    https://tuwunel.me:8448 {
       reverse_proxy http://127.0.0.1:8008
    }

    https://tuwunel.me:443 {
        root * /var/www/element/
        file_server
   }
    ```
    `caddy reload --config /etc/caddy/Caddyfile`


 🤗 Did you find this and other documentation helpful? We would love your feedback when setting up Tuwunel. _No problem is your fault_. Please open an issue to help us streamline.
	

### Migrating to Tuwunel

| Can I migrate from | |
|-----------------|-----------|
| conduwuit? | ✅ Yes. This will be supported at a minimum for one year, but likely indefinitely. |
| Synapse? | ❌ Not yet, but this is planned and an important issue. Subscribe to [#2](https://github.com/matrix-construct/tuwunel/issues/2). |
| Conduit? | ❌ Not right now, but this is planned and expected in the near future. Subscribe to [#41](https://github.com/matrix-construct/tuwunel/issues/41). |
| Another domain? |  ❌ No. Matrix does not yet support changing your domain. The `server_name` you choose is permanent. |
| Any other fork of Conduit? |  ❌ **Never switch between different derivatives of Conduit or you will corrupt your database.** Even if it appears to be working fine for a while, the corruption is often silent and the damage is permanent. We cannot help you the moment you run another fork without a migration path explicitly sanctioned by Tuwunel in this table. |


#### Migrating from conduwuit

Migrating from conduwuit to Tuwunel _just works_. In technical parlance it is a "binary swap."
All you have to do is update to the latest Tuwunel and change the path to the executable from
`conduwuit` to `tuwunel`.

Anything else named "conduwuit" is still recognized, this includes environment variables with prefixes
such as `CONDUWUIT_`. In fact, `CONDUIT_` is still recognized for our legacy users. You may have
noticed that various configs, yamls, services, users, and other items were renamed, but if you
were a conduwuit user we recommend against changing anything at all. This will keep things simple.
If you are not sure please ask. If you found out that something did in fact need to be changed
please open an issue immediately.


### Upgrading & Downgrading Tuwunel

We strive to make moving between versions of Tuwunel safe and easy. There may be some startup
delays during a significant update involving database schema changes. These are always crafted to
handle any loss of power or partial completion. If you encounter a problem during startup after an
update: don't panic, report the issue, try again a few times, or fallback to the last version you
were using.

Downgrading Tuwunel is always safe but often prevented by a guard. An error will indicate the
downgrade is not possible and a newer version which does not error must be sought.

#### Versioning

Tuwunel uses a semantic version tag in the format of `v<major>.<minor>.<patch>`. The `patch` value will
always correspond to the number of commits from the last `minor` change. The `minor` value is changed
at developer discretion, but has significance to users. The `major` value is changed to indicate
significant milestones, but does not necessarily indicate any "breaking" change. Note that an irreversible
database schema change may occur with only a `minor` version change, but these policies are not finalized,
or very important for users right now. **Users should place an importance on keeping up to date with the
latest `minor` version.**

We currently do not have separate stable and unstable branches, but we reserve the possibility for
exploring this model in the future. This would only occur between different `major`
versions (i.e. `1.x` is stable and only receives fixes, while `2.x` receives new features). There are no
concrete plans to move to this model at this time.

#### Branches

The main branch is always _reasonably safe_ to run. We understand the propensity for users to simply clone
the main branch to get up and running, and we're obliged to ensure it's always viable. Nevertheless, only
tagged releases are true releases. If you don't care to update often, find the latest `minor` version
change rather than `patch`. We don't recommend simply following `major` version changes at this time.

#### Tracking

Containerized deployments often track an image tag to automatically update and restart the server. **We
strongly advise tracking the `:latest` tag** (or some other similar indirect meta-tag appropriate for
your platform). This gives us the necessary discretion to keep you on the appropriate version; we will
point these tags at the latest stable release. We discourage tracking the main branch, as we want to
update that more frequently moving forward.


### Getting Help & Support

The official community will be found at [#tuwunel:tuwunel.chat](https://matrix.to/#/#tuwunel:tuwunel.chat).
If this is currently inaccessible please be patient as it's still coming online at the time of
the first release; we will have updates to this section. This is a fully moderated space to protect
the wellbeing of our users and create a non-toxic work environment for staff. Users will not have
permission to post by default. Additional access is granted on an as-needed/as-trusted basis.
If you require assistance with anything that is not remedied by the documentation, please open
an issue on github. If discussion is required we will grant access at that point.

If you are opposed to using github, or private discussion is required, or for any other reason,
I would be happy to receive your DM at [@jason:tuwunel.me](https://matrix.to/#/@jason:tuwunel.me),
you will not be bothering me and it would be my pleasure to help you anytime. As an emergency contact
you can send an email to jasonzemos@gmail.com.

##### Tuwunel Fanclub

We also have an unofficial community-run chat which is publicly accessible at
[#tuwunel:grin.hu](https://matrix.to/#/#tuwunel:grin.hu). The members, content, or moderation
decisions of this room are not in any way related or endorsed by this project or its sponsors,
and not all project staff will be present there. There will be at least some presence by staff to
offer assistance so long as the room remains in minimally good standing.


## Tuwunel <sup>🎔</sup>

Tuwunel's theme is **empathy** in communication defined by the works of
[Edith Stein](https://plato.stanford.edu/entries/stein/). Empathy is the basis for how we approach
every message, and a reminder for how we should all conduct ourselves in every conversation.

<!-- ANCHOR_END: body -->

<!-- ANCHOR: footer -->

<!-- ANCHOR_END: footer -->
