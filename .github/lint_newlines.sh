#!/bin/bash

# List all tracked files
FILES=$(git ls-files)

EXIT_CODE=0

for file in $FILES; do
    # Skip .svg files
    if [[ "$file" == *.svg ]]; then
        continue
    fi
    
    # Skip binary files
    if file "$file" | grep -qv 'text'; then
        continue
    fi

    # Skip empty files
    if [ ! -s "$file" ]; then
        continue
    fi

    # Skip special files
    if [[ "$file" == *"Cargo.recipe.json" ]]; then
        continue
    fi

    # Check if the last byte is a newline
    if [ "$(tail -c1 "$file")" != "" ]; then
        echo "Missing newline at end of file: $file"
        EXIT_CODE=1
    fi
done

exit $EXIT_CODE
