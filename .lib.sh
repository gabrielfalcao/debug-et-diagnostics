#!/usr/bin/env bash

term_width() {
    stty -a | head -1 | sed 's/^.*columns:\?\s*\([0-9]\+\)[^0-9]\+.*$/\1/g'
}

get_caller() {
    offset=$(( ${1:-0} + 0 ))
    stack_len=${#FUNCNAME[@]};
    last_caller_index=$(( ${stack_len} - 2 - ${offset} ))
    caller_index=$(( ${stack_len} - ${last_caller_index} ))
    echo -n "${FUNCNAME[$caller_index]}"
}
get_lineno() {
    offset=$(( ${1:-0} + 0 ))
    stack_len=${#BASH_LINENO[@]};
    last_lineno_index=$(( ${stack_len} - 1 - ${offset} ))
    lineno_index=$(( ${stack_len} - ${last_lineno_index} ))
    echo -n "${BASH_LINENO[$lineno_index]}"
}
panic() {
    caller=$(get_caller 2)
    lineno=$(get_lineno 2)

    title="PANIC"
    title_width=${#title}
    half_title_width=$(( $title_width / 2 ))
    location="in function ${caller}:${lineno}"
    location_width=${#location}
    half_location_width=$(( $location_width / 2 ))
    location="in function \x1b[4m\x1b[5m${caller}:${lineno}\x1b[24m\x1b[25m"
    msg="${@}"
    msg_width=${#msg}
    cols=$(term_width)
    half_cols=$(( $cols / 2 ))
    half_msg_width=$(( $msg_width / 2 ))
    bar_1st_half_end=$(( $half_cols - $half_title_width ))
    bar_2nd_half_end=$(( $half_cols - $half_location_width ))
    bar_3rd_half_end=$(( $half_cols - $half_msg_width ))
    panic_start="$(echo -en "$(seq $bar_1st_half_end | sed 's/[0-9]\+/@/g' | tr -d '[:space:]' | sed 's/@/ /g')")"
    location_start="$(echo -en "$(seq $bar_2nd_half_end | sed 's/[0-9]\+/@/g' | tr -d '[:space:]' | sed 's/@/ /g')")"
    msg_start="$(echo -en "$(seq $bar_3rd_half_end | sed 's/[0-9]\+/@/g' | tr -d '[:space:]' | sed 's/@/ /g')")"
    hr="$(echo -en "$(seq $cols | sed 's/[0-9]\+/@/g' | tr -d '[:space:]' | sed 's/@/ /g')")"
    1>&2 echo -e "\r\n\x1b[1;48;5;160m\x1b[1;38;5;231m${hr}"
    1>&2 echo -e "\x1b[1;48;5;160m\x1b[1;38;5;231m${panic_start}\x1b[1;48;5;160m\x1b[1;38;5;231m${title}\x1b[1;48;5;160m"
    1>&2 echo -e "\x1b[1;48;5;160m\x1b[1;38;5;231m${msg_start}\x1b[1;48;5;231m\x1b[1;38;5;160m${msg}\x1b[1;48;5;160m"
    1>&2 echo -e "\x1b[1;48;5;160m\x1b[1;38;5;231m${location_start}\x1b[1;48;5;231m\x1b[1;38;5;124m${location}\x1b[1;48;5;160m"
    1>&2 echo -e "\x1b[1;48;5;160m\x1b[1;38;5;231m${hr}\x1b[0m\n"
    exit 101
}
require_color_argument() {
    function=$(get_caller)
    argument_description="${1}"
    shift
    actual="${1}"
    color=$(( ${actual:-0} + 0 ));
    if [ -z "$actual" ]; then
        panic "${function}: ${argument_description} must be a number, actual is empty"
    elif [ $actual -gt 255 ]; then
        panic "${function}: ${argument_description} max value should be 256, actual: ${actual}"
    elif [ $actual -lt 0 ]; then
        panic "${function}: ${argument_description} min value should be 0, actual: ${actual}"
    elif [ "${color}" != "$actual" ]; then
        panic "${function}: ${argument_description} must be a number between 0 and 255, actual: ${actual}"
    fi
    echo -n ${color}
}
require_number_argument() {
    function=$(get_caller)
    argument_description="${1}"
    shift
    actual="${1}"
    number=$(( ${actual:-0} + 0 ));
    if [ -z "$actual" ]; then
        panic "${function}: ${argument_description} must be a number, actual is empty"
    elif [ "${number}" != "$actual" ]; then
        panic "${function}: ${argument_description} must be a number, actual: ${actual}"
    fi
    echo -n ${number}
}
require_argument() {
    function=$(get_caller)
    argument_description="${1}"
    shift
    actual="${@}"
    if [ -z "$actual" ]; then
        panic "${function}: ${argument_description} cannot be empty"
    fi
    echo -n "${actual}"
}
ansi_set_fg() {
    fg=$(require_color_argument "first argument (.i.e: ansi foreground)" "${1}")
    shift
    echo -en "\x1b[1;38;5;${fg}m"
}
ansi_set_bg() {
    bg=$(require_color_argument "first argument (.i.e: ansi background)" "${1}")
    shift
    echo -en "\x1b[1;48;5;${bg}m"
}
ansi_reset() {
    echo -en "\x1b[0m"
}

function display_colored() {
    fg=$(require_color_argument "first argument (.i.e: ansi foreground)" "${1}")
    shift
    bg=$(require_color_argument "second argument (.i.e: ansi background)" "${1}")
    shift
    desc=$(require_argument "third argument (.i.e: prefix)" "${1}")
    shift
    1>&2 echo -en "\x1b[1;48;5;${fg}m\x1b[1;38;5;${bg}m${desc}\x1b[0m"
    msg="$@"
    1>&2 echo -e "\x1b[1;48;5;${bg}m\x1b[1;38;5;${fg}m${msg}\x1b[0m"
}
function display_error() {
    desc="$1"
    shift
    msg="$@"
    display_colored 196 16 "\n$desc" "$msg"
}
function error() {
    desc="$1"
    shift
    msg="$@"
    display_error "$desc" "$msg"
    exit 101
}
function calc_indent_text() {
    requested=$1;
    padby=$1;
    shift
    text="$@"
    text_width=${#text}
    if [ -z "$text" ]; then
        display_error "calc_indent_text" "MISSING TEXT"
        exit 101
    fi
    if [ ${padby} -gt ${text_width} ]; then
        padby=$(( $padby - ${text_width} ))
    elif [ ${padby} -lt ${text_width} ]; then
        padby=$(( ${text_width} - $padby ))
    elif [ ${text_width} -eq ${padby} ]; then
        padby=0
    fi
    if [ $padby -lt $requested ]; then
        padby=$requested
    fi
    if [ $padby -lt 0 ]; then
        padby=0
    fi
    echo -n $padby
}
function make_indent() {
    padby=$(require_number_argument "first argument (.i.e: indent count)" "${1}")
    echo -en "$(seq $padby | sed 's/[0-9]\+/@/g' | tr -d '[:space:]' | sed 's/@/ /g')"
}
function indent() {
    padby=$(require_number_argument "first argument (.i.e: indent count)" "${1}")
    shift
    text=$(require_argument "second argument (.i.e: text)" "${@}")
    padby=$(calc_indent_text $padby "${text}")
    echo -en "$(make_indent $padby)${text}"
}
function indent_rev() {
    padby=$(require_number_argument "first argument (.i.e: indent count)" "${1}")
    shift
    text=$(require_argument "second argument (.i.e: text)" "${@}")
    padby=$(calc_indent_text $padby "${text}")
    echo -en "${text}$(make_indent $padby)"
}

bar() {
    color=$(require_color_argument "first argument (.i.e: ansi foreground)" "${1}")
    shift
    cols=$(term_width)
    hr=$(make_indent ${cols})
    1>&2 echo -e "\x1b[0m\x1b[1;48;5;${color}m${hr}\x1b[0m"
}
bar_text_left() {
    fg=$(require_color_argument "first argument (.i.e: ansi foreground)" "${1}")
    shift
    bg=$(require_color_argument "second argument (.i.e: ansi background)" "${1}")
    shift
    text=$(require_argument "second argument (.i.e: text)" "${@}")
    text_width=${#text}
    cols=$(term_width)
    bar_end=$(( $cols - ${text_width} ));
    hr=$(make_indent "$bar_end")
    1>&2 echo -e "\x1b[1;48;5;${fg}m\x1b[1;38;5;${bg}m${text}${hr}\x1b[0m"
}
bar_text_right() {
    fg=$(require_color_argument "first argument (.i.e: ansi foreground)" "${1}")
    shift
    bg=$(require_color_argument "second argument (.i.e: ansi background)" "${1}")
    shift
    text=$(require_argument "second argument (.i.e: text)" "${@}")
    text_width=${#text}
    cols=$(term_width)
    bar_start=$(( $cols - ${text_width} ));
    hr=$(make_indent "$bar_start")
    1>&2 echo -e "\x1b[1;48;5;${fg}m\x1b[1;38;5;${bg}m${hr}${text}\x1b[0m"
}
bar_text_center() {
    fg=$(require_color_argument "first argument (.i.e: ansi foreground)" "${1}")
    shift
    bg=$(require_color_argument "second argument (.i.e: ansi background)" "${1}")
    shift
    text=$(require_argument "second argument (.i.e: text)" "${@}")
    text_width=${#text}
    cols=$(term_width)
    bar_start=$(( $cols - ${text_width} ));
    hr=$(make_indent "$bar_start")
    half_cols=$(( $cols / 2 ))
    half_text_width=$(( $text_width / 2 ))
    first_pad=$(( $half_cols - $half_text_width ))
    text_start=$(make_indent $first_pad)
    hr=$(make_indent $cols);
    1>&2 echo -en "\x1b[1;48;5;${fg}m${text_start}\x1b[1;38;5;${bg}m${text}\x1b[1;48;5;${fg}m"
    first_pad=$(( $half_cols - $half_text_width ))
    hr=$(make_indent "$first_pad")
    1>&2 echo -e "\x1b[1;48;5;${fg}m${hr}\x1b[0m"
}


banner() {
    fg=$(require_color_argument "first argument (.i.e: ansi foreground)" "${1}")
    shift
    bg=$(require_color_argument "second argument (.i.e: ansi background)" "${1}")
    shift
    title=$(require_argument "third argument (.i.e: title)" "${1}")
    shift
    title_width=${#title}
    half_title_width=$(( $title_width / 2 ))
    msg=$(require_argument "fourth argument (.i.e: message)" "${@}")
    msg_width=${#msg}
    half_msg_width=$(( $msg_width / 2 ))
    cols=$(term_width)
    half_cols=$(( $cols / 2 ))
    bar_1st_half_end=$(( $half_cols - $half_title_width ))
    bar_2nd_half_end=$(( $half_cols - $half_msg_width ))
    panic_start=$(make_indent $bar_1st_half_end)
    panic_end=$(make_indent $(( $bar_1st_half_end - $half_title_width )) )
    msg_start=$(make_indent $bar_2nd_half_end )
    msg_end=$(make_indent $(( $bar_2nd_half_end - $half_msg_width )) )
    bar ${fg}
    bar_text_center ${fg} ${bg} "${title}"
    bar_text_center ${bg} ${fg} "${msg}"
    bar ${fg}
    exit 101
}

if [ "$0" == "${BASH_SOURCE[0]}" ]; then
    bar_text_left 220 16 "text"
    bar_text_right 220 16 "text"
    bar_text_center 220 16 "text"

    ansi_set_bg 33
    indent 16 "text"
    ansi_reset
    echo
    ansi_set_bg 124
    indent_rev 16 "text"
    ansi_reset
    echo

    banner 48 16 FOO BAR
fi
