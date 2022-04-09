#!/bin/bash

echo "Starting build..."
trunk build

HASH=$(git log --pretty=format:'%h' -n 1)

echo "Deploying..."

git switch github-pages

mv build/* .
rm -r build

git add -A
git commit -m "deploy version $HASH"

git switch main
echo "Done!"