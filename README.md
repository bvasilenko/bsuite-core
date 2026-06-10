# bsuite-core

Shared types and helpers that the b-* command-line prompt-lookup tools link at build time.

Trait surfaces + closed enums + value types that every `bground` / `banchor` / `bsmell` / `bratch` / `bwatch` / `bspector` binary consumes at build time.

```rust
use bsuite_core::{RoutingKey, ExitCode, HostContext};
use bsuite_core::{PromptResolver, Updater, TranscriptAppender};
use bsuite_core::{ManifestOverlayReader, ExitCodeEmitter, OpacityHookPublisher};
use bsuite_core::AdapterHostBinder;
```

7 trait surfaces, 3 closed enums (`RoutingKey` 6 variants, `ExitCode` reserves `{0, 1, 2, 64}`, `HostContext` 5 variants), 1 `thiserror` error enum, all proptest-invariant-tested.

## Install

```sh
cargo add bsuite-core --git https://github.com/bvasilenko/bsuite-core
```

## API surface

| Item | Purpose |
|---|---|
| `RoutingKey` | Closed 6-variant enum naming each `b-*` CLI; `stable_name()` returns kebab-case identifier. |
| `ExitCode` | Closed 4-variant enum reserving `Success=0`, `Finding=1`, `InternalError=2`, `Usage=64`. |
| `HostContext` | Closed 5-variant enum naming CLI vs CMS-plugin host (`L2a`, `PayloadV3`, `StrapiV5`, `SanityV3`, `DirectusV10`). |
| `PromptResolver::resolve` | Trait: routing key + evidence map + optional manifest overlay → directive string. |
| `Updater::update` | Trait: update channel → update outcome. |
| `TranscriptAppender::append` | Trait: transcript record → transcript handle. |
| `ManifestOverlayReader::read_overlay` | Trait: read + validate manifest overlay. |
| `ExitCodeEmitter::emit` | Trait: write `ExitCode` to stdout per CLI convention. |
| `OpacityHookPublisher::publish` | Trait: publish per-tier visibility evidence. |
| `AdapterHostBinder::bind` | Trait: bind `HostContext` to per-host adapter. |

Method bodies are placeholder at this version; the published surface is the trait shape + the closed-enum invariants.

## Versioning

`0.1` is the first published surface. Method bodies land at `0.2`. The closed enums (`RoutingKey`, `ExitCode`, `HostContext`) are stable; variant additions bump the minor version, removals bump major.

## License

MIT.
