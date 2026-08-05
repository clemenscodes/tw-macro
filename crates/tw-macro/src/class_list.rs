/// An opaque, component-private class list. See the crate docs: it has no
/// `Display`, no `Debug`, and no public accessor, so a `class:` attribute can
/// take it but nothing can append to it or read it out. `Clone`/`Copy` are the
/// only derives it qualifies for without breaking that opacity.
#[derive(Clone, Copy)]
pub struct ClassList {
    #[cfg_attr(not(feature = "dioxus"), allow(dead_code))]
    class: &'static str,
}

impl ClassList {
    /// Wrap an already-assembled class string. **Internal to the generated
    /// `classes!`/`states!` macros only** — hidden from docs and namespaced so
    /// it can never be mistaken for a public constructor. There is deliberately
    /// no public `ClassList::new`: a component's styling is always the `classes!`
    /// macro over Tailwind literals, never a hand-written class string, and a
    /// library element never carries a project class directly — it is wrapped in
    /// one of our own components whose `classes!` `CLASS` styles a real element.
    /// A raw `ClassList::new("…")` or `.to_library_class()` no longer compiles,
    /// which surfaces every such escape-hatch as a build error.
    #[doc(hidden)]
    pub const fn __from_macro(class: &'static str) -> Self {
        Self {
            class,
        }
    }
}

#[cfg(feature = "dioxus")]
impl dioxus::core::IntoAttributeValue for ClassList {
    fn into_value(self) -> dioxus::core::AttributeValue {
        dioxus::core::AttributeValue::Text(self.class.to_string())
    }
}
