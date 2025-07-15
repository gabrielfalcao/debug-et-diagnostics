#!/usr/bin/env bash
set -e
output=$(2>&1 cargo test -j1 | ack '(thread.*[a-z_]+.rs|left|right):' | head -3 | sed 's/^[[:space:]]*//g')
filename=$(echo "$output" | head -1 | sed 's,^\s*thread.*at\s*\([a-z_]\+/[a-z_]\+[.]rs\):\([0-9]\+\):.*,\1,g')
linenumb=$(echo "$output" | head -1 | sed 's,^\s*thread.*at\s*[a-z_]\+/[a-z_]\+[.]rs:\([0-9]\+\):.*,\1,g')
current=$(echo "$output" | tail -1 | sed 's,^.*right:\s*\([[].*[]]\).*$,\1,g')
replace=$(echo "$output" | tail -2 | head -1 | sed 's,^.*left:\s*\([[].*[]]\).*$,\1,g')

echo -e "filename=\"${filename}\""
echo -e "linenumb=\"${linenumb}\""
echo -e "current=\"${current}\""
echo -e "replace=\"${replace}\""
exit
#
# make_indent() {
#     padby=$1;
#     echo -en "$(seq $padby | sed 's/[0-9]\+/@/g' | tr -d '[:space:]' | sed 's/@/ /g')"
# }
# indent() {
#     padby=$1;
#     shift
#     text="$(echo -en "$@" | sed -z \"s/\n/\n$(make_indent $padby)/g\")"
#     echo -e "$(make_indent $padby)${text}"
# }
# colored() {
#     fg=$1
#     shift
#     bg=$1
#     shift
#     msg="$@"
#     echo -e "\x1b[1;48;5;${bg}m\x1b[1;38;5;${fg}m${msg}\x1b[0m"
# }
# tag() {
#     prefix="";
#     case $1 in
#         'close')
#             prefix="</";
#             shift
#             ;;
#         'open')
#             prefix="<";
#             shift
#             ;;
#         'wrap')
#             shift
#             name="$1"
#             shift
#             content="$@"
#             echo "$(tag open "$name")"
#             indent 4 "$(colored 82 16 "${content}")"
#             echo "$(tag close "$name")"
#             return
#             ;;
#         *)
#             prefix="<";
#             shift
#             ;;
#     esac
#     tag="$1"
#     colored 220 16 "${prefix}${tag}>"
# }
#
# echo -e "$(tag wrap 'output' "${output}")\n"
# echo -e "$(tag wrap 'linenumb' "${linenumb}")\n"
# echo -e "$(tag wrap 'current' "${current}")\n"
# echo -e "$(tag wrap 'replace' "${replace}")\n"
#
