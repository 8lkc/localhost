#!/usr/bin/env python3
import os
import datetime
import html

# CGI needs to print headers first, followed by blank line
print("Content-Type: text/html\n")

# Get current directory or from query string
base_path = "public"
current_path = os.path.join(base_path, os.environ.get('QUERY_STRING', '').strip('/'))

print("""
<!DOCTYPE html>
<html>
<head>
    <title>File Explorer</title>
    <style>
        body {
            font-family: Arial, sans-serif;
            margin: 40px;
            background: #f5f5f5;
        }
        .explorer {
            background: white;
            padding: 20px;
            border-radius: 5px;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }
        .item {
            display: flex;
            padding: 10px;
            border-bottom: 1px solid #eee;
        }
        .name { flex: 2; }
        .size, .date { flex: 1; }
        .directory { color: #2c5282; font-weight: bold; }
        .file { color: #444; }
        a { text-decoration: none; color: inherit; }
        .header {
            font-weight: bold;
            background: #f8f9fa;
            padding: 10px;
        }
    </style>
</head>
<body>
    <div class="explorer">
        <h1>File Explorer</h1>
        <div class="item header">
            <div class="name">Name</div>
            <div class="size">Size</div>
            <div class="date">Modified</div>
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
