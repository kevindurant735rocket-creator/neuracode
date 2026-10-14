"""
Semantic code embeddings using sentence transformers
"""

from typing import List, Optional
import numpy as np


class EmbeddingModel:
    """Generate semantic embeddings for code"""
    
    def __init__(self, model_name: str = "microsoft/codebert-base"):
        """Initialize the embedding model
        
        Args:
            model_name: Name of the sentence transformer model
        """
        self.model_name = model_name
        self._model = None
        self._tokenizer = None
        self.is_fallback = False
    
    def _load_model(self):
        """Lazy load the model"""
        if self._model is None and not self.is_fallback:
            try:
                from transformers import AutoTokenizer, AutoModel
                self._tokenizer = AutoTokenizer.from_pretrained(self.model_name)
                self._model = AutoModel.from_pretrained(self.model_name)
            except Exception as e:
                import warnings
                warnings.warn(f"EmbeddingModel falling back to hashing embed (no torch/transformers): {e}")
                self.is_fallback = True
    
    def embed(self, texts: List[str]) -> np.ndarray:
        """Generate embeddings for a list of texts
        
        Args:
            texts: List of text strings
            
        Returns:
            numpy array of embeddings
        """
        self._load_model()
        
        if self._model is None:
            # Fallback: simple bag-of-words embedding
            return self._simple_embed(texts)
        
        import torch
        
        embeddings = []
        for text in texts:
            inputs = self._tokenizer(
                text,
                return_tensors="pt",
                truncation=True,
                max_length=512,
                padding=True
            )
            
            with torch.no_grad():
                outputs = self._model(**inputs)
                # Use mean pooling
                embedding = outputs.last_hidden_state.mean(dim=1)
                embeddings.append(embedding.squeeze().numpy())
        
        return np.array(embeddings)
    
    def _simple_embed(self, texts: List[str]) -> np.ndarray:
        """Hashing-based fallback with fixed dim — stable across calls (no vocab mismatch)."""
        import hashlib
        dim = 384
        embeddings = []
        for text in texts:
            vec = np.zeros(dim, dtype=np.float64)
            for word in text.lower().split():
                h = int(hashlib.md5(word.encode()).hexdigest(), 16) % dim
                vec[h] += 1.0
            norm = np.linalg.norm(vec)
            if norm > 0:
                vec = vec / norm
            embeddings.append(vec)
        return np.array(embeddings)
    
    def similarity(self, emb1: np.ndarray, emb2: np.ndarray) -> float:
        """Calculate cosine similarity between two embeddings"""
        dot = np.dot(emb1, emb2)
        norm1 = np.linalg.norm(emb1)
        norm2 = np.linalg.norm(emb2)
        
        if norm1 == 0 or norm2 == 0:
            return 0.0
        
        return dot / (norm1 * norm2)
    
    def search(self, query: str, documents: List[str], top_k: int = 10) -> List[tuple]:
        """Search for most similar documents
        
        Args:
            query: Search query
            documents: List of documents to search
            top_k: Number of results to return
            
        Returns:
            List of (index, score) tuples
        """
        # Generate embeddings
        query_emb = self.embed([query])[0]
        doc_embs = self.embed(documents)
        
        # Calculate similarities
        similarities = []
        for i, doc_emb in enumerate(doc_embs):
            sim = self.similarity(query_emb, doc_emb)
            similarities.append((i, sim))
        
        # Sort by similarity
        similarities.sort(key=lambda x: x[1], reverse=True)
        
        return similarities[:top_k]
