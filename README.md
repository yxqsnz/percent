# Percent

A **Work In Progress** Twitter clone written in Rust.

## Requirements

- [Docker compose](https://docs.docker.com/compose/)
- [Rust](https://rust-lang.org)

## Building

- Run: `cargo build`

## Running

- Start database: `docker-compose up -d postgres`
- Start APIServer: `cargo run --bin api-server`

## FAQ

### Why Rust?

🦀

### I don't like docker!!! Can I use a alternative?

Yes. [Podman](https://podman.io/) and [Podman compose](https://github.com/containers/podman-compose)

### Where can I check development phase?

- Check [PHASES.md](./PHASES.md)
