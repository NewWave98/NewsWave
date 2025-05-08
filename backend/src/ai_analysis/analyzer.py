from transformers import pipeline
from typing import Dict
import numpy as np

class NewsAnalyzer:
    def __init__(self):
        self.sentiment_analyzer = pipeline(
            "sentiment-analysis",
            model="finiteautomata/bertweet-base-sentiment-analysis"
        )
        
    def analyze_news(self, news_item: Dict) -> Dict:
        """
        Analyze news content and predict market impact
        """
        # Perform sentiment analysis
        sentiment_result = self.sentiment_analyzer(news_item['content'])[0]
        
        # Calculate impact score (-1 to 1)
        impact_score = self._calculate_impact_score(
            sentiment_result['label'],
            sentiment_result['score']
        )
        
        # Generate prediction confidence
        confidence = self._calculate_confidence(sentiment_result['score'])
        
        return {
            'news_id': news_item['id'],
            'sentiment': sentiment_result['label'],
            'impact_score': impact_score,
            'confidence': confidence,
            'prediction': self._generate_prediction(impact_score)
        }
    
    def _calculate_impact_score(self, sentiment: str, score: float) -> float:
        """
        Convert sentiment to impact score
        """
        if sentiment == 'POSITIVE':
            return score
        elif sentiment == 'NEGATIVE':
            return -score
        return 0.0
    
    def _calculate_confidence(self, score: float) -> float:
        """
        Calculate prediction confidence
        """
        return min(max(score * 100, 0), 100)
    
    def _generate_prediction(self, impact_score: float) -> Dict:
        """
        Generate market predictions for different timeframes
        """
        return {
            '15min': self._predict_timeframe(impact_score, 0.15),
            '30min': self._predict_timeframe(impact_score, 0.3),
            '4h': self._predict_timeframe(impact_score, 0.5),
            '1d': self._predict_timeframe(impact_score, 0.7)
        }
    
    def _predict_timeframe(self, impact_score: float, weight: float) -> str:
        """
        Predict market trend for specific timeframe
        """
        weighted_score = impact_score * weight
        if weighted_score > 0.2:
            return 'uptrend'
        elif weighted_score < -0.2:
            return 'downtrend'
        return 'stable'