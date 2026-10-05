Advertising, commercial promotion, and traffic solicitation are prohibited in PRs
and Issues. Business requests must go through the email in the
[contribution policy](https://github.com/Kuddev/pebrel/blob/main/CONTRIBUTING.md#commercial-promotion-policy).
PRs or Issues containing unauthorized promotion will be closed directly.

## Result

What changes for users? Link the relevant issue; do not claim unfinished work.

## Design

- Responsibility and affected modules:
- Why this belongs here; interfaces that remain unchanged:
- Dependency, data-format, threading, or lifetime changes (ADR if applicable):
- Compatibility and migration/fallback behavior:

## Evidence

- Commands and actual results (include unrun checks):
- Regression tests: what fails before the fix?
- UI changes: screenshots, long translations, keyboard access, DPI checks:
- Hot-path changes: allocation/work/load measurements, where applicable:

## Required Review

- [ ] I followed `CONTRIBUTING.md`, `docs/architecture.md`, and `docs/project-constraints.md`.
- [ ] I split responsibilities, not arbitrary line ranges; no duplicate behavior authority was added.
- [ ] `python3 scripts/check_architecture.py --base <PR-base-commit>` passes; budgets were not inflated to fit the change.
- [ ] Tests cover success and failure; platform/feature coverage limitations are stated.
- [ ] New messages use typed i18n IDs and matching placeholders; untranslated content has an explicit fallback.
- [ ] Governance changes include a counterexample, corrected contract, tests, and a maintainer-reviewed decision.

Checkboxes explain the review; they do not replace CI or maintainer approval.
