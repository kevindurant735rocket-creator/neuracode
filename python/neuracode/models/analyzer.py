"""
Code analyzer - Deep code analysis using AST
"""

import ast
import os
from typing import Dict, List, Set, Optional, Any
from dataclasses import dataclass, field
from pathlib import Path
import re


@dataclass
class CodeMetrics:
    """Code quality metrics"""
    lines_of_code: int = 0
    comment_lines: int = 0
    blank_lines: int = 0
    functions: int = 0
    classes: int = 0
    imports: int = 0
    complexity: float = 0.0


@dataclass
class FunctionInfo:
    """Information about a function"""
    name: str
    lineno: int
    end_lineno: int
    args: List[str]
    returns: Optional[str]
    decorators: List[str]
    is_async: bool
    is_method: bool
    docstring: Optional[str]
    metrics: CodeMetrics


@dataclass
class ClassInfo:
    """Information about a class"""
    name: str
    lineno: int
    end_lineno: int
    bases: List[str]
    methods: List[str]
    attributes: List[str]
    docstring: Optional[str]


@dataclass
class FileAnalysis:
    """Complete analysis of a file"""
    path: str
    language: str
    metrics: CodeMetrics
    functions: List[FunctionInfo] = field(default_factory=list)
    classes: List[ClassInfo] = field(default_factory=list)
    imports: List[str] = field(default_factory=list)
    issues: List[str] = field(default_factory=list)


class PythonAnalyzer:
    """Analyze Python source code"""
    
    def __init__(self):
        self.issues: List[str] = []
    
    def analyze_file(self, path: str) -> Optional[FileAnalysis]:
        """Analyze a Python file"""
        try:
            with open(path, 'r', encoding='utf-8') as f:
                source = f.read()
            return self.analyze_source(source, path)
        except Exception as e:
            return None
    
    def analyze_source(self, source: str, path: str = "<unknown>") -> FileAnalysis:
        """Analyze Python source code"""
        self.issues = []
        
        try:
            tree = ast.parse(source)
        except SyntaxError as e:
            self.issues.append(f"Syntax error: {e}")
            return FileAnalysis(
                path=path,
                language="python",
                metrics=CodeMetrics(),
                issues=self.issues
            )
        
        # Calculate metrics
        metrics = self._calculate_metrics(source)
        
        # Extract functions
        functions = self._extract_functions(tree)
        
        # Extract classes
        classes = self._extract_classes(tree)
        
        # Extract imports
        imports = self._extract_imports(tree)
        
        # Update metrics
        metrics.functions = len(functions)
        metrics.classes = len(classes)
        metrics.imports = len(imports)
        
        # Check for issues
        self._check_issues(tree, source)
        
        return FileAnalysis(
            path=path,
            language="python",
            metrics=metrics,
            functions=functions,
            classes=classes,
            imports=imports,
            issues=self.issues
        )
    
    def _calculate_metrics(self, source: str) -> CodeMetrics:
        """Calculate code metrics"""
        lines = source.split('\n')
        
        loc = len(lines)
        blank = sum(1 for line in lines if not line.strip())
        comment = sum(1 for line in lines if line.strip().startswith('#'))
        
        return CodeMetrics(
            lines_of_code=loc,
            comment_lines=comment,
            blank_lines=blank,
            complexity=0.0
        )
    
    def _extract_functions(self, tree: ast.AST) -> List[FunctionInfo]:
        """Extract function definitions"""
        functions = []
        
        for node in ast.walk(tree):
            if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
                func = self._parse_function(node)
                functions.append(func)
        
        return functions
    
    def _parse_function(self, node: ast.FunctionDef) -> FunctionInfo:
        """Parse a function definition"""
        args = [arg.arg for arg in node.args.args]
        returns = None
        if node.returns:
            returns = ast.unparse(node.returns)
        
        decorators = []
        for dec in node.decorator_list:
            try:
                decorators.append(ast.unparse(dec))
            except:
                pass
        
        docstring = ast.get_docstring(node)
        
        # Calculate function metrics
        func_source = ast.unparse(node)
        metrics = self._calculate_metrics(func_source)
        
        return FunctionInfo(
            name=node.name,
            lineno=node.lineno,
            end_lineno=node.end_lineno or node.lineno,
            args=args,
            returns=returns,
            decorators=decorators,
            is_async=isinstance(node, ast.AsyncFunctionDef),
            is_method=False,  # Will be updated later
            docstring=docstring,
            metrics=metrics
        )
    
    def _extract_classes(self, tree: ast.AST) -> List[ClassInfo]:
        """Extract class definitions"""
        classes = []
        
        for node in ast.walk(tree):
            if isinstance(node, ast.ClassDef):
                cls = self._parse_class(node)
                classes.append(cls)
        
        return classes
    
    def _parse_class(self, node: ast.ClassDef) -> ClassInfo:
        """Parse a class definition"""
        bases = []
        for base in node.bases:
            try:
                bases.append(ast.unparse(base))
            except:
                pass
        
        methods = []
        attributes = []
        
        for item in node.body:
            if isinstance(item, (ast.FunctionDef, ast.AsyncFunctionDef)):
                methods.append(item.name)
            elif isinstance(item, ast.AnnAssign) and isinstance(item.target, ast.Name):
                attributes.append(item.target.id)
            elif isinstance(item, ast.Assign):
                for target in item.targets:
                    if isinstance(target, ast.Name):
                        attributes.append(target.id)
        
        docstring = ast.get_docstring(node)
        
        return ClassInfo(
            name=node.name,
            lineno=node.lineno,
            end_lineno=node.end_lineno or node.lineno,
            bases=bases,
            methods=methods,
            attributes=attributes,
            docstring=docstring
        )
    
    def _extract_imports(self, tree: ast.AST) -> List[str]:
        """Extract import statements"""
        imports = []
        
        for node in ast.walk(tree):
            if isinstance(node, ast.Import):
                for alias in node.names:
                    imports.append(alias.name)
            elif isinstance(node, ast.ImportFrom):
                module = node.module or ""
                for alias in node.names:
                    imports.append(f"{module}.{alias.name}")
        
        return imports
    
    def _check_issues(self, tree: ast.AST, source: str):
        """Check for common issues"""
        # Check for unused imports
        imports = self._extract_imports(tree)
        for imp in imports:
            name = imp.split('.')[-1]
            if name not in source.replace(f"import {name}", ""):
                self.issues.append(f"Potentially unused import: {imp}")
        
        # Check for missing docstrings
        for node in ast.walk(tree):
            if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
                if not ast.get_docstring(node):
                    if not node.name.startswith('_'):
                        self.issues.append(
                            f"Missing docstring: {node.name} at line {node.lineno}"
                        )
            elif isinstance(node, ast.ClassDef):
                if not ast.get_docstring(node):
                    self.issues.append(
                        f"Missing docstring: {node.name} at line {node.lineno}"
                    )
        
        # Check for long functions
        for node in ast.walk(tree):
            if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
                length = (node.end_lineno or node.lineno) - node.lineno
                if length > 50:
                    self.issues.append(
                        f"Long function: {node.name} ({length} lines)"
                    )
