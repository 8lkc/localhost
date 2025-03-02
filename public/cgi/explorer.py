#!/usr/bin/env python3
import os
import html
import datetime
import sys

# Set the base path to the public directory
base_path = os.path.abspath(os.path.join(os.path.dirname(__file__), '..'))
current_path = os.path.join(base_path, os.environ.get('QUERY_STRING', '').lstrip('/'))

# Ensure the current path is within the base path for security
if not current_path.startswith(base_path):
    current_path = base_path

print("Content-Type: text/html\n")

print(f"""
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>File Explorer</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 0; padding: 20px; }}
        .item {{ display: flex; padding: 10px; border-bottom: 1px solid #eee; }}
        .name {{ flex: 1; }}
        .size, .date {{ width: 100px; text-align: right; }}
        .directory {{ font-weight: bold; }}
    </style>
</head>
<body>
    <h1>Directory Listing: {os.path.relpath(current_path, base_path)}</h1>
    <div class="item">
        <div class="name"><strong>Name</strong></div>
        <div class="size"><strong>Size</strong></div>
        <div class="date"><strong>Last Modified</strong></div>
    </div>
""")

try:
    # List directory contents
    entries = os.scandir(current_path)

    # Sort entries (directories first, then files)
    sorted_entries = sorted(entries, key=lambda e: (not e.is_dir(), e.name.lower()))

    # Show parent directory link if not in base
    if current_path != base_path:
        parent = os.path.dirname(current_path.rstrip('/'))
        print(f'''
            <div class="item">
                <div class="name directory">
                    <a href="?{os.path.relpath(parent, base_path)}">..</a>
                </div>
                <div class="size">-</div>
                <div class="date">-</div>
            </div>
        ''')

    # List all entries
    for entry in sorted_entries:
        name = html.escape(entry.name)
        is_dir = entry.is_dir()
        stat = entry.stat()

        # Format size
        if is_dir:
            size = "-"
        else:
            size_bytes = stat.st_size
            if size_bytes < 1024:
                size = f"{size_bytes} B"
            elif size_bytes < 1024 * 1024:
                size = f"{size_bytes/1024:.1f} KB"
            else:
                size = f"{size_bytes/(1024*1024):.1f} MB"

        # Format date
        mtime = datetime.datetime.fromtimestamp(stat.st_mtime)
        date = mtime.strftime("%Y-%m-%d %H:%M")

        # Create relative path for link
        rel_path = os.path.relpath(entry.path, base_path)

        # Print entry
        print(f'''
            <div class="item">
                <div class="name {'directory' if is_dir else 'file'}">
                    <a href="?{rel_path}">{name}</a>
                </div>
                <div class="size">{size}</div>
                <div class="date">{date}</div>
            </div>
        ''')

except Exception as e:
    print(f"<p>Error: {html.escape(str(e))}</p>")

print("""
    </div>
</body>
</html>
""")
