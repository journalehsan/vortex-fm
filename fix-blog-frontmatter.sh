#!/bin/bash

# Script to add frontmatter to blog posts that don't have it

cd /home/ehsator/Documents/GitHub/vortex-fm/content/blog/

for file in *.md; do
    if [ "$file" != "_index.md" ] && [ "$file" != "theme-system.md" ] && [ "$file" != "cosmic-integration.md" ]; then
        # Check if file already has frontmatter
        if ! head -n 1 "$file" | grep -q "+++"; then
            # Get the title from the first heading
            title=$(head -n 10 "$file" | grep "^#" | head -n 1 | sed 's/^# *//' | sed 's/🎨//' | sed 's/✅//' | sed 's/🔍//' | sed 's/🚀//' | sed 's/📝//' | sed 's/💡//' | sed 's/🔧//' | sed 's/📊//' | sed 's/🎯//' | sed 's/⚡//' | sed 's/🌟//' | sed 's/🔬//' | sed 's/📈//' | sed 's/🎨//' | sed 's/🔍//' | sed 's/✅//' | sed 's/🚀//' | sed 's/📝//' | sed 's/💡//' | sed 's/🔧//' | sed 's/📊//' | sed 's/🎯//' | sed 's/⚡//' | sed 's/🌟//' | sed 's/🔬//' | sed 's/📈//' | sed 's/^ *//' | sed 's/ *$//')
            
            # Create a temporary file with frontmatter
            echo "+++" > temp_file
            echo "title = \"$title\"" >> temp_file
            echo "date = 2024-01-15" >> temp_file
            echo "description = \"Development insights and technical updates\"" >> temp_file
            echo "+++" >> temp_file
            echo "" >> temp_file
            
            # Append the original content
            cat "$file" >> temp_file
            
            # Replace the original file
            mv temp_file "$file"
            
            echo "Added frontmatter to $file"
        fi
    fi
done

echo "Frontmatter addition complete!"
