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

# Check if 'all' remote exists
if git remote | grep -q "^all$"; then
    # 'all' remote exists, push to it
    git push all master
else
    # 'all' remote doesn't exist, check for origin and github
    echo "ERROR: 'all' remote not configured. Pushing to individual remotes..."
    echo "USAGE:\n\t\"git remote add all origin\"\n\"git remote set-url --add all <URL_of_mirror_repository>\""
fi

echo "Changes committed successfully!"
