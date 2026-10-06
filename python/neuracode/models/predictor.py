"""
Context prediction for tasks
"""

from typing import List, Dict, Set
from dataclasses import dataclass
import os


@dataclass
class ContextItem:
    """A context item"""
    path: str
    relevance: float
    kind: str  # 'file', 'function', 'class', 'test', etc.


class ContextPredictor:
    """Predict what context is needed for a task"""
    
    def __init__(self, project_root: str = "."):
        """Initialize the predictor
        
        Args:
            project_root: Root directory of the project
        """
        self.project_root = project_root
        self.file_index = {}
        self.dependency_graph = {}
    
    def index_project(self):
        """Index the project structure"""
        for root, dirs, files in os.walk(self.project_root):
            # Skip common non-source directories
            dirs[:] = [d for d in dirs if d not in {
                'node_modules', '.git', 'target', 'dist', 'build',
                '__pycache__', '.venv', 'venv', '.idea', '.vscode'
            }]
            
            for file in files:
                if self._is_source_file(file):
                    path = os.path.join(root, file)
                    rel_path = os.path.relpath(path, self.project_root)
                    self.file_index[rel_path] = {
                        'extension': os.path.splitext(file)[1],
                        'size': os.path.getsize(path),
                    }
    
    def _is_source_file(self, filename: str) -> bool:
        """Check if a file is a source file"""
        source_extensions = {
            '.py', '.js', '.ts', '.jsx', '.tsx', '.rs', '.go',
            '.java', '.c', '.cpp', '.h', '.hpp', '.cs', '.rb',
            '.php', '.swift', '.kt', '.scala', '.sh', '.sql',
        }
        ext = os.path.splitext(filename)[1]
        return ext in source_extensions
    
    def predict_context(self, task: str, task_type: str) -> List[ContextItem]:
        """Predict context for a task
        
        Args:
            task: Task description
            task_type: Type of task
            
        Returns:
            List of context items
        """
        context = []
        
        # Extract keywords from task
        keywords = self._extract_keywords(task)
        
        # Find relevant files
        for path, info in self.file_index.items():
            relevance = self._calculate_relevance(path, keywords, task_type)
            if relevance > 0:
                context.append(ContextItem(
                    path=path,
                    relevance=relevance,
                    kind='file'
                ))
        
        # Sort by relevance
        context.sort(key=lambda x: x.relevance, reverse=True)
        
        return context[:20]  # Return top 20
    
    def _extract_keywords(self, task: str) -> Set[str]:
        """Extract keywords from task description"""
        # Simple keyword extraction
        words = task.lower().split()
        
        # Filter out common words
        stop_words = {
            'the', 'a', 'an', 'is', 'are', 'was', 'were', 'be', 'been',
            'being', 'have', 'has', 'had', 'do', 'does', 'did', 'will',
            'would', 'could', 'should', 'may', 'might', 'must', 'shall',
            'can', 'need', 'dare', 'ought', 'used', 'to', 'of', 'in',
            'for', 'on', 'with', 'at', 'by', 'from', 'as', 'into',
            'through', 'during', 'before', 'after', 'above', 'below',
            'between', 'under', 'and', 'but', 'or', 'nor', 'not', 'so',
            'yet', 'both', 'either', 'neither', 'each', 'every', 'all',
            'any', 'few', 'more', 'most', 'other', 'some', 'such', 'no',
            'only', 'own', 'same', 'than', 'too', 'very', 'just', 'also',
        }
        
        keywords = set()
        for word in words:
            # Remove punctuation
            word = ''.join(c for c in word if c.isalnum())
            if word and word not in stop_words and len(word) > 2:
                keywords.add(word)
        
        return keywords
    
    def _calculate_relevance(self, path: str, keywords: Set[str], task_type: str) -> float:
        """Calculate relevance of a file to the task"""
        relevance = 0.0
        
        # Check filename
        filename = os.path.basename(path).lower()
        for keyword in keywords:
            if keyword in filename:
                relevance += 0.5
        
        # Check path
        path_lower = path.lower()
        for keyword in keywords:
            if keyword in path_lower:
                relevance += 0.3
        
        # Boost test files for testing tasks
        if task_type == 'testing' and 'test' in path_lower:
            relevance *= 2.0
        
        # Boost source files for bug fixes
        if task_type == 'bug_fix' and 'test' not in path_lower:
            relevance *= 1.5
        
        return relevance
    
    def get_related_files(self, file_path: str) -> List[str]:
        """Get files related to a given file
        
        Args:
            file_path: Path to the file
            
        Returns:
            List of related file paths
        """
        related = []
        
        # Find files in the same directory
        dir_path = os.path.dirname(file_path)
        for path in self.file_index:
            if os.path.dirname(path) == dir_path and path != file_path:
                related.append(path)
        
        # Find files with similar names
        basename = os.path.basename(file_path)
        name_without_ext = os.path.splitext(basename)[0]
        
        for path in self.file_index:
            if path == file_path:
                continue
            
            other_basename = os.path.basename(path)
            other_name = os.path.splitext(other_basename)[0]
            
            # Check for test files
            if 'test' in other_name and name_without_ext in other_name:
                related.append(path)
            elif 'test' in name_without_ext and other_name in name_without_ext:
                related.append(path)
        
        return related
