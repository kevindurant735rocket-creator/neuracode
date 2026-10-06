"""
NeuraCode AI - Advanced AI capabilities for code intelligence

This package provides:
- Semantic code embeddings
- Task classification
- Context prediction
- Multi-modal understanding
- Code analysis and recommendations
"""

__version__ = "0.1.0"

from .models.embeddings import EmbeddingModel
from .models.classifier import TaskClassifier
from .models.predictor import ContextPredictor
from .models.analyzer import PythonAnalyzer
from .models.summarizer import CodeSummarizer
from .models.recommender import CodeRecommender
from .multimodal.image import ImageUnderstanding
from .multimodal.diagram import DiagramParser

__all__ = [
    "EmbeddingModel",
    "TaskClassifier",
    "ContextPredictor",
    "PythonAnalyzer",
    "CodeSummarizer",
    "CodeRecommender",
    "ImageUnderstanding",
    "DiagramParser",
]
