---
description: "How non-Markdown resources and source code bindings are mapped, validated, and linked to documentation in ODS."
ods:
  profile: "note"
  status: "stable"
  depends:
    - README.md
    - keys.md
  related:
    - context.md
    - validation.md
    - core.md
    - ../guides/04-bind-code-and-files.md
---

# ODS · Assets & Code Bindings

This document specifies how **Assets**—comprising non-Markdown **resources** and source **code** bindings—are attached to documentation in Open Document Spec (ODS), how they interact with AI prompts, and why line numbers are prohibited.

## At a glance

- **What this chapter defines:** `ods.resources` vs `ods.code` vs `ods.context.load`, the 8 code roles, and the ban on line numbers.
- **Why it exists:** Attachments that look similar (a PNG, a `.ts` file, a JSON schema) must not be treated the same in a prompt.
- **When you need it:** You are binding implementation or cataloging files on disk.
- **When you can skip it:** Documents that do not point at files or source.
- **Learn this first:** [Bind files and code](../guides/04-bind-code-and-files.md)
- **Prerequisite chapters:** [keys.md](keys.md)

---

## 1. Conformance Language

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **NOT RECOMMENDED**, **MAY**, and **OPTIONAL** in this document are to be interpreted as described in BCP 14 ([RFC 2119](https://www.rfc-editor.org/rfc/rfc2119.txt), [RFC 8174](https://www.rfc-editor.org/rfc/rfc8174.txt)) when, and only when, they appear in all capitals.

---

## 2. What are Assets?

Assets are attachments that connect human-readable prose in Markdown to concrete artifacts on disk:
1. **`ods.resources`**: Non-Markdown data files (PDF reports, architecture diagrams, sample CSVs, OpenAPI specifications).
2. **`ods.code`**: Implementation source files, test fixtures, infrastructure manifests, and CI/CD pipelines.

```text
┌─────────────────────────────────────────────────────────────────────────┐
│ Markdown Document (Source of Truth)                                     │
│ ---                                                                     │
│ ods:                                                                    │
│   # 1. Asset Catalog (Disk-level files verified by 'ods lint')          │
│   resources:                                                            │
│     - path: ../diagrams/auth.png        → PNG Diagram on disk           │
│                                                                         │
│   # 2. Code Bindings (Semantic links to implementation)                 │
│   code:                                                                 │
│     - path: src/auth.ts                                                 │
│       role: implementation              → TypeScript Logic              │
│       symbol: verifySession                                             │
│ ---                                                                     │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 3. When to Use: `resources` vs `code` vs `context.load`

| Need | Declaration | Phase / Behavior | Why |
| :--- | :--- | :--- | :--- |
| **Architecture diagram / image** | `ods.resources` | Verified by `ods lint`; **NOT** loaded into prompt | Binary image; loading it would waste LLM prompt tokens. |
| **Full PDF specification / report**| `ods.resources` | Verified by `ods lint`; **NOT** loaded into prompt | Large binary file; human reference only. |
| **Small JSON schema / mock CSV** | `ods.context.load` | Verified by `ods lint`; **INJECTED** into prompt | Structured text data the AI agent needs to inspect. |
| **API route / HTTP handler** | `ods.code` (`role: entrypoint`) | Verified by `ods lint`; Included when `--with-code` | Marks where execution starts. |
| **Core business logic** | `ods.code` (`role: implementation`) | Verified by `ods lint`; Included when `--with-code` | Marks domain functions. |
| **Unit or integration test** | `ods.code` (`role: test`) | Verified by `ods lint`; Included when `--with-code` | Test suite verifying document requirements. |
| **Database migration script** | `ods.code` (`role: migration`) | Verified by `ods lint`; Included when `--with-code` | Persistent state transition script. |
| **Terraform / Cloud manifest** | `ods.code` (`role: infrastructure`) | Verified by `ods lint`; Included when `--with-code` | Cloud resource definitions. |

---

## 4. The Binary Asset Token Budget Problem

A common mistake in AI tooling is automatically dumping all document attachments into the LLM context window:

```text
┌─────────────────────────────────────────────────────────────────────────┐
│ DUMPING ALL RESOURCES (Naïve Tooling Failure):                          │
│ • 'resources' contains:                                                 │
│   - network-diagram.png (4.2 MB)                                        │
│   - compliance-audit-2025.pdf (48 MB)                                   │
│ • Result: LLM prompt exceeds 128k token context window instantly!      │
├─────────────────────────────────────────────────────────────────────────┤
│ THE ODS SOLUTION (Surgical Separation):                                 │
│ 1. 'ods.resources' is an Asset Catalog: 'ods lint' verifies files exist │
│    on disk for human readers, but NEVER passes them to AI prompts.      │
│ 2. 'ods.context.load' is the Prompt Scoping Key: Authors explicitly     │
│    declare lightweight JSON schemas, CSVs, or configs for the LLM.     │
└─────────────────────────────────────────────────────────────────────────┘
```

---

## 5. Non-Markdown Resources (`ods.resources`)

The `ods.resources` list captures non-Markdown attachments without transforming their native format:

```yaml
ods:
  resources:
    - path: ../diagrams/network-topology.svg   # Vector diagram for human readers
    - path: ../reports/q3-audit.pdf           # Audit report attachment
    - path: ../contracts/billing.openapi.yaml # OpenAPI specification on disk
```

### Normative Rules:
1. Every entry MUST be a mapping containing a `path` string.
2. `path` MUST be a relative path resolved from the document's directory location.
3. The referenced resource MUST exist on disk. A non-existent resource path is dangling and MUST trigger a validation error.
4. Source code files MUST NOT be declared under `resources`; they MUST be declared under `ods.code`.

---

## 6. Source Code Bindings (`ods.code`)

The `ods.code` mapping creates a verifiable, bidirectional bridge between architectural prose and software implementation.

### 6.1 Code Files are NOT ODS Documents
- Source code files (`.ts`, `.rs`, `.py`, `.go`, `.tf`) MUST NOT contain ODS frontmatter.
- Source code files are NOT graph nodes and MUST NOT be indexed as documents.
- The Markdown document remains the single source of truth for all code bindings.

### 6.2 Code Binding Schema & Roles
Each entry in `ods.code` MUST satisfy the following schema:

```yaml
ods:
  code:
    # 1. Entrypoint: Where execution begins
    - path: apps/api/src/routes/refunds.ts
      role: entrypoint
      symbol: HandleRefundPost

    # 2. Implementation: Domain logic & core algorithms
    - path: apps/api/src/services/refunds.ts
      role: implementation
      symbol:
        - executeRefund
        - calculateTaxes

    # 3. Test: Automated test suites and fixtures
    - path: apps/api/tests/refunds.test.ts
      role: test
      symbol: TestRefundFlow

    # 4. Schema: Type definitions & data models
    - path: packages/db/prisma/schema.prisma
      role: schema
      symbol: RefundTransaction

    # 5. Migration: Database state transitions
    - path: packages/db/migrations/20260115_add_refunds.sql
      role: migration

    # 6. Config: Feature flags & environment settings
    - path: apps/api/config/flags.toml
      role: config
      symbol: EnableRefundV2

    # 7. Infrastructure: Cloud provisioning & Kubernetes
    - path: infra/terraform/refund_queue.tf
      role: infrastructure

    # 8. Pipeline: CI/CD automation & release workflows
    - path: .github/workflows/deploy-billing.yml
      role: pipeline
```

---

## 7. The 8 Standard Code Roles Reference

| Role | Semantic Meaning | Common File Types | Typical Symbols Linked |
| :--- | :--- | :--- | :--- |
| **`entrypoint`** | Where execution begins: HTTP route handler, CLI command, event consumer, or UI view. | `.tsx`, `.ts`, `.rs`, `.go`, `.py` | `RefundRoute`, `main`, `handleCheckout` |
| **`implementation`** | Core business logic, algorithm, or domain service. | `.ts`, `.rs`, `.py`, `.go`, `.java` | `calculateTotal`, `processPayment` |
| **`test`** | Automated tests, mock suites, and test fixtures. | `.test.ts`, `_test.go`, `test_*.py` | `TestRefundFlow`, `test_tax_calculation` |
| **`schema`** | Data models, type definitions, protobufs, OpenAPI schemas, Zod schemas. | `.prisma`, `.proto`, `.sql`, `.d.ts` | `UserSchema`, `PaymentIntentModel` |
| **`migration`** | State transitions: database migration scripts, data backfills. | `.sql`, `.ts` (Prisma/Flyway/Diesel) | `V003_add_refund_status.sql` |
| **`config`** | Runtime settings, feature flag definitions, build configs. | `.toml`, `.json`, `.yaml`, `.env.example`| `FeatureFlags`, `redisConfig` |
| **`infrastructure`**| Cloud provisioning, Terraform modules, Helm charts, Kubernetes specs. | `.tf`, `.yaml` (K8s), `.jsonnet` | `aws_rds_cluster`, `payment_queue` |
| **`pipeline`** | CI/CD automation, release workflows, Dockerfiles, build scripts. | `.github/workflows/*.yml`, `Dockerfile` | `deploy-prod`, `build-image` |

---

## 8. Why Line Numbers are Strictly Forbidden

```text
┌─────────────────────────────────────────────────────────────────────────┐
│ FRAGILE (Prohibited):                                                   │
│ path: src/pricing.ts:L45-L60                                            │
│ -> A developer inserts 2 lines of imports at the top of pricing.ts.     │
│    Every line number in the documentation is instantly broken and stale.│
├─────────────────────────────────────────────────────────────────────────┤
│ RESILIENT & REFACTOR-SAFE (Mandated by ODS):                            │
│ path: src/pricing.ts                                                    │
│ symbol: calculateDiscount                                               │
│ -> Language-aware symbols survive line insertions, formatting changes,  │
│    and routine refactoring without documentation drift.                 │
└─────────────────────────────────────────────────────────────────────────┘
```

- Tools MUST emit a validation error if any `ods.code[].path` contains a line number suffix (such as `:L45` or `#L10-L20`).
- Use the `symbol` field for precise symbol-level linking.

---

## 9. Design Decisions

### Why a closed enum of 8 code roles instead of custom user-defined roles?
A closed taxonomy of 8 standard code roles ensures that external AI coding agents, linters, and analysis tools can reliably classify code without needing custom project-specific parser rules. Every software artifact naturally falls into one of the 8 universal roles.

---

## Navigation & Reading Order

| [← Previous Chapter](context.md) | [📑 Specification Index](README.md) | [Next Chapter →](indexes.md) |
| :--- | :---: | ---: |
| **06. Bounded AI Context Scope** | **Open Document Spec (ODS)** | **08. Workspace Config & Progressive Discovery** |
