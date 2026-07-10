#!/usr/bin/env bash
# Assemble a directory of numbered animation frames (as rendered by the
# `animate` binary) into a looping gif.
#
# Usage: make_gif.sh <frames_dir> <output.gif> [fps]

set -euo pipefail

if [[ $# -lt 2 || $# -gt 3 ]]; then
    echo "usage: $0 <frames_dir> <output.gif> [fps]" >&2
    exit 2
fi

frames_dir=$1
output=$2
fps=${3:-12}

# Generate a palette from the frames themselves in a first pass: gifs are
# limited to 256 colors, and a tailored palette avoids visible banding in
# the shading gradients.
ffmpeg -y -loglevel error -framerate "$fps" -i "$frames_dir/frame_%03d.png" \
    -vf "split[a][b];[a]palettegen[palette];[b][palette]paletteuse" \
    -loop 0 "$output"

echo "Wrote $output"
