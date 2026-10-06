"""
Task classification using machine learning
"""

from typing import List, Dict, Tuple
from enum import Enum
import re


class TaskType(Enum):
    """Types of coding tasks"""
    BUG_FIX = "bug_fix"
    REFACTOR = "refactor"
    NEW_FEATURE = "new_feature"
    CODE_REVIEW = "code_review"
    PERFORMANCE = "performance"
    SECURITY = "security"
    DOCUMENTATION = "documentation"
    TESTING = "testing"
    ARCHITECTURE = "architecture"
    GENERAL = "general"


class TaskClassifier:
    """Classify coding tasks into categories"""
    
    def __init__(self):
        """Initialize the classifier"""
        self.patterns = self._build_patterns()
    
    def _build_patterns(self) -> Dict[TaskType, List[str]]:
        """Build pattern matching rules"""
        return {
            TaskType.BUG_FIX: [
                r'\bfix\b', r'\bbug\b', r'\berror\b', r'\bcrash\b',
                r'\bbroken\b', r'\bissue\b', r'\bproblem\b', r'\bfailing\b',
                r'\bdoesn\'?t work\b', r'\bnot working\b', r'\bexception\b',
            ],
            TaskType.REFACTOR: [
                r'\brefactor\b', r'\bclean\b', r'\bsimplify\b',
                r'\brestructure\b', r'\bimprove\b', r'\boptimize\b',
                r'\bredesign\b', r'\brewrite\b',
            ],
            TaskType.NEW_FEATURE: [
                r'\badd\b', r'\bfeature\b', r'\bimplement\b',
                r'\bcreate\b', r'\bnew\b', r'\bbuild\b', r'\bdevelop\b',
                r'\bintroduce\b',
            ],
            TaskType.CODE_REVIEW: [
                r'\breview\b', r'\bcheck\b', r'\baudit\b',
                r'\binspect\b', r'\bverify\b', r'\bvalidate\b',
            ],
            TaskType.PERFORMANCE: [
                r'\bperformance\b', r'\boptimize\b', r'\bspeed\b',
                r'\bslow\b', r'\bcache\b', r'\blatency\b',
                r'\bmemory\b', r'\bbottleneck\b',
            ],
            TaskType.SECURITY: [
                r'\bsecurity\b', r'\bvulnerability\b', r'\bxss\b',
                r'\binjection\b', r'\bauth\b', r'\bencrypt\b',
                r'\bsanitize\b', r'\bvalidate\b', r'\bsecure\b',
            ],
            TaskType.DOCUMENTATION: [
                r'\bdocument\b', r'\bdocs\b', r'\bcomment\b',
                r'\breadme\b', r'\bexplain\b', r'\bdescribe\b',
            ],
            TaskType.TESTING: [
                r'\btest\b', r'\bspec\b', r'\bcoverage\b',
                r'\bunit\b', r'\bintegration\b', r'\be2e\b',
                r'\bmock\b', r'\bstub\b',
            ],
            TaskType.ARCHITECTURE: [
                r'\barchitecture\b', r'\bdesign\b', r'\bpattern\b',
                r'\bstructure\b', r'\blayer\b', r'\bmodule\b',
                r'\bcomponent\b', r'\bservice\b',
            ],
        }
    
    def classify(self, task: str) -> Tuple[TaskType, float]:
        """Classify a task
        
        Args:
            task: Task description
            
        Returns:
            Tuple of (TaskType, confidence)
        """
        task_lower = task.lower()
        
        scores = {}
        for task_type, patterns in self.patterns.items():
            score = 0
            for pattern in patterns:
                matches = len(re.findall(pattern, task_lower))
                score += matches
            
            if score > 0:
                scores[task_type] = score
        
        if not scores:
            return TaskType.GENERAL, 0.5
        
        # Get the task type with highest score
        best_type = max(scores, key=scores.get)
        best_score = scores[best_type]
        
        # Calculate confidence
        total_score = sum(scores.values())
        confidence = best_score / total_score if total_score > 0 else 0.5
        
        return best_type, confidence
    
    def classify_batch(self, tasks: List[str]) -> List[Tuple[TaskType, float]]:
        """Classify multiple tasks
        
        Args:
            tasks: List of task descriptions
            
        Returns:
            List of (TaskType, confidence) tuples
        """
        return [self.classify(task) for task in tasks]
    
    def get_suggestions(self, task_type: TaskType) -> List[str]:
        """Get suggestions for a task type
        
        Args:
            task_type: Type of task
            
        Returns:
            List of suggestions
        """
        suggestions = {
            TaskType.BUG_FIX: [
                "Identify the root cause first",
                "Write a test that reproduces the bug",
                "Check recent changes that might have caused it",
                "Look at error logs and stack traces",
            ],
            TaskType.REFACTOR: [
                "Ensure tests pass before refactoring",
                "Make small, incremental changes",
                "Preserve existing behavior",
                "Update documentation",
            ],
            TaskType.NEW_FEATURE: [
                "Design the API first",
                "Write tests before implementation",
                "Consider edge cases",
                "Update documentation",
            ],
            TaskType.CODE_REVIEW: [
                "Check for bugs and logic errors",
                "Verify code style consistency",
                "Look for security issues",
                "Check performance implications",
            ],
            TaskType.PERFORMANCE: [
                "Profile before optimizing",
                "Identify bottlenecks",
                "Consider caching strategies",
                "Measure after changes",
            ],
            TaskType.SECURITY: [
                "Validate all inputs",
                "Check for injection vulnerabilities",
                "Verify authentication and authorization",
                "Use parameterized queries",
            ],
            TaskType.DOCUMENTATION: [
                "Keep documentation up to date",
                "Use clear and concise language",
                "Include examples",
                "Document edge cases",
            ],
            TaskType.TESTING: [
                "Write unit tests for new code",
                "Aim for high coverage",
                "Test edge cases",
                "Use meaningful test names",
            ],
            TaskType.ARCHITECTURE: [
                "Follow established patterns",
                "Consider scalability",
                "Document design decisions",
                "Keep components decoupled",
            ],
            TaskType.GENERAL: [
                "Break down the task into smaller steps",
                "Write tests",
                "Document your changes",
            ],
        }
        
        return suggestions.get(task_type, [])
