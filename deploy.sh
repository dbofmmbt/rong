#!/bin/bash

set -e

trunk build

HASH=$(git log --pretty=format:'%h' -n 1)

git switch github-pages

rm index*
mv dist/* .
rm -r dist
git checkout main -- assets

git add -A
git commit -m "deploy version $HASH"
git push

git switch main
