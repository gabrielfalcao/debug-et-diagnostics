#!/usr/bin/env bash
set -e
test_name="test_color"

output=$(2>&1 cargo test -j1 --test "$test_name" | ack '((thread|[-][-][>]).*[a-z_]+[.]rs|left|right):' | head -3 | sed 's/^[[:space:]]*//g')
filename=$(echo "$output" | head -1 | sed 's,^.*\?\(thread.*at\|[-][-][>]\)\s*\([a-z_]\+/[a-z_]\+[.]rs\):\([0-9]\+\):.*,\2,g')
lineno=$(( $(echo "$output" | head -1 | sed 's,^.*\?\(thread.*at\|[-][-][>]\)\s*[a-z_]\+/[a-z_]\+[.]rs:\([0-9]\+\):.*,\2,g') + 0 ))
next_lineno=$(( $lineno + 1 ))
current=$(echo "$output" | tail -1 | sed 's,^.*right:\s*\([0-9]\+\).*$,\1,g')
replace=$(echo "$output" | tail -2 | head -1 | sed 's,^.*left:\s*\([0-9]\+\).*$,\1,g' | sed 's/\\/\\\\/g' | sed 's/\([<>/]\)/\\\1/g')
regex="$(echo -n "${current}" | sed 's/\([^a-zA-Z0-9]\)/[\1]/g')[)][;]"
replace="${replace});"
# # # # echo -e "\x1b[1;48;5;202m\x1b[1;38;5;16moutput=\x1b[0m\x1b[1;38;5;202m${output}\x1b[0m"
# # # # exit
error_filename=fix-test-error.sed
# echo -e "\x1b[1;48;5;202m\x1b[1;38;5;16mfilename=\x1b[0m\x1b[1;38;5;82m${filename}\x1b[0m"
# echo -e "\x1b[1;48;5;202m\x1b[1;38;5;16mlineno=\x1b[0m\x1b[1;38;5;71m${lineno}\x1b[0m"
# echo -e "\x1b[1;48;5;202m\x1b[1;38;5;16mcurrent=\x1b[0m\x1b[1;38;5;206m${current}\x1b[0m"
# echo -e "\x1b[1;48;5;202m\x1b[1;38;5;16mreplace=\x1b[0m\x1b[1;38;5;33m${replace}\x1b[0m"
# echo -e "\x1b[1;48;5;231m\x1b[1;38;5;16mregex=\x1b[0m\x1b[1;38;5;202m${regex}\x1b[0m"
# exit

if [ ! -e "$filename" ]; then
    1>&2 echo "ERROR: '${filename}' does not exist"
    exit 101
fi
rm -f "${error_filename}"

linecount=$(( $(wc -l "$filename" | awk '{ print $1 }') + 0 ))
if [ $lineno -gt $linecount ]; then
    1>&2 echo "ERROR: line number ${lineno} is greater than the line count of '${filename}': ${linecount}"
    exit 101
fi

expression="${lineno}s/$regex/$replace/"
1>&2 echo "sed \"${expression}\" -i \"$filename\""
if 2>"${error_filename}" sed "${expression}" -i "$filename"; then
    if 2>/dev/random cargo test -j1 --test "$test_name"; then
        # git diff "$filename"
        rm -f "$error_filename"
        git commit "$filename" -m "fix ${filename} line ${lineno}, such that \"${current}\" becomes \"${replace}\""
        expression="${next_lineno}s/^\(\s*\)\/\/\s*\(assert_[a-z_]\+!.*;\)/\1\2/"
        if 2>"${error_filename}" sed "${expression}" -i "$filename"; then
            rm -f "${error_filename}"
            exit 0
        else
            git restore "$filename"
            1>&2 echo -e "\x1b[1;48;5;160m\x1b[1;38;5;231mERROR:\t\x1b[1;48;5;231m\x1b[1;38;5;160m failed to uncomment \x1b[1;38;5;33m${filename}\x1b[0m\x1b[1;48;5;231m\x1b[1;38;5;16m line \x1b[1;38;5;28m${lineno}             \x1b[0m"
            exit 101
        fi
    else
        echo -e "\n\x1b[1;48;5;160m\x1b[1;38;5;231m                                                             \x1b[0m"
        echo -e "\x1b[1;48;5;160m\x1b[1;38;5;231mERROR:\t\x1b[1;48;5;231m\x1b[1;38;5;160mtests failed                                         \x1b[0m"
        echo -e "\x1b[0m\x1b[1;48;5;231m\x1b[1;38;5;16mtry manually fixing \x1b[1;38;5;33m${filename}\x1b[0m\x1b[1;48;5;231m\x1b[1;38;5;16m line \x1b[1;38;5;28m${lineno}             \x1b[0m"
        echo -e "\x1b[1;48;5;160m\x1b[1;38;5;231m                                                             \x1b[0m"
        exit 101
    fi
else
    echo -en "\x1b[1;38;5;231m"
    char=$(cat "${error_filename}" | sed 's/^sed:.*\?expression.*\?char\s*\([0-9]\+\).*/\1/g')
    charbefore=$(( $char - 1 ))
    echo -en "\x1b[1;48;5;231m\x1b[1;38;5;160merror in \x1b[1;48;5;231m\x1b[4m\x1b[1;38;5;202mexpression\x1b[24m\x1b[1;48;5;231m\x1b[1;38;5;160m char:\x1b[1;48;5;160m\x1b[1;38;5;231m $char \x1b[0m: "
    echo -n "$expression" | sed "s/^\(.\{$charbefore\}\)\(.\)\(.*\)/\\x1b[4m\\x1b[1;48;5;202m\\x1b[1;38;5;16m\2\\x1b[0m\\n/g"
    echo -n "$expression" | sed "s/^\(.\{$charbefore\}\)\(.\)\(.*\)/\\x1b[1;48;5;16m\\x1b[1;38;5;231m\1\\x1b[5m\\x1b[4m\\x1b[1;48;5;202m\\x1b[1;38;5;16m\2\\x1b[25m\\x1b[24m\\x1b[1;48;5;16m\\x1b[1;38;5;231m\3/"
    echo -en "\nchars: "
    echo -n "$expression" | wc -c
    echo
    # cat "${error_filename}" | sed "s/^\(.\)\{,\}"
    # 1>&2 echo "failed to fix file \"${filename}\" line ${lineno}"
    exit 101
fi
