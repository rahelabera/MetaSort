#!/bin/bash

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Open a new terminal window and run MetaSort
osascript << EOSCRIPT
tell application "Terminal"
    do script "cd '$SCRIPT_DIR' && ./target/release/MetaSort"
    activate
end tell
EOSCRIPT
