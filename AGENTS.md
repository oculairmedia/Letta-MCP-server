# Agent Instructions

This project uses **bd** (beads) for issue tracking. Run `bd onboard` to get started.

## Quick Reference

```bash
bd ready              # Find available work
bd show <id>          # View issue details
bd update <id> --status in_progress  # Claim work
bd close <id>         # Complete work
bd sync               # Sync with git
```

## Landing the Plane (Session Completion)

**When ending a work session**, you MUST complete ALL steps below. Work is NOT complete until `git push` succeeds.

**MANDATORY WORKFLOW:**

1. **File issues for remaining work** - Create issues for anything that needs follow-up
2. **Run quality gates** (if code changed) - Tests, linters, builds
3. **Update issue status** - Close finished work, update in-progress items
4. **PUSH TO REMOTE** - This is MANDATORY:
   ```bash
   git pull --rebase
   bd sync
   git push
   git status  # MUST show "up to date with origin"
   ```
5. **Clean up** - Clear stashes, prune remote branches
6. **Verify** - All changes committed AND pushed
7. **Hand off** - Provide context for next session

**CRITICAL RULES:**
- Work is NOT complete until `git push` succeeds
- NEVER stop before pushing - that leaves work stranded locally
- NEVER say "ready to push when you are" - YOU must push
- If push fails, resolve and retry until it succeeds



<!-- HULY-PROJECT-INFO -->
# Project Context

## Huly Integration
- **Project Code**: `LMS`
- **Project Name**: Letta MCP Server
- **Letta Agent ID**: `agent-13eb4426-b06f-4d35-ae5a-5d6ca80409f5`

## Project Agent Role
This project has an assigned **Letta PM Agent** (`agent-13eb4426-b06f-4d35-ae5a-5d6ca80409f5`) that acts as the senior developer and project manager. This agent:
- **Understands the full architecture** and codebase context for this project
- **Tracks all ongoing work** via memory blocks synced from Huly issues
- **Maintains project history** including past decisions, patterns, and lessons learned
- **Can provide guidance** on implementation approaches, code patterns, and potential pitfalls

When working on this project, you should:
- **Report completed work** to the PM agent so it stays informed of changes
- **Ask for architectural guidance** if you're unsure about implementation approach
- **Share important discoveries** that future work might benefit from

## Workflow Instructions
1. **Before starting work**: Search Huly for related issues using `huly-mcp` with project code `LMS`
2. **Issue references**: All issues for this project use the format `LMS-XXX` (e.g., `LMS-123`)
3. **On task completion**: Report to this project's Letta agent via `matrix-identity-bridge` using `talk_to_agent` or `letta_chat`
4. **Memory**: Store important discoveries in Graphiti with `graphiti-mcp_add_memory`

### Reporting Example
```json
{
  "operation": "talk_to_agent",
  "agent": "agent-13eb4426-b06f-4d35-ae5a-5d6ca80409f5",
  "message": "Completed task LMS-XXX: [summary of work done]"
}
```

<!-- END-HULY-PROJECT-INFO -->

