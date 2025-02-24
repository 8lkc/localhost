#!/bin/bash

# Check if commit message is provided
if [ $# -eq 0 ]; then
    echo "ERROR: Commit message is required"
    echo "USAGE: ./gitify.sh \"Your commit message here\""
    exit 1
fi

# Store the commit message
COMMIT_MESSAGE="$1"

# Stage all changes
git add .
# Commit with the provided message
git commit -m "$COMMIT_MESSAGE"

# Check if origin exists and push to it
if git remote | grep -q "^origin$"; then
    echo "Pushing to origin..."
    git push origin master
else
    echo "ERROR: origin remote not found"
    exit 1
fi

# Check if github exists and push to it with branch mapping
if git remote | grep -q "^github$"; then
    echo "Pushing to github..."
    git push github master:main
else
    echo "ERROR: github remote not found"
    exit 1
fi

echo "Changes committed successfully!"