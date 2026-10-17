# tw-macro

Compile-time typed Tailwind class lists with a declared, keyed responsive band
vocabulary.

A consumer declares its responsive bands once with `define_styling!`, then each
component writes one keyed class list per band it uses; `classes!` joins them
into a single compile-time `const CLASS`. Band typos, misplaced prefixes, and
string interpolation of the assembled class are all compile errors. The crate is
pure Rust and native-testable.

## Usage

Add it as a git dependency, pinned to a tag:

```toml
[dependencies]
tw-macro = { git = "https://github.com/clemenscodes/tw-macro", tag = "v0.3.0" }
```

### The `dioxus` feature

Enable the optional `dioxus` feature to get `ClassList`'s
`IntoAttributeValue` impl, so a `ClassList` can be handed directly to a
`class:` attribute on one of our own elements. There is no bridge for a
third-party component's `String` `class` prop: wrap the library element in one
of our own components and style a real element with `classes!` `CLASS` instead.

```toml
tw-macro = { git = "https://github.com/clemenscodes/tw-macro", tag = "v0.3.0", features = ["dioxus"] }
```

Without the feature, the crate is a pure, framework-agnostic engine.
