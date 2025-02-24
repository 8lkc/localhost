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

# Push to all remotes
# This assumes you've set up the 'all' remote as described earlier
# If not, you can push to individual remotes here
git push all master

# If you need to handle the master/main branch difference for GitHub
# Uncomment the line below and comment the line above
# git push origin master && git push github master:main

echo "Changes committed and pushed to all remotes successfully!"