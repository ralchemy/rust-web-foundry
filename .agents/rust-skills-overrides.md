# Project overrides for rust-skills

Authority order is: the requested behavior and acceptance tests, this project's code/manifests/gates, this file, then the generic `rust-skills` rules.

- `proj-mod-by-feature`: group capabilities inside their owning architecture layer; never merge Domain, Application, HTTP, Infrastructure, and `app` responsibilities into one feature module.
- `proj-prelude-module`: do not add a prelude by default; explicit imports make cross-layer dependencies visible.
- `obs-tracing-over-log` and `obs-instrument-spans`: the selected stack is fastrace + Logforth + the `log` facade + OpenTelemetry. Review that integration instead of replacing it with `tracing`.
- `err-anyhow-app`: `anyhow` is limited to the executable host boundary. Domain, Application, HTTP, and Infrastructure retain typed errors and stable inward/public categories.
- `err-source-chain`, `err-context-chain`, and `obs-error-chain`: preserve concrete causes only at their owning boundary and only when safe. Stable cross-layer categories and redaction take precedence over logging raw SQLx, reqwest, URL, payload, credential, or rejected-value details.
- `test-mockall-mocking` and `test-mock-traits`: prefer the existing small inline fakes, local Axum peer, and Compose MySQL path. Add a mocking dependency only for a demonstrated repeated setup problem.
- `async-cancellation-token`: preserve the existing owned server drain/shutdown path. Introduce cancellation tokens only for a new independently scheduled subsystem that needs them.
- `doc-all-public`, `lint-missing-docs`, `api-non-exhaustive`, and `api-serde-optional`: these crates form an internal service workspace, not a published library API; apply these rules only when the task establishes a public library contract.
- `proj-msrv-declare`: the generated service tracks stable Rust and does not promise an MSRV unless the project explicitly adopts one.
- `opt-*`, dependency-adding `mem-*` rules, `perf-ahash`, and generic Cargo profile recommendations require measurements and explicit performance scope; they are never default review findings.
