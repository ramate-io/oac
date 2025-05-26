#!/bin/bash

set -e

# Status helper function
status() {
    if [ $? -eq 0 ]; then
        echo "✅ $1"
    else
        echo "❌ $1"
        return 1
    fi
}

echo "Running pre-commit hook: Managing OAC footer in markdown files..."

# Find all tracked markdown files in the repository, excluding template files
FILES=$(git ls-files --cached --exclude-standard -- '*.md' | grep -v '\.template\.md$')
status "Finding markdown files" || exit 1

if [ -z "$FILES" ]; then
    echo "📭 No markdown files found in the repository."
    exit 0
fi

# Count total files for progress reporting
TOTAL_FILES=$(echo "$FILES" | wc -l | tr -d ' ')
CURRENT=0
FAILED_FILES=()

# Process each markdown file
for file in $FILES; do
    CURRENT=$((CURRENT + 1))
    echo "📝 Processing ${file} (${CURRENT}/${TOTAL_FILES})..."
    
    # Create a temporary file
    TMP_FILE=$(mktemp)
    
    # Remove existing footer if present and clean up whitespace
    awk '
        BEGIN { in_footer=0; last_was_empty=0 }
        /<!--OAC FOOTER: DO NOT REMOVE THIS LINE-->/ { in_footer=1; next }
        in_footer && /^--$/ { next }
        in_footer && /^<h5>/ { next }
        in_footer && /^<\/h5>/ { in_footer=0; next }
        !in_footer { 
            if ($0 ~ /^[[:space:]]*$/) {
                if (!last_was_empty) print
                last_was_empty=1
            } else {
                print
                last_was_empty=0
            }
        }
        END { 
            if (!last_was_empty) print ""
        }
    ' "$file" | sed -e 's/[[:space:]]*$//' > "$TMP_FILE"
    
    # Add new footer
    if [ $? -eq 0 ]; then
        cat assets/footer.template.md >> "$TMP_FILE"
        
        # Replace original file with modified content
        if mv "$TMP_FILE" "$file"; then
            # Add the modified file to staging
            if git add "$file"; then
                echo "✅ Updated ${file}"
            else
                echo "❌ Failed to stage ${file}"
                FAILED_FILES+=("${file} (staging failed) ❌")
            fi
        else
            echo "❌ Failed to update ${file}"
            FAILED_FILES+=("${file} (update failed) ❌")
        fi
    else
        echo "❌ Failed to process ${file}"
        FAILED_FILES+=("${file} (processing failed) ❌")
    fi
done

# Report results
echo "✨ Footer management complete! Processed ${TOTAL_FILES} markdown files."

if [ ${#FAILED_FILES[@]} -gt 0 ]; then
    echo "❌ The following files failed to process:"
    for file in "${FAILED_FILES[@]}"; do
        echo "   - ${file}"
    done
    exit 1
fi

exit 0