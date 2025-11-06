#!/bin/bash
# Template for updating progress notes
# Usage: Just tell Claude "update progress notes" and I'll use this pattern

cat <<'TEMPLATE'
## Update .dev-notes.md:
1. Mark completed items with [x]
2. Add new completed items to "Last Session"
3. Update "Next Steps"
4. Add any technical notes/blockers

## Update Apple Notes:
1. Add session date and summary to "Session Log"
2. Update "Resume Here" pointer
3. Mark completed items

## Quick check:
- What did we accomplish?
- What's next?
- Any blockers?
TEMPLATE
