# Promote stable workflows to directory modules

## Decision

Start a responsibility in one file. Promote it to a directory module only after it owns multiple independently named workflows or stable responsibilities. The directory root is the module interface, and each child module owns one complete responsibility.

## Why

Complex workflow files make a named operation difficult to locate and force readers to load unrelated behavior. A workflow directory gives each stable operation a predictable home while preserving one small public interface.

The split is responsibility-specific. Mirroring `create.rs`, `update.rs`, and `delete.rs` across Domain, Application, HTTP, and Infrastructure would replace one large file with shallow modules and cross-file glue. Each crate therefore splits at its own ownership boundary.

## Rejected alternatives

- Keep every responsibility in one file: acceptable at first, but obscures independently evolving workflows once they exist.
- Create one file for every command from the start: adds navigation and wrappers before there is enough behavior to own them.
- Mirror one feature tree through every crate: confuses shared vocabulary with shared implementation shape and weakens layer ownership.
