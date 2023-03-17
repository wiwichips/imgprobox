#!/usr/bin/env sh
./build.sh && echo "Build successful, now serving files" && echo "→ http://localhost:8000/" && python3 -m http.server
