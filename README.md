# Synt SDK

The extension API of [Synt](https://synt.so) and a Rust SDK for writing extensions against it.

An extension is a WebAssembly component that adds actions to Synt, and with them resource kinds, views, panels, sidebar trees, commands and settings. You, your agents and runbooks call its actions like any other. It runs in a sandbox inside the Synt core and reaches nothing on its own: every host, file, secret or core action it uses goes through the core, only when its manifest asks for it, and you see what it asks for before you install it.

<br />

## What's inside

| | |
|---|---|
| [`wit/`](wit/synt-extension.wit) | The interface, `synt:extension@1.0.0`, in WebAssembly's interface language: what the core offers an extension and what an extension exports |
| [`src/`](src) | `synt-sdk`, generated from that file: the `Extension` trait, `register_extension!`, and typed helpers for the host |
| [`schema/`](schema/extension.schema.json) | The JSON Schema of `extension.json`, the manifest |
| [`examples/hello`](examples/hello) | A whole extension with one action |

The SDK covers calls into the core (`call`, and `ssh`, `sftp`, `k8s`, `db` and `tunnel` over its actions), `Storage` and `Cache`, `http`, `resources`, `documents`, `files`, `timers`, `Terminal`, `Notification`, dialogs (`Confirm`, `Prompt`, `Pick`), `Progress`, `view` for native views and `Assets` for web panels.

<br />

## Start an extension

You need Rust 1.98 or later with the WebAssembly target:

```bash
rustup target add wasm32-wasip2
```

An extension is a library crate with a manifest beside it:

```text
hello/
├── Cargo.toml
├── extension.json
└── src/lib.rs
```

### Cargo.toml

```toml
[package]
name = "hello"
version = "0.1.0"
edition = "2024"
publish = false

[lib]
crate-type = ["cdylib"]

[dependencies]
serde = { version = "1", features = ["derive"] }
synt-sdk = { git = "https://github.com/syntso/sdk", tag = "v0.2.0" }

[profile.release]
opt-level = "s"
lto = true
codegen-units = 1
strip = true
```

### extension.json

```json
{
  "$schema": "https://raw.githubusercontent.com/syntso/sdk/main/schema/extension.schema.json",
  "id": "hello",
  "name": "Hello",
  "version": "0.1.0",
  "description": "Greets whoever calls it.",
  "actions": [
    {
      "name": "greet",
      "title": "Say hello",
      "risk": "read",
      "params": {
        "type": "object",
        "properties": { "name": { "type": "string" } }
      },
      "output": {
        "type": "object",
        "properties": { "greeting": { "type": "string" } }
      },
      "agents": true
    }
  ]
}
```

The manifest is the whole promise an extension makes: who it is, what it may reach (`capabilities`, all off unless listed), the actions it adds and what it contributes to the app. Unknown keys are refused. The `id` is 1 to 32 lowercase letters, digits and `-`, starting with a letter, and prefixes every action: this one is called as `hello.greet`. Each action names its `risk` (`read`, `write` or `privileged`), and `agents: true` lets AI agents call it. The core checks params and output against the schemas given here.

### src/lib.rs

```rust
use serde::Deserialize;
use synt_sdk::serde_json::{Value, json};
use synt_sdk::{Error, Extension, Invocation, register_extension};

struct Hello;

#[derive(Deserialize)]
struct Greet {
    #[serde(default)]
    name: Option<String>,
}

impl Extension for Hello {
    fn run(invocation: &Invocation) -> Result<Value, Error> {
        if invocation.action() != "greet" {
            return Err(Error::unknown_action(invocation.action()));
        }

        let greet: Greet = invocation.params()?;
        let name = greet.name.as_deref().unwrap_or("world");

        Ok(json!({ "greeting": format!("Hello, {name}!") }))
    }
}

register_extension!(Hello);
```

`run` gets every call of the extension's actions, with the action's short name, its params, its target and who called. The trait's other methods (`start`, `render`, `children`, `panel_message`, `asset`, `on_event`, `on_timer`, `merge`) have defaults; implement the ones your manifest needs.

The core starts the component afresh for every call, so nothing in memory survives between calls. Keep state in `Storage`, `Cache`, documents or resources.

<br />

## Build

```bash
cargo build --target wasm32-wasip2 --release
```

The component is `target/wasm32-wasip2/release/hello.wasm`.

<br />

## Install it into Synt

Install the folder as a local build with the core action `extension.install_local`, which takes the folder's absolute path. Only a person can call it, from the Synt app; agents and other clients are refused.

- The core reads `extension.json` from the folder, and the component from `extension.wasm` beside it or else the one `.wasm` file in `target/wasm32-wasip2/release`. A component is at most 32 MiB.
- It compiles the component, checks that it imports only what its capabilities allow, checks its dependencies, and shows you its identity, capabilities, actions and contributions. It installs once you approve.
- A local build is unsigned and stays marked as such. Install it again after each rebuild.

Then call it like any action, for example from the [MCP bridge](https://github.com/syntso/mcp) as the tool `hello_greet`.

<br />

## Call the core

An extension reaches the core only through the actions its manifest lists under `capabilities.call`:

```json
{
  "id": "uptime",
  "name": "Uptime",
  "version": "0.1.0",
  "capabilities": { "call": ["ssh.exec"] },
  "actions": [
    { "name": "show", "title": "Show the uptime", "risk": "read", "targets": ["host"], "agents": true }
  ]
}
```

```rust
use synt_sdk::serde_json::{Value, json};
use synt_sdk::{Error, Extension, Invocation, register_extension, ssh};

struct Uptime;

impl Extension for Uptime {
    fn run(invocation: &Invocation) -> Result<Value, Error> {
        let host = invocation.require_target()?;
        let output = ssh::exec(host, "uptime")?;

        if !output.succeeded() {
            return Err(Error::new("uptime_failed", output.stderr));
        }

        Ok(json!({ "uptime": output.stdout.trim() }))
    }
}

register_extension!(Uptime);
```

A call into the core goes through the same approvals and journal as any other caller, with the extension named as the one who asked.

<br />

## Build the SDK

```bash
git clone https://github.com/syntso/sdk.git
cd sdk
git config core.hooksPath .githooks
cargo test
cargo build --manifest-path examples/hello/Cargo.toml --target wasm32-wasip2 --release
```

<br />

## License

[Apache-2.0](LICENSE).
