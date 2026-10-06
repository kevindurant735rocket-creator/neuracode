"""Helper utilities"""

import os
import hashlib
from typing import List, Set


def get_file_hash(path: str) -> str:
    """Calculate SHA256 hash of a file"""
    sha256 = hashlib.sha256()
    with open(path, 'rb') as f:
        for chunk in iter(lambda: f.read(8192), b''):
            sha256.update(chunk)
    return sha256.hexdigest()


def find_files(root: str, extensions: Set[str]) -> List[str]:
    """Find all files with given extensions"""
    result = []
    for dirpath, dirnames, filenames in os.walk(root):
        # Skip common directories
        dirnames[:] = [d for d in dirnames if d not in {
            'node_modules', '.git', 'target', 'dist', 'build',
            '__pycache__', '.venv', 'venv'
        }]
        
        for filename in filenames:
            ext = os.path.splitext(filename)[1]
            if ext in extensions:
                result.append(os.path.join(dirpath, filename))
    
    return result


def read_file_content(path: str, max_lines: int = None) -> str:
    """Read file content with optional line limit"""
    try:
        with open(path, 'r', encoding='utf-8', errors='ignore') as f:
            if max_lines:
                lines = []
                for i, line in enumerate(f):
                    if i >= max_lines:
                        break
                    lines.append(line)
                return ''.join(lines)
            else:
                return f.read()
    except Exception as e:
        return f"Error reading file: {e}"


def truncate_text(text: str, max_length: int = 1000) -> str:
    """Truncate text to maximum length"""
    if len(text) <= max_length:
        return text
    return text[:max_length - 3] + '...'


def format_size(size_bytes: int) -> str:
    """Format bytes to human readable string"""
    for unit in ['B', 'KB', 'MB', 'GB', 'TB']:
        if size_bytes < 1024:
            return f"{size_bytes:.2f} {unit}"
        size_bytes /= 1024
    return f"{size_bytes:.2f} PB"
