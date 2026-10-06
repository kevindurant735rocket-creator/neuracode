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
    
    def _load_model(self):
        """Lazy load the model"""
        if self._model is None:
            try:
                from transformers import AutoTokenizer, AutoModel
                self._tokenizer = AutoTokenizer.from_pretrained(self.model_name)
                self._model = AutoModel.from_pretrained(self.model_name)
            except Exception as e:
                print(f"Warning: Could not load model {self.model_name}: {e}")
                print("Falling back to simple embeddings")
    
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
        """Simple bag-of-words embedding as fallback"""
        # Create a simple vocabulary
        vocab = {}
        for text in texts:
            for word in text.lower().split():
                if word not in vocab:
                    vocab[word] = len(vocab)
        
        # Create embeddings
        embeddings = []
        for text in texts:
            embedding = np.zeros(len(vocab))
            for word in text.lower().split():
                if word in vocab:
                    embedding[vocab[word]] += 1
            
            # Normalize
            norm = np.linalg.norm(embedding)
            if norm > 0:
                embedding = embedding / norm
            
            embeddings.append(embedding)
        
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
