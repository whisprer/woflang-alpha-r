# Security Policy

## Supported Versions
| Version  | Supported  |
|----------|------------|
| 10.1.1   | ✅ Active  |

## Reporting a Vulnerability
If you discover a potential vulnerability:
1. **Do not** open a public GitHub issue immediately.
2. Contact the maintainer privately via:
   - Email: security@whispr.dev
   - Or via GitHub’s “Report a vulnerability” feature.

We aim to respond within **72 hours**.


## Security Scope
This project focuses on interpreter-level secure operation of the woflang stack machine, its core plugin system, and safe execution of Unicode glyph-encoded operations.

It does not claim full isolation from host system vulnerabilities, hardware side-channels, or external threat actors targeting the underlying OS, runtime libraries, or network-connected environments.

Woflang ops manipulate in-memory stacks and invoke native code via plugins—users are responsible for validating input sources, sanitizing external data, and ensuring plugins do not introduce unsafe behavior (e.g., unbounded recursion, memory leaks, or malicious syscalls).

TL;DR: woflang keeps your stack sane and your ops clean, but it won't armor-plate your motherboard or debug your dodgy ISP, bruv.


## Known Limitations

## Verification
Users are encouraged to inspect, audit, and rebuild the source before deployment.

