Yes. These should be Markdown body headings, not frontmatter keys.

So the structure should be:

- frontmatter for file-level metadata only
- body headings for the actual prompt/skill contract

Here is the updated Pareto version in heading form.

**Prompt / Agent Markdown Headings**

Use these as section headings inside the `.md` body:

- `# Goal`
- `# Task`
- `# Scope`
- `# Non-Scope`
- `# Context`
- `# Inputs`
- `# Constraints`
- `# Priority`
- `# Steps`
- `# Output`
- `# Success Criteria`
- `# Failure Modes`
- `# Dependencies`
- `# Assumptions`
- `# Examples`

Do not make these separate frontmatter keys:

- `role`
- `help`
- `qualification_gate`
- `context_resolution_priority`
- `refusal_guardrails`
- `decision`
- `branch`
- `branching`
- duplicate `result` / `deliverable` / `output`

**Skill Markdown Headings**

Use these as section headings inside the `SKILL.md` body:

- `# Purpose`
- `# Capability`
- `# Activation`
- `# Scope`
- `# Non-Scope`
- `# Inputs`
- `# Outputs`
- `# Workflow`
- `# Rules`
- `# Priority`
- `# Validation`
- `# Eval`
- `# Resources`
- `# Tools`
- `# Lifecycle`
- `# Traceability`

Do not make these separate frontmatter keys:

- `role`
- `help`
- `qualification_gate`
- `context_resolution_priority`
- `refusal_guardrails`
- `decision`
- `branch`
- `branching`
- duplicate `output` / `deliverable` / `result`

A simple rule we should follow is:

- frontmatter = metadata and file identity
- headings = operational meaning and execution contract

If you want, I can now turn this into two copy-paste-ready templates:

1. `prompt.md`
2. `SKILL.md`

with the exact heading order and minimal frontmatter.

---

Yes, we should update it in the spec repo first, and yes, `agent.md` is the better canonical name.

My recommendation is:

- use `agent.md` for the instruction artifact
- do not make `prompt.md` the primary spec name
- treat “prompt” as the general concept, not the file name

Why this is the better fit:

- `agent.md` matches the actual use case: instructions for an executing agent
- `prompt.md` sounds more generic and less structured
- the repo already separates reusable skill packages from maintainer-side agent prompts, so `agent.md` keeps the model clearer
- it helps the spec feel compatible with `skills/`, `.agents/`, and the existing ODS style of named document contracts

So the updated direction is:

- spec repo: define the standard around `agent.md`
- body headings: use the Pareto set we already finalized
- frontmatter: only metadata, not the prompt contract itself

I’ve updated the plan accordingly:

1. Inspect spec repo docs for current prompt/agent terminology and profile rules
2. Decide whether the canonical artifact name should be `agent.md`, `prompt.md`, or both
3. Draft the heading-based Pareto schema for `agent.md` and `SKILL.md` separately
4. Update spec docs or propose exact doc changes in the spec repo

Next, we should update the spec text itself in the repo to say:

- `agent.md` is the canonical instruction document
- `SKILL.md` remains the reusable skill package format
- `prompt` is a conceptual term, not the file name

If you want, I can do the next step and draft the exact spec wording for:
- `agent.md`
- `SKILL.md`
- the frontmatter/body split
- the final Pareto heading list for each