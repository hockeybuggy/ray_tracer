#!/bin/bash

echo "Checking the gallery in the README for completeness..."
any_missing=0

for filename in tests/fixtures/*.png; do
    if grep --quiet -i $filename README.md; then
        echo "✅ found: $filename"
    else
        echo "❌ could not find: $filename"
        any_missing=1
    fi
done

echo ""
echo ""

if [ $any_missing -eq 1 ]; then
    echo "❌ could not find all images in the fixtures found in the Gallery."
    exit 1
fi

echo "✅ all images in the fixtures found in the Gallery."
exit 0
