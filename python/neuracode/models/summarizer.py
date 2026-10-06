"""
Code summarizer - Generate human-readable summaries of code
"""

from typing import List, Optional
from dataclasses import dataclass
import re


@dataclass
class Summary:
    """A code summary"""
    title: str
    description: str
    key_points: List[str]
    complexity: str  # 'low', 'medium', 'high'
    tags: List[str]


class CodeSummarizer:
    """Generate summaries of code"""
    
    def __init__(self):
        pass
    
    def summarize_function(self, name: str, source: str, language: str = "python") -> Summary:
        """Summarize a function"""
        key_points = []
        tags = []
        
        # Extract function signature
        sig_match = re.search(r'def\s+\w+\([^)]*\)', source)
        if sig_match:
            key_points.append(f"Signature: {sig_match.group()}")
        
        # Check for async
        if 'async def' in source:
            tags.append('async')
            key_points.append("Asynchronous function")
        
        # Check for decorators
        if '@' in source:
            tags.append('decorated')
        
        # Check for return statements
        returns = re.findall(r'return\s+', source)
        if returns:
            key_points.append(f"Returns: {len(returns)} return statement(s)")
        
        # Check for error handling
        if 'try:' in source or 'except' in source:
            tags.append('error-handling')
            key_points.append("Includes error handling")
        
        # Check for loops
        loops = len(re.findall(r'for\s+|while\s+', source))
        if loops > 0:
            key_points.append(f"Contains {loops} loop(s)")
        
        # Calculate complexity
        complexity = self._calculate_complexity(source)
        
        # Generate description
        description = self._generate_description(name, source, key_points)
        
        return Summary(
            title=f"Function: {name}",
            description=description,
            key_points=key_points,
            complexity=complexity,
            tags=tags
        )
    
    def summarize_class(self, name: str, source: str, language: str = "python") -> Summary:
        """Summarize a class"""
        key_points = []
        tags = []
        
        # Count methods
        methods = re.findall(r'def\s+\w+\s*\(', source)
        key_points.append(f"Methods: {len(methods)}")
        
        # Check for inheritance
        class_match = re.search(r'class\s+\w+\s*\(([^)]+)\)', source)
        if class_match:
            bases = class_match.group(1).strip()
            if bases:
                tags.append('inheritance')
                key_points.append(f"Inherits from: {bases}")
        
        # Check for properties
        if '@property' in source:
            tags.append('properties')
            key_points.append("Has properties")
        
        # Check for class methods
        if '@classmethod' in source:
            tags.append('class-methods')
        
        # Check for static methods
        if '@staticmethod' in source:
            tags.append('static-methods')
        
        # Calculate complexity
        complexity = self._calculate_complexity(source)
        
        # Generate description
        description = f"Class {name} with {len(methods)} methods"
        
        return Summary(
            title=f"Class: {name}",
            description=description,
            key_points=key_points,
            complexity=complexity,
            tags=tags
        )
    
    def summarize_file(self, path: str, source: str, language: str = "python") -> Summary:
        """Summarize a file"""
        key_points = []
        tags = []
        
        # Count lines
        lines = source.split('\n')
        key_points.append(f"Lines of code: {len(lines)}")
        
        # Count functions
        functions = re.findall(r'def\s+\w+\s*\(', source)
        if functions:
            key_points.append(f"Functions: {len(functions)}")
        
        # Count classes
        classes = re.findall(r'class\s+\w+', source)
        if classes:
            key_points.append(f"Classes: {len(classes)}")
        
        # Count imports
        imports = re.findall(r'^import\s+|^from\s+', source, re.MULTILINE)
        if imports:
            key_points.append(f"Imports: {len(imports)}")
        
        # Check for main block
        if '__name__' in source and '__main__' in source:
            tags.append('executable')
            key_points.append("Has main entry point")
        
        # Check for tests
        if 'test' in path.lower():
            tags.append('test')
        
        # Calculate complexity
        complexity = self._calculate_complexity(source)
        
        # Generate description
        description = f"File: {path}"
        
        return Summary(
            title=f"File: {path}",
            description=description,
            key_points=key_points,
            complexity=complexity,
            tags=tags
        )
    
    def _calculate_complexity(self, source: str) -> str:
        """Calculate code complexity"""
        score = 0
        
        # Count decision points
        score += len(re.findall(r'\bif\b|\belse\b|\belif\b', source))
        score += len(re.findall(r'\bfor\b|\bwhile\b', source))
        score += len(re.findall(r'\btry\b|\bexcept\b', source))
        score += len(re.findall(r'\band\b|\bor\b', source))
        
        if score < 5:
            return 'low'
        elif score < 15:
            return 'medium'
        else:
            return 'high'
    
    def _generate_description(self, name: str, source: str, key_points: List[str]) -> str:
        """Generate a description"""
        # Try to extract docstring
        docstring_match = re.search(r'"""(.*?)"""', source, re.DOTALL)
        if docstring_match:
            docstring = docstring_match.group(1).strip()
            # Take first line
            first_line = docstring.split('\n')[0].strip()
            if first_line:
                return first_line
        
        # Generate from name
        words = re.findall(r'[A-Z]?[a-z]+|[A-Z]+(?=[A-Z][a-z]|\d|\b)', name)
        if words:
            return ' '.join(words)
        
        return f"Function {name}"
    
    def format_summary(self, summary: Summary) -> str:
        """Format a summary for display"""
        lines = []
        lines.append(f"# {summary.title}")
        lines.append("")
        lines.append(summary.description)
        lines.append("")
        
        if summary.key_points:
            lines.append("## Key Points")
            for point in summary.key_points:
                lines.append(f"- {point}")
            lines.append("")
        
        if summary.tags:
            lines.append(f"**Tags:** {', '.join(summary.tags)}")
            lines.append("")
        
        lines.append(f"**Complexity:** {summary.complexity}")
        
        return '\n'.join(lines)
