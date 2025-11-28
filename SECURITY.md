Security Policy
Supported Versions
Component	Version Range	Support Level
C++ WofLang	v10.1.1	✅ Active
Rust WofLang	v0.0.3 (alpha)	✅ Active
WofLang Alpha-R is currently in an alpha phase; security expectations should match an experimental research and tooling project rather than a hardened production system.

Reporting a Vulnerability
If you believe you’ve found a security issue in:

The C++ interpreter, plugins, or REPL.

The Rust core, runtime, CLI, or ops crates.

Any build scripts or example configurations.

Please:

Do not open a public GitHub issue with exploitable details.

Contact the maintainer privately via:

GitHub’s “Report a vulnerability” feature, or

Direct message / email as listed on the maintainer’s GitHub profile.

Please include:

A clear description of the issue.

Steps to reproduce (minimal repro scripts or input if possible).

The commit hash or release tag you tested against.

Platform details (OS, compiler, Rust version, etc.).

The goal is to acknowledge within 72 hours and coordinate a fix and disclosure timeline.

Security Scope
This project focuses on:

Correct and safe handling of WofLang programs at the interpreter and runtime level.

Avoiding obvious crashes, memory corruption, or unsandboxed behavior in normal use.

Providing clear boundaries around “forbidden” or experimental features (e.g., chaos/void ops, Easter eggs).

This project does NOT guarantee:

Sandboxing or isolation from your host OS.

Protection against malicious plugins built by third parties.

Hard multi-tenant guarantees or production-grade isolation.

If you run untrusted WofLang code or plugins, do so in a sandboxed environment (VM, container, or dedicated test machine).

Known Limitations
Many plugins are experimental and may assume trusted input.

The C++ build depends on your compiler’s security configuration (ASLR, stack canaries, etc.).

The Rust implementation is early-stage and may evolve in ways that introduce or remove security properties.

Verification
You are encouraged to:

Audit the source code before using WofLang in sensitive contexts.

Build from source using your own toolchain.

Run interpreters and plugins inside containers or VMs when evaluating untrusted scripts.

If you are performing a formal audit and need clarification, please reach out via a private channel first.

