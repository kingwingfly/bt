#! /bin/bash
set -e

echo "http://localhost:8000/"

python -m http.server -d bittorrent.org
