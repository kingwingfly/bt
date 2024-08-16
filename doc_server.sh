#! /bin/bash
set -e

echo "http://localhost:8000/bep_0003.html"
echo "http://localhost:8000/bep_0005.html"

python -m http.server -d doc
