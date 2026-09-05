# Held-out requirement: release policy

Before publishing a `Release`, call an external ReleasePolicy capability with the release identifier and channel. The downstream decision is `allow` or `deny`. Denial is a stable business/application rejection; malformed downstream data and unavailability are distinct dependency failures. Do not persist the publish transition when policy denies or cannot provide an allowed decision.

The requirement does not define TaskPolicy-compatible payload fields, retries, caching, authentication, idempotency, or additional release lifecycle rules. Do not infer them from the Task reference.

Acceptance includes a local controllable HTTP peer and production composition evidence without contacting a public network service.
