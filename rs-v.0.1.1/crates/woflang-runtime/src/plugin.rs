//! Dynamic plugin loading for Woflang.
//!
//! This module provides runtime loading of plugin shared libraries.
//! It is gated behind the `dynamic-plugins` feature to keep the core
//! runtime dependency-free.
//!
//! # Plugin ABI
//!
//! Plugins must export a `register_plugin` function with the following
//! signature:
//!
//! ```c
//! extern "C" void register_plugin(void* interpreter);
//! ```
//!
//! The interpreter pointer is transmuted to a mutable reference to
//! [`Interpreter`] for registration.
//!
//! # Safety
//!
//! Dynamic plugin loading is inherently unsafe. Plugins can:
//! - Execute arbitrary code
//! - Violate memory safety guarantees
//! - Cause undefined behavior if ABI mismatches occur
//!
//! Only load plugins from trusted sources.

// Dynamic loading via libloading requires unsafe blocks.
// This is an unavoidable consequence of FFI with unknown code.
#![allow(unsafe_code)]

use crate::Interpreter;
use libloading::{Library, Symbol};
use std::path::Path;
use woflang_core::{Result, WofError};

/// Type signature for plugin registration functions.
///
/// Plugins receive a mutable reference to the interpreter and should
/// use it to register their operations via `interpreter.register(...)`.
pub type RegisterPluginFn = unsafe extern "C" fn(&mut Interpreter);

/// Manages loaded plugin libraries.
///
/// The loader keeps plugin libraries alive for the duration of the
/// interpreter's lifetime. Dropping the loader will unload all plugins.
pub struct PluginLoader {
    libraries: Vec<Library>,
}

impl Default for PluginLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginLoader {
    /// Create a new empty plugin loader.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            libraries: Vec::new(),
        }
    }

    /// Load a single plugin from a shared library path.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The library cannot be loaded
    /// - The library does not export `register_plugin`
    ///
    /// # Safety
    ///
    /// This function loads and executes code from the specified path.
    /// Only load plugins from trusted sources.
    pub fn load_plugin(&mut self, path: impl AsRef<Path>, interp: &mut Interpreter) -> Result<()> {
        let path = path.as_ref();

        // SAFETY: Loading arbitrary shared libraries is inherently unsafe.
        // We're trusting the plugin to maintain memory safety.
        let library = unsafe { Library::new(path) }
            .map_err(|e| WofError::plugin(format!("failed to load {}: {e}", path.display())))?;

        // Look up the registration function
        // SAFETY: We're trusting the plugin exports the correct function signature
        let register_fn: Symbol<RegisterPluginFn> = unsafe { library.get(b"register_plugin") }
            .map_err(|e| {
                WofError::plugin(format!(
                    "plugin {} missing 'register_plugin': {e}",
                    path.display()
                ))
            })?;

        // Call the registration function
        // SAFETY: Trusting the plugin to not violate memory safety
        unsafe {
            register_fn(interp);
        }

        // Keep the library alive
        self.libraries.push(library);

        Ok(())
    }

    /// Load all plugins from a directory.
    ///
    /// Loads all files matching the platform's shared library extension:
    /// - Windows: `.dll`
    /// - macOS: `.dylib`
    /// - Linux: `.so`
    ///
    /// Errors loading individual plugins are collected and returned
    /// together; loading continues for remaining plugins.
    pub fn load_plugins_from_dir(
        &mut self,
        dir: impl AsRef<Path>,
        interp: &mut Interpreter,
    ) -> Result<Vec<String>> {
        let dir = dir.as_ref();
        let mut loaded = Vec::new();
        let mut errors = Vec::new();

        if !dir.exists() {
            return Ok(loaded);
        }

        let entries = std::fs::read_dir(dir).map_err(|e| WofError::io(e.to_string()))?;

        for entry in entries.flatten() {
            let path = entry.path();
            if !is_shared_library(&path) {
                continue;
            }

            match self.load_plugin(&path, interp) {
                Ok(()) => {
                    loaded.push(path.display().to_string());
                }
                Err(e) => {
                    errors.push(format!("{}: {e}", path.display()));
                }
            }
        }

        if !errors.is_empty() {
            eprintln!("Plugin loading errors:");
            for err in &errors {
                eprintln!("  - {err}");
            }
        }

        Ok(loaded)
    }

    /// Get the number of loaded plugins.
    #[must_use]
    pub fn count(&self) -> usize {
        self.libraries.len()
    }
}

/// Check if a path is a shared library based on extension.
fn is_shared_library(path: &Path) -> bool {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    #[cfg(target_os = "windows")]
    let valid = ext.eq_ignore_ascii_case("dll");

    #[cfg(target_os = "macos")]
    let valid = ext == "dylib";

    #[cfg(target_os = "linux")]
    let valid = ext == "so";

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    let valid = ext == "so" || ext == "dylib";

    valid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_library_detection() {
        #[cfg(target_os = "windows")]
        {
            assert!(is_shared_library(Path::new("plugin.dll")));
            assert!(!is_shared_library(Path::new("plugin.so")));
        }

        #[cfg(target_os = "linux")]
        {
            assert!(is_shared_library(Path::new("plugin.so")));
            assert!(!is_shared_library(Path::new("plugin.dll")));
        }

        #[cfg(target_os = "macos")]
        {
            assert!(is_shared_library(Path::new("plugin.dylib")));
            assert!(!is_shared_library(Path::new("plugin.dll")));
        }
    }

    #[test]
    fn empty_loader() {
        let loader = PluginLoader::new();
        assert_eq!(loader.count(), 0);
    }
}
