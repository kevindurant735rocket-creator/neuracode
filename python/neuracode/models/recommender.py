"""
Code recommender - AI-powered code recommendations
"""

from typing import List, Dict, Optional, Tuple
from dataclasses import dataclass
from enum import Enum
import re


class RecommendationType(Enum):
    """Types of recommendations"""
    BEST_PRACTICE = "best_practice"
    PERFORMANCE = "performance"
    SECURITY = "security"
    STYLE = "style"
    REFACTORING = "refactoring"
    TESTING = "testing"


@dataclass
class Recommendation:
    """A code recommendation"""
    type: RecommendationType
    title: str
    description: str
    priority: int  # 1-5, 5 being highest
    code_example: Optional[str] = None
    documentation_link: Optional[str] = None


class CodeRecommender:
    """Generate code recommendations"""
    
    def __init__(self):
        self.rules = self._load_rules()
    
    def _load_rules(self) -> List[Dict]:
        """Load recommendation rules"""
        return [
            {
                'type': RecommendationType.BEST_PRACTICE,
                'pattern': r'print\s*\(',
                'title': 'Use logging instead of print',
                'description': 'Consider using the logging module for better control over output.',
                'priority': 2,
            },
            {
                'type': RecommendationType.SECURITY,
                'pattern': r'eval\s*\(',
                'title': 'Avoid using eval()',
                'description': 'eval() can execute arbitrary code and is a security risk.',
                'priority': 5,
            },
            {
                'type': RecommendationType.SECURITY,
                'pattern': r'exec\s*\(',
                'title': 'Avoid using exec()',
                'description': 'exec() can execute arbitrary code and is a security risk.',
                'priority': 5,
            },
            {
                'type': RecommendationType.PERFORMANCE,
                'pattern': r'for\s+.*:\s*\n\s+.*\.append\(',
                'title': 'Consider using list comprehension',
                'description': 'List comprehensions are more Pythonic and often faster.',
                'priority': 2,
            },
            {
                'type': RecommendationType.BEST_PRACTICE,
                'pattern': r'except\s*:',
                'title': 'Avoid bare except clauses',
                'description': 'Specify the exception type to catch.',
                'priority': 4,
            },
            {
                'type': RecommendationType.STYLE,
                'pattern': r'==\s*True',
                'title': 'Simplify boolean comparison',
                'description': 'Use "if condition:" instead of "if condition == True:"',
                'priority': 1,
            },
            {
                'type': RecommendationType.BEST_PRACTICE,
                'pattern': r'global\s+',
                'title': 'Avoid global variables',
                'description': 'Consider using function parameters or class attributes.',
                'priority': 3,
            },
            {
                'type': RecommendationType.TESTING,
                'pattern': r'def\s+test_',
                'title': 'Add docstrings to test functions',
                'description': 'Document what each test is verifying.',
                'priority': 2,
            },
        ]
    
    def analyze(self, source: str, language: str = "python") -> List[Recommendation]:
        """Analyze code and generate recommendations"""
        recommendations = []
        
        for rule in self.rules:
            if re.search(rule['pattern'], source):
                rec = Recommendation(
                    type=rule['type'],
                    title=rule['title'],
                    description=rule['description'],
                    priority=rule['priority'],
                    code_example=rule.get('code_example'),
                    documentation_link=rule.get('documentation_link')
                )
                recommendations.append(rec)
        
        # Sort by priority
        recommendations.sort(key=lambda r: r.priority, reverse=True)
        
        return recommendations
    
    def get_quick_fixes(self, source: str, language: str = "python") -> List[Tuple[str, str]]:
        """Get quick fix suggestions"""
        fixes = []
        
        # Fix print to logging
        if 'print(' in source:
            fixes.append((
                "Replace print with logging",
                "import logging\nlogging.info('message')"
            ))
        
        # Fix bare except
        if re.search(r'except\s*:', source):
            fixes.append((
                "Specify exception type",
                "except Exception as e:\n    # handle exception"
            ))
        
        # Fix == True
        if '== True' in source:
            fixes.append((
                "Simplify boolean check",
                "if condition:  # instead of if condition == True:"
            ))
        
        return fixes
    
    def get_refactoring_suggestions(self, source: str, language: str = "python") -> List[str]:
        """Get refactoring suggestions"""
        suggestions = []
        
        # Check for long functions
        functions = re.findall(r'def\s+\w+\s*\([^)]*\):', source)
        if len(functions) > 10:
            suggestions.append("Consider splitting into multiple smaller files")
        
        # Check for deep nesting
        max_indent = 0
        for line in source.split('\n'):
            indent = len(line) - len(line.lstrip())
            max_indent = max(max_indent, indent)
        
        if max_indent > 16:  # More than 4 levels
            suggestions.append("Reduce nesting depth - consider early returns or helper functions")
        
        # Check for duplicate code
        lines = source.split('\n')
        line_counts = {}
        for line in lines:
            stripped = line.strip()
            if len(stripped) > 20:  # Only check substantial lines
                line_counts[stripped] = line_counts.get(stripped, 0) + 1
        
        duplicates = [line for line, count in line_counts.items() if count > 2]
        if duplicates:
            suggestions.append(f"Found {len(duplicates)} potentially duplicated code blocks")
        
        return suggestions
    
    def format_recommendations(self, recommendations: List[Recommendation]) -> str:
        """Format recommendations for display"""
        if not recommendations:
            return "No recommendations at this time. Code looks good!"
        
        lines = []
        lines.append("# Code Recommendations")
        lines.append("")
        
        # Group by type
        by_type: Dict[RecommendationType, List[Recommendation]] = {}
        for rec in recommendations:
            by_type.setdefault(rec.type, []).append(rec)
        
        for rec_type, recs in by_type.items():
            lines.append(f"## {rec_type.value.replace('_', ' ').title()}")
            lines.append("")
            
            for rec in recs:
                priority_str = "🔴" * rec.priority + "⚪" * (5 - rec.priority)
                lines.append(f"### {rec.title}")
                lines.append(f"Priority: {priority_str}")
                lines.append(f"{rec.description}")
                
                if rec.code_example:
                    lines.append("")
                    lines.append("Example:")
                    lines.append(f"```python\n{rec.code_example}\n```")
                
                lines.append("")
        
        return '\n'.join(lines)
