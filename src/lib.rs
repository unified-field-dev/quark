//! Generic link-time registry infrastructure for Rust.
//!
//! Provides a generic `Registrable` trait, `Registry<T>` struct, and
//! `define_registry!` macro for building type-safe, owned registries
//! backed by [`inventory`] link-time collection.

pub use inventory;

use std::collections::HashMap;

/// Trait for types that can be stored in a `Registry`.
///
/// Implementors must provide a string key used for lookup and deduplication.
/// The `'static + Send + Sync` bounds are required because descriptors live
/// in static memory (via `inventory`) and registries are shared across threads.
pub trait Registrable: 'static + Send + Sync {
    /// Return the unique lookup key used by the registry.
    fn registry_key(&self) -> &str;
}

/// A generic, owned registry mapping string keys to `&'static T` references.
///
/// Registries are immutable after construction — no `Mutex` needed.
/// Use `auto_discover()` to populate from `inventory`, or `new()` + `register()`
/// for manual construction (useful in tests).
pub struct Registry<T: Registrable> {
    items: HashMap<String, &'static T>,
}

impl<T: Registrable> std::fmt::Debug for Registry<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Registry")
            .field("len", &self.items.len())
            .field("keys", &self.list())
            .finish()
    }
}

impl<T: Registrable> Registry<T> {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    /// Populate from all `inventory::submit!`-ed items of type `T`.
    pub fn auto_discover() -> Self
    where
        T: inventory::Collect,
    {
        let mut reg = Self::new();
        for item in inventory::iter::<T> {
            reg.register(item);
        }
        reg
    }

    /// Insert a descriptor. If a descriptor with the same key already exists,
    /// it is replaced (last-write-wins).
    pub fn register(&mut self, item: &'static T) {
        self.items.insert(item.registry_key().to_string(), item);
    }

    /// Look up an item by registry key.
    pub fn get(&self, key: &str) -> Option<&'static T> {
        self.items.get(key).copied()
    }

    /// Returns all registered keys in arbitrary order.
    pub fn list(&self) -> Vec<&str> {
        self.items.keys().map(|k| k.as_str()).collect()
    }

    /// Return the number of registered items.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Return `true` when no items are registered.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Iterates over all registered descriptors in arbitrary order.
    pub fn iter(&self) -> impl Iterator<Item = &'static T> + '_ {
        self.items.values().copied()
    }
}

impl<T: Registrable> Default for Registry<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Registrable> Clone for Registry<T> {
    fn clone(&self) -> Self {
        Self {
            items: self.items.clone(),
        }
    }
}

/// Generates a newtype wrapper around `Registry<T>` with standard impls.
///
/// # Usage
///
/// ```ignore
/// quark::define_registry! {
///     pub struct ScriptRegistry for ScriptDescriptor;
/// }
/// ```
///
/// This generates:
/// - A newtype struct wrapping `quark::Registry<T>`
/// - `Deref` / `DerefMut` to `Registry<T>` (inherits `get`, `list`, `len`, etc.)
/// - `new()`, `auto_discover()`, `register()` on the newtype
/// - `Debug`, `Clone`, `Default` impls
#[macro_export]
macro_rules! define_registry {
    (
        $(#[$meta:meta])*
        $vis:vis struct $Name:ident for $Item:ty;
    ) => {
        $(#[$meta])*
        $vis struct $Name {
            inner: $crate::Registry<$Item>,
        }

        impl $Name {
            pub fn new() -> Self {
                Self {
                    inner: $crate::Registry::new(),
                }
            }

            pub fn auto_discover() -> Self {
                Self {
                    inner: $crate::Registry::auto_discover(),
                }
            }

            pub fn register(&mut self, item: &'static $Item) {
                self.inner.register(item);
            }
        }

        impl ::std::ops::Deref for $Name {
            type Target = $crate::Registry<$Item>;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl ::std::ops::DerefMut for $Name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }

        impl ::std::fmt::Debug for $Name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct(stringify!($Name))
                    .field("len", &self.inner.len())
                    .field("keys", &self.inner.list())
                    .finish()
            }
        }

        impl Clone for $Name {
            fn clone(&self) -> Self {
                Self {
                    inner: self.inner.clone(),
                }
            }
        }

        impl Default for $Name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}
