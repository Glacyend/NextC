#!/bin/bash

set -e

PAGES_DIR="./pages"
SITE_DIR="./site"

echo "----- Preparing pages directory..."
rm -rf "$PAGES_DIR"
mkdir -p "$PAGES_DIR"

echo "----- Copying static site files..."
cp -r "$SITE_DIR/." "$PAGES_DIR/"

echo "----- Building English Documentation..."
mdbook build docs -d "$PAGES_DIR/docs/en"

echo "----- Building Chinese Documentation..."
mdbook build docs-zh -d "$PAGES_DIR/docs/zh"

echo "----- Documentation Build Complete!"
