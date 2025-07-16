#!/usr/bin/env bash
set -e
output=$(2>&1 cargo test -j1 | ack '(thread.*[a-z_]+.rs|left|right):' | head -3 | sed 's/^[[:space:]]*//g')
filename=$(echo "$output" | head -1 | sed 's,^\s*thread.*at\s*\([a-z_]\+/[a-z_]\+[.]rs\):\([0-9]\+\):.*,\1,g')
lineno=$(( $(echo "$output" | head -1 | sed 's,^\s*thread.*at\s*[a-z_]\+/[a-z_]\+[.]rs:\([0-9]\+\):.*,\1,g') + 0 ))
current=$(echo "$output" | tail -1 | sed 's,^.*right:\s*\(".*"\).*$,\1,g')
replace=$(echo "$output" | tail -2 | head -1 | sed 's,^.*left:\s*\(".*"\).*$,\1,g' | sed 's/\\/\\\\/g' | sed 's/\([<>/]\)/\\\1/g')
regex="$(echo -n "${current}" | sed 's/\([^a-zA-Z0-9]\)/[\1]/g')[)]"

# # echo -e "\x1b[1;48;5;202m\x1b[1;38;5;16moutput=\x1b[0m\x1b[1;38;5;202m${output}\x1b[0m"
# # exit
echo -e "\x1b[1;48;5;202m\x1b[1;38;5;16mfilename=\x1b[0m\x1b[1;38;5;82m${filename}\x1b[0m"
echo -e "\x1b[1;48;5;202m\x1b[1;38;5;16mlineno=\x1b[0m\x1b[1;38;5;71m${lineno}\x1b[0m"
echo -e "\x1b[1;48;5;202m\x1b[1;38;5;16mcurrent=\x1b[0m\x1b[1;38;5;206m${current}\x1b[0m"
echo -e "\x1b[1;48;5;202m\x1b[1;38;5;16mreplace=\x1b[0m\x1b[1;38;5;33m${replace}\x1b[0m"
echo -e "\x1b[1;48;5;231m\x1b[1;38;5;16mregex=\x1b[0m\x1b[1;38;5;202m${regex}\x1b[0m"
# exit

if [ ! -e "$filename" ]; then
    1>&2 echo "ERROR: '${filename}' does not exist"
    exit 101
fi
linecount=$(( $(wc -l "$filename" | awk '{ print $1 }') + 0 ))
if [ $lineno -gt $linecount ]; then
    1>&2 echo "ERROR: line number ${lineno} is greater than the line count of '${filename}': ${linecount}"
    exit 101
fi

expression="${lineno}s/$regex/$replace/"

1>&2 echo "sed \"${lineno}s/$regex/$replace/\" -i \"$filename\""
if 2>fix-test-error.sed sed "${lineno}s/$regex/$replace/" -i "$filename"; then
    # git diff "$filename"
    git commit "$filename" -m "fix ${filename} line ${lineno}, such that \"${current}\" becomes \"${replace}\""
else
    echo -en "\x1b[1;38;5;231m"
    char=$(cat fix-test-error.sed | sed 's/^sed:.*\?expression.*\?char\s*\([0-9]\+\).*/\1/g')
    charbefore=$(( $char - 1 ))
    echo -en "\x1b[1;48;5;231m\x1b[1;38;5;160merror in \x1b[1;48;5;231m\x1b[4m\x1b[1;38;5;202mexpression\x1b[24m\x1b[1;48;5;231m\x1b[1;38;5;160m char:\x1b[1;48;5;160m\x1b[1;38;5;231m $char \x1b[0m: "
    echo -n "$expression" | sed "s/^\(.\{$charbefore\}\)\(.\)\(.*\)/\\x1b[4m\\x1b[1;48;5;202m\\x1b[1;38;5;16m\2\\x1b[0m\\n/g"
    echo -n "$expression" | sed "s/^\(.\{$charbefore\}\)\(.\)\(.*\)/\\x1b[1;48;5;16m\\x1b[1;38;5;231m\1\\x1b[5m\\x1b[4m\\x1b[1;48;5;202m\\x1b[1;38;5;16m\2\\x1b[25m\\x1b[24m\\x1b[1;48;5;16m\\x1b[1;38;5;231m\3/"
    echo -en "\nchars: "
    echo -n "$expression" | wc -c
    echo
    # cat fix-test-error.sed | sed "s/^\(.\)\{,\}"
    # 1>&2 echo "failed to fix file \"${filename}\" line ${lineno}"
    exit 101
fi
