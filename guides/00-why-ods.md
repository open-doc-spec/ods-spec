---
description: "Why ODS exists, what it is (and is not), when to adopt it, and where it lives in a Git repository."
tags:
  - learn
  - ods
  - onboarding
owner: team:ods
ods:
  profile: guide
  status: stable
  related:
    - README.md
    - 01-first-document.md
    - faq.md
    - ../specs/scope.md
---

# Why ODS Exists

## Overview

Open Document Spec (ODS) is a convention for plain Markdown files in a Git repository so **people** and **AI agents** can find, trust, and update the same docs.

It is not a CMS, not a new file extension, and not a database. Your files stay `.md`. You add a short YAML block when structure pays rent, and a root `ods.toml` when you want the folder treated as one workspace.

Three words you need on this page:

| Word | Meaning |
| :--- | :--- |
| **Document** | Any `.md` file. Frontmatter is optional. |
| **Frontmatter** | The optional YAML block between `---` lines at the top of a file. |
| **Workspace** | A folder tree whose root contains `ods.toml`. |

The rest of the vocabulary (`profile`, `depends`, `context`, …) waits until you need it.

## Prerequisites

- You keep engineering or product docs in Git, or you want to.
- You can edit a Markdown file in any text editor.
- You do **not** need the `ods` CLI to understand this page.

## Steps

### 1. See the problem ODS is for

A typical docs folder starts honest and then drifts:

```text
docs/
├── refunds.md          # outdated steps, no owner
├── auth.md             # "see refunds" in prose; the link is wrong
├── checkout.md         # mentions src/checkout.ts:L45 — that line moved
└── architecture.pdf    # nobody knows which doc it belongs to
```

What goes wrong:

- **Humans** cannot tell which doc is current, which one to read first, or who owns it.
- **Links rot** because relationships live in sentences, not in something a tool can check.
- **AI tools** dump the whole folder into a prompt, blow the token budget, and still miss the one prerequisite that mattered.

ODS does not invent a new knowledge app. It makes the missing facts **explicit and checkable** inside the files you already have.

### 2. Answer the five questions

| Question | Answer |
| :--- | :--- |
| **What** | A convention: optional YAML frontmatter on standard `.md` files, plus a root `ods.toml` that marks the workspace. |
| **Why** | So identity, ownership, document shape, prerequisites, and AI reading lists are written down once and lintable in CI. |
| **When** | You maintain architecture notes, runbooks, ADRs, PRDs, onboarding guides, or agent instructions next to code. |
| **Where** | In the Git repo, beside the code. The same files render on GitHub and in Hugo, Astro, Docusaurus, Next.js, or Obsidian. |
| **How** | Write normal Markdown. Add frontmatter only for facts a tool should enforce. Run `ods lint` when you want a pass/fail check. |

### 3. See the same folder after ODS

```text
ods.toml                      # this tree is an ODS workspace
docs/
├── guides/refunds.md         # profile: guide; depends on auth/sessions.md
├── auth/sessions.md          # the prerequisite, not a prose "see also"
└── decisions/004-redis.md    # profile: decision; four expected headings
```

Nothing left the repo. No `.ods` extension. The PDF, if you still have one, is listed on the doc that uses it — verified on disk, not stuffed into an AI prompt.

### 4. Know when *not* to adopt it

Skip ODS when:

- The site is marketing copy with no relationships to enforce.
- Another tool already owns a closed schema you cannot coexist with.
- You only have a handful of one-off notes and no plan to validate them.

Plain Markdown without frontmatter is already valid. ODS is additive. You can put `ods.toml` in a repo full of untouched `.md` files and enrich documents one at a time.

### 5. Remember where each fact will live (preview)

You will use three places, never more:

| Place | Holds |
| :--- | :--- |
| **Body** (`#` / `##`) | Title and the human explanation. |
| **Frontmatter** | Machine facts: summary, tags, shape, links, attachments. |
| **`ods.toml`** | Workspace-wide settings, not per-document facts. |

[Your first document](01-first-document.md) puts this to work in about fifteen minutes.

## Troubleshooting

- **"Is this a Static Site Generator?"** No. ODS does not render HTML. Keep Hugo, Astro, or Docusaurus. Unknown frontmatter keys (`layout`, `hero_image`, `permalink`) stay untouched.
- **"Do I have to rewrite existing docs?"** No. Add `ods.toml`, then add frontmatter to the files that benefit. See [Your first document](01-first-document.md).
- **"Where is the full rulebook?"** [`specs/README.md`](../specs/README.md) is the reference. Do not start there unless you are implementing a parser.

**You can stop here** if you only needed the idea.

**Next only if** you want to write one real ODS document: [01 · Your first document](01-first-document.md).
