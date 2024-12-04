#!/bin/bash
# file   : aoc_day_template.sh
# author : palcoo
# purpose: Create new folder for 'Advent of Code' puzzle with basic templates

# Path to the templates directory
readonly TEMPLATES_DIR="$HOME/workspace/rust/advent-of-code/templates"

# Path to the new directory is in current directory
readonly PUZZLE_ROOT_DIR="$(pwd)"

# Name of the puzzle
PUZZLE_NAME=""

# Terminates script with an error message
#
# 1st - Error message
function die()
{
    echo "ERROR: $1"
    exit 1
}

# Terminates script with an error message and help screen
#
# 1st - Error message
function die_with_help()
{
    echo "ERROR: $1"
    echo
    show_help
    exit 1
}

# Show help screen
function show_help()
{
    echo "./$(basename "$0") <puzzle name> [-h | --help]"
    echo
    echo "  Creates a new folder structure for 'Advent of Code' daily challenge."
    echo
    echo "  ARGUMENTS:"
    echo "      -h | --help     - help screen"
}

function parse_cmd_line()
{
    # We will use manual parsing as it is weird to use 'aoc-day-template -- "--- Day 01: Puzzle ---"'.
    # With manual parsing we can simply use 'aoc-day-template "--- Day 01: Puzzle ---"'
    POSITIONAL_ARGS=()

    while [[ $# -gt 0 ]]; do
        case $1 in
            -h | --help)
                show_help
                exit
                ;;

            *)
                POSITIONAL_ARGS+=("$1")
                shift # past argument
                ;;
        esac
    done

    # We support only single positional argument with the name of the puzzle
    length=${#POSITIONAL_ARGS[@]}

    if [ "$length" -eq 0 ]; then
        die_with_help "Missing argument 'puzzle name'"
    elif [ "$length" -gt 1 ]; then
        die_with_help "Too many arguments provided"
    fi

    PUZZLE_NAME="${POSITIONAL_ARGS[0]}"
}

# Creates new folder with predefined puzzle template
function create_day_puzzle()
{
    local puzzle_dir_name=
    puzzle_dir_name=$(format_puzzle_dir)

    local puzzle_dir_name_path="$PUZZLE_ROOT_DIR/$puzzle_dir_name"

    # Create folder structure
    echo "- Creating project structure '$puzzle_dir_name'"

    if ! mkdir -p "$puzzle_dir_name_path"; then
        die "Failed to create directory '$puzzle_dir_name_path'"
    fi

    # Copy template files
    echo "- Copying template files ..."

    if ! cp -r "$TEMPLATES_DIR"/day_xx/* "$puzzle_dir_name_path"; then
        die "Failed to copy template files to '$puzzle_dir_name_path'"
    fi

    # Replace default files content with puzzle specific names
    echo "- Updating template files for your puzzle ..."

    if ! sed -i "s/TEMPLATE_PACKAGE_NAME/${puzzle_dir_name}/g" "${puzzle_dir_name_path}/Cargo.toml"; then
        die "Failed to replace TEMPLATE_PACKAGE_NAME in Cargo.toml"
    fi

    if ! sed -i "s/TEMPLATE_SOLVER_DESCRIPTION/${PUZZLE_NAME}/g" "${puzzle_dir_name_path}/src/puzzle/solver.rs"; then
        die "Failed to replace TEMPLATE_SOLVER_DESCRIPTION in src/puzzle/solver.rs"
    fi

    echo
    echo "- You can find your Rust project under '${puzzle_dir_name_path}'"
}

# Format new puzzle directory
function format_puzzle_dir()
{
    # Take puzzle name from command line and do following changes to format name of the puzzle directory:
    #
    # 1) Append 0 before 1-9 in Day                             sed 's/Day \([1-9]\)\b/Day 0\1/g'
    #    Append 0 before 1-9 in Quest                           sed 's/Quest \([1-9]\)\b/Quest 0\1/g'
    # 2) Remove non-ASCII characters -:'?                       tr -d
    # 3) Remove leading and trailing spaces                     xargs echo -n
    # 4) Convert all characters to lowercase                    tr '[:upper:]' '[:lower:]'
    # 5) Replace spaces with underscores                        tr '[:space:]' '_'

    # We support both 'Advent of Code' and 'Everybody Codes'
    echo "$PUZZLE_NAME" | sed 's/Day \([1-9]\)\b/Day 0\1/g' | sed 's/Quest \([1-9]\)\b/Quest 0\1/g' | tr -d - | tr -d : | tr -d \' | tr -d ? | xargs echo -n  |  tr '[:upper:]' '[:lower:]' | tr '[:space:]' '_'
}

# main
parse_cmd_line "$@"
create_day_puzzle
