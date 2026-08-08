# ods-spec

Normative dialect documentation for Open Document Spec (ODS), Google OKF, and Agent Skills keys as used by the `ods` CLI.

First-cut extract from monorepo `specs/`.

## Layout

| Path | Dialect |
|------|---------|
| `specs/ods/` | Open Document Spec — start at `intro.md` + `keys.md` |
| `specs/okf/` | OKF v0.2 keys/intro |
| `specs/skills/` | Agent Skills keys/intro |

## Source of truth

| Concern | Repo |
|---------|------|
| Spec markdown (this tree) | **This repo** (SoT after merge) |
| Runtime key schema (Rust) | [open-doc-spec/ods](https://github.com/open-doc-spec/ods) `src/ods-core/src/spec/schema.rs` |
| Engine / CLI | [open-doc-spec/ods](https://github.com/open-doc-spec/ods) |

## First-cut policy

Monorepo still mirrors `specs/` for local workspace demos and tests until a later hard-delete PR.
