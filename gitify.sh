#!/bin/bash

# Check if commit message is provided
if [ $# -eq 0 ]; then
    echo "Error: Commit message is required"
    echo "Usage: ./gitify.sh \"Your commit message here\""
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
    echo "'all' remote not configured. Pushing to individual remotes..."
    
    # Check if origin exists and push to it
    if git remote | grep -q "^origin$"; then
        echo "Pushing to origin..."
        git push origin master
    fi
    
    # Check if github exists and push to it with branch mapping
    if git remote | grep -q "^github$"; then
        echo "Pushing to github..."
        git push github master:main
    fi
    
    echo "Consider setting up an 'all' remote with:"
    echo "git remote add all origin"
    echo "git remote set-url --add all https://github.com/8lkc/localhost.git"
fi

echo "Changes committed successfully!"