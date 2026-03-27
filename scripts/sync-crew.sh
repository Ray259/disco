#!/bin/bash
# sync-crew.sh - Syncs agents from the My-Brain-Is-Full-Crew submodule to local providers

set -e

echo "=> Fetching latest crew submodule..."
git submodule update --remote --merge My-Brain-Is-Full-Crew

echo "=> Creating local provider directories..."
mkdir -p .claude .gemini

echo "=> Syncing Claude agent directories..."
rsync -a --delete My-Brain-Is-Full-Crew/agents/ .claude/agents/
rsync -a --delete My-Brain-Is-Full-Crew/skills/ .claude/skills/
rsync -a --delete My-Brain-Is-Full-Crew/references/ .claude/references/

echo "=> Syncing Gemini agent directories..."
rsync -a --delete My-Brain-Is-Full-Crew/agents/ .gemini/agents/
rsync -a --delete My-Brain-Is-Full-Crew/skills/ .gemini/skills/
rsync -a --delete My-Brain-Is-Full-Crew/references/ .gemini/references/

echo "=> Sync complete! Local .claude and .gemini folders updated."
