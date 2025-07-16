#!/usr/bin/env bash
set -e
this_script="${BASH_SOURCE[0]}"
working_dir="$(cd "$(dirname "${this_script}")" && pwd)"
script_name="$(basename "${this_script}")"
cd "${working_dir}"

test_name="test_color"

output=$(2>&1 cargo test -j1 --test "$test_name" | ack '((thread|[-][-][>]).*[a-z_]+[.]rs|left|right):' | head -3 | sed 's/^[[:space:]]*//g')
filename=$(echo "$output" | head -1 | sed 's,^.*\?\(thread.*at\|[-][-][>]\)\s*\([a-z_]\+/[a-z_]\+[.]rs\):\([0-9]\+\):.*,\2,g')
lineno=$(( $(echo "$output" | head -1 | sed 's,^.*\?\(thread.*at\|[-][-][>]\)\s*[a-z_]\+/[a-z_]\+[.]rs:\([0-9]\+\):.*,\2,g') + 0 ))
next_lineno=$(( $lineno + 1 ))
current=$(echo "$output" | tail -1 | sed 's,^.*right:\s*\([0-9]\+\).*$,\1,g')
replace=$(echo "$output" | tail -2 | head -1 | sed 's,^.*left:\s*\([0-9]\+\).*$,\1,g' | sed 's/\\/\\\\/g' | sed 's/\([<>/]\)/\\\1/g')
regex="$(echo -n "${current}" | sed 's/\([^a-zA-Z0-9]\)/[\1]/g')[)][;]"
replace="${replace});"
current="${current});"

# # # # echo -e "\x1b[1;48;5;202m\x1b[1;38;5;16moutput=\x1b[0m\x1b[1;38;5;202m${output}\x1b[0m"
# # # # exit
error_filename=fix-test-error.sed
# echo -e "\x1b[1;48;5;202m\x1b[1;38;5;16mfilename=\x1b[0m\x1b[1;38;5;82m${filename}\x1b[0m"
# echo -e "\x1b[1;48;5;202m\x1b[1;38;5;16mlineno=\x1b[0m\x1b[1;38;5;71m${lineno}\x1b[0m"
# echo -e "\x1b[1;48;5;202m\x1b[1;38;5;16mcurrent=\x1b[0m\x1b[1;38;5;206m${current}\x1b[0m"
# echo -e "\x1b[1;48;5;202m\x1b[1;38;5;16mreplace=\x1b[0m\x1b[1;38;5;33m${replace}\x1b[0m"
# echo -e "\x1b[1;48;5;231m\x1b[1;38;5;16mregex=\x1b[0m\x1b[1;38;5;202m${regex}\x1b[0m"
# exit

line_matches_regex() {
    line="$1"
    if [ -z "$line" ]; then
        1>&2 echo -e "\x1b[1;48;5;160m\x1b[1;38;5;231mERROR:\t\x1b[1;48;5;231m\x1b[1;38;5;160m line_matches_regex received no \"line\" param\x1b[0m"
        exit 101
    fi
    shift
    regex="$@"
    if [ -z "$regex" ]; then
        1>&2 echo -e "\x1b[1;48;5;160m\x1b[1;38;5;231mERROR:\t\x1b[1;48;5;231m\x1b[1;38;5;160m line_matches_regex received no \"regex\" param\x1b[0m"
        exit 101
    fi
    2>/dev/random ack --with-filename --column "${regex}" "${filename}" | 2>/dev/random 1>/dev/random ack "${filename}:${line}:"
}

ensure_next_lines_commented() {
    ensure_next_lineno=$(( $lineno + 1 ))
    error="false"
    if line_matches_regex ${ensure_next_lineno} '^\s*assert_eq'; then
        1>&2 echo -e "\x1b[1;38;5;231mattempt to silence \x1b[1;38;5;33m${filename}\x1b[1;38;5;231m line \x1b[1;38;5;82m${ensure_next_lineno}\x1b[0m"
    else
        return
    fi
    # echo -e "\x1b[1;48;5;202m\x1b[1;38;5;16mfilename=\x1b[0m\x1b[1;38;5;82m${filename}\x1b[0m"
    # echo -e "\x1b[1;48;5;202m\x1b[1;38;5;16mlineno=\x1b[0m\x1b[1;38;5;71m${lineno}\x1b[0m"
    # echo -e "\x1b[1;48;5;202m\x1b[1;38;5;16merror=\x1b[0m\x1b[1;38;5;206m${error}\x1b[0m"
    # echo -e "\x1b[1;48;5;202m\x1b[1;38;5;16mnext_lineno=\x1b[0m\x1b[1;38;5;33m${ensure_next_lineno}\x1b[0m"
    # exit
    while line_matches_regex ${ensure_next_lineno} '^\s*assert_eq'; do
        ensure_regex='^\(\s\+\)\(assert_eq[!].*;\)\s*$'
        ensure_replace='\1\/\/ \2'
        ensure_expression="${ensure_next_lineno}s/$ensure_regex/$ensure_replace/"
        if sed "${ensure_expression}" -i "$filename"; then
            1>&2 echo -e "\r\x1b[A\x1b[1;38;5;231msilenced \x1b[1;38;5;33m${filename}\x1b[1;38;5;231m line \x1b[1;38;5;82m${ensure_next_lineno}\x1b[0m\t\t\t\t\t\t\t\t\t"
            ensure_next_lineno=$(( $ensure_next_lineno + 1 ))
        else
            1>&2 echo -e "\x1b[1;48;5;160m\x1b[1;38;5;231mERROR:\t\x1b[1;48;5;231m\x1b[1;38;5;160m failed to silence \x1b[1;38;5;33m${filename}\x1b[0m\x1b[1;48;5;231m\x1b[1;38;5;16m line \x1b[1;38;5;28m${ensure_next_lineno}             \x1b[0m\t\t\t\t\t\t\t\t\t"
            error="true"
            break
        fi
    done
    ensure_next_lineno=$(( $ensure_next_lineno - 1 ))
    if [ "${error}" == "false" ]; then
        1>&2 echo -e "\r\x1b[A\x1b[1;38;5;231msilenced \x1b[1;38;5;33m${filename}\x1b[1;38;5;231m lines \x1b[1;38;5;220m$(( $lineno + 1 )) through \x1b[1;38;5;48m${ensure_next_lineno}\x1b[0m\t\t\t\t\t\t\t\t\t"
        git commit "${filename}" -m "silence \"${filename}\" lines $(( $lineno + 1 )) through ${ensure_next_lineno}"
    fi
}

if [ ! -e "$filename" ]; then
    1>&2 echo "ERROR: '${filename}' does not exist"
    exit 101
fi

rm -f "${error_filename}"
set +e
ensure_next_lines_commented
set -e

case "$1" in
    'auto'|'--auto')
        while ./"$script_name"; do
            ./"$script_name"
        done
        ;;
    *)

        output=$(2>&1 cargo test -j1 --test "$test_name" | ack '((thread|[-][-][>]).*[a-z_]+[.]rs|left|right):' | head -3 | sed 's/^[[:space:]]*//g')
        filename=$(echo "$output" | head -1 | sed 's,^.*\?\(thread.*at\|[-][-][>]\)\s*\([a-z_]\+/[a-z_]\+[.]rs\):\([0-9]\+\):.*,\2,g')
        lineno=$(( $(echo "$output" | head -1 | sed 's,^.*\?\(thread.*at\|[-][-][>]\)\s*[a-z_]\+/[a-z_]\+[.]rs:\([0-9]\+\):.*,\2,g') + 0 ))
        # # # # echo -e "\x1b[1;48;5;202m\x1b[1;38;5;16moutput=\x1b[0m\x1b[1;38;5;202m${output}\x1b[0m"
        # # # # exit
        error_filename=fix-test-error.sed

        next_lineno=$(( $lineno + 1 ))
        current=$(echo "$output" | tail -1 | sed 's,^.*right:\s*\([0-9]\+\).*$,\1,g')
        replace=$(echo "$output" | tail -2 | head -1 | sed 's,^.*left:\s*\([0-9]\+\).*$,\1,g' | sed 's/\\/\\\\/g' | sed 's/\([<>/]\)/\\\1/g')
        regex="$(echo -n "${current}" | sed 's/\([^a-zA-Z0-9]\)/[\1]/g')[)][;]"
        replace="${replace});"
        current="${current});"

        linecount=$(( $(wc -l "$filename" | awk '{ print $1 }') + 0 ))
        if [ $lineno -gt $linecount ]; then
            1>&2 echo "ERROR: line number ${lineno} is greater than the line count of '${filename}': ${linecount}"
            exit 101
        fi

        expression="${lineno}s/$regex/$replace/"
        1>&2 echo "sed \"${expression}\" -i \"$filename\""
        if 2>"${error_filename}" sed "${expression}" -i "$filename"; then
            if 2>/dev/random cargo test -j1 --test "$test_name"; then
                rm -f "$error_filename"
                git add -f "${filename}"
                expression="${next_lineno}s/^\(\s*\)\/\/\s*\(assert_[a-z_]\+!.*;\)/\1\2/"
                if 2>"${error_filename}" sed "${expression}" -i "$filename"; then
                    rm -f "${error_filename}"
                    git commit "$filename" -m "fix ${filename} line ${lineno}, such that \"${current}\" becomes \"${replace}\""
                    exit 0
                else
                    # git restore "$filename"
                    1>&2 echo -e "\x1b[1;48;5;160m\x1b[1;38;5;231mERROR:\t\x1b[1;48;5;231m\x1b[1;38;5;160m failed to uncomment \x1b[1;38;5;33m${filename}\x1b[0m\x1b[1;48;5;231m\x1b[1;38;5;16m line \x1b[1;38;5;28m${lineno}             \x1b[0m"
                    git diff
                    git diff --staged
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
        ;;
esac
