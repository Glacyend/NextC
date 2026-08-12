#!/bin/bash

set -e

echo "----- Building English Documentation..."
mdbook build docs -d ./pages/docs/en

echo "----- Building Chinese Documentation..."
mdbook build docs-zh -d ./pages/docs/zh

echo "----- Documentation Build Complete!"
