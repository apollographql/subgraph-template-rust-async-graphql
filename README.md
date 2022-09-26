# async-graphql Subgraph Template

This template can be used to quickly create an [Apollo Federation] subgraph with the [async-graphql] crate.

## What's Included

- A basic, [Apollo Federation] subgraph with simple examples for queries, entities, and mutations. You can run this subgraph with `cargo run`.
- Example tests in the `tests` directory. You can run these tests with `cargo test`.
- GitHub Actions workflows which will:
  - Run `cargo test` on every push.
  - Check the schema against Apollo Studio on every push.
  - Publish the subgraph to Apollo Studio on every push to the `main` branch.

## Next Steps

- Download [Rover] and start it using the command printed out from `cargo run` to start a local version of Apollo Explorer.
- Replace "my-subgraph" in `Cargo.toml` with the name of your subgraph.
- Start filling in your own types and resolvers in `src/lib.rs`.
- Set these secrets in GitHub Actions:
  - `APOLLO_KEY`: An Apollo Studio API key for the supergraph to enable schema checks and publishing of the subgraph.
  - `APOLLO_GRAPH_REF`: The name of the supergraph in Apollo Studio.
  - `PRODUCTION_URL`: The URL of the deployed subgraph that the supergraph gateway will route to.
- Remove the `if: false` lines from `.github/workflows/checks.yaml` and `.github/workflows/deploy.yaml` to enable schema checks and publishing.
- Write your custom deploy logic in `.github/workflows/deploy.yaml`.

[apollo federation]: https://www.apollographql.com/docs/federation/
[async-graphql]: https://async-graphql.github.io/async-graphql/
[rover]: https://www.apollographql.com/docs/rover/
