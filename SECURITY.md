# Security Policy

## Supported versions

| Version | Supported |
|---------|-----------|
| `main` (v0.1.x early release) | Yes |
| Older commits | No |

## Reporting a vulnerability

**Do not open a public GitHub issue** for undisclosed security problems.

Report privately via [GitHub Security Advisories](https://github.com/unified-field-dev/quark/security/advisories/new) for this repository.

Include:

- Description of the issue and potential impact
- Steps to reproduce (proof-of-concept if available)
- Affected code paths

## Scope

In scope:

- The `quark` crate (`Registrable`, `Registry`, `define_registry!`, `inventory` re-export)

Out of scope:

- Consumer application descriptor contents and registration data
- Third-party `inventory` crate internals

## Response

This is an early-release project with no formal SLA. Reports will be acknowledged in a reasonable timeframe. Fixes may land on `main` and be documented in advisory release notes when applicable.
