import React, { useEffect, useState } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { NewsItem, PredictionData } from '../types';
import { fetchNews, submitVote } from '../services/api';

interface NewsFeedProps {
  onVoteSubmit: (predictionId: string, vote: boolean) => void;
}

export const NewsFeed: React.FC<NewsFeedProps> = ({ onVoteSubmit }) => {
  const [news, setNews] = useState<NewsItem[]>([]);
  const [loading, setLoading] = useState(true);
  const { connected } = useWallet();

  useEffect(() => {
    const loadNews = async () => {
      try {
        const newsData = await fetchNews();
        setNews(newsData);
      } catch (error) {
        console.error('Error loading news:', error);
      } finally {
        setLoading(false);
      }
    };

    loadNews();
    const interval = setInterval(loadNews, 60000); // Refresh every minute
    return () => clearInterval(interval);
  }, []);

  const handleVote = async (predictionId: string, vote: boolean) => {
    if (!connected) {
      alert('Please connect your wallet first');
      return;
    }

    try {
      await submitVote(predictionId, vote);
      onVoteSubmit(predictionId, vote);
    } catch (error) {
      console.error('Error submitting vote:', error);
      alert('Failed to submit vote');
    }
  };

  if (loading) {
    return <div>Loading news feed...</div>;
  }

  return (
    <div className="news-feed">
      {news.map((item) => (
        <div key={item.id} className="news-item">
          <h3>{item.title}</h3>
          <p>{item.content}</p>
          <div className="prediction-data">
            <h4>Market Predictions</h4>
            <div className="timeframes">
              {Object.entries(item.predictions).map(([timeframe, prediction]) => (
                <div key={timeframe} className="timeframe">
                  <span>{timeframe}: {prediction.trend}</span>
                  <span>Confidence: {prediction.confidence}%</span>
                </div>
              ))}
            </div>
          </div>
          <div className="voting-buttons">
            <button
              onClick={() => handleVote(item.id, true)}
              disabled={!connected}
            >
              Agree
            </button>
            <button
              onClick={() => handleVote(item.id, false)}
              disabled={!connected}
            >
              Disagree
            </button>
          </div>
        </div>
      ))}
    </div>
  );
};