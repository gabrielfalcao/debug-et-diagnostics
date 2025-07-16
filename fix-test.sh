#!/usr/bin/env bash
set -e
output=$(2>&1 cargo test -j1 | ack '(thread.*[a-z_]+.rs|left|right):' | head -3 | sed 's/^[[:space:]]*//g')
filename=$(echo "$output" | head -1 | sed 's,^\s*thread.*at\s*\([a-z_]\+/[a-z_]\+[.]rs\):\([0-9]\+\):.*,\1,g')
lineno=$(( $(echo "$output" | head -1 | sed 's,^\s*thread.*at\s*[a-z_]\+/[a-z_]\+[.]rs:\([0-9]\+\):.*,\1,g') + 0 ))
current=$(echo "$output" | tail -1 | sed 's,^.*right:\s*\([[].*[]]\).*$,\1,g')
replace=$(echo "$output" | tail -2 | head -1 | sed 's,^.*left:\s*\([[].*[]]\).*$,\1,g')
regex=$(echo -n "${current}" | sed 's/\([^a-zA-Z0-9_,.;: -]\)/[\1]/g')

if [ ! -e "$filename" ]; then
    1>&2 echo "ERROR: '${filename}' does not exist"
    exit 101
fi
linecount=$(( $(wc -l "$filename" | awk '{ print $1 }') + 0 ))
if [ $lineno -gt $linecount ]; then
    1>&2 echo "ERROR: line number ${lineno} is greater than the line count of '${filename}': ${linecount}"
    exit 101
fi

set -x
sed "${lineno}s/$regex/$replace/" -i "$filename"
#git diff "$filename"
git commit "$filename" -m "fix ${filename} line ${lineno}: \"${current}\" => \"${replace}\""
