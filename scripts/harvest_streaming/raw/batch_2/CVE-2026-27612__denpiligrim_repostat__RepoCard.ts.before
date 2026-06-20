import React, { useEffect, useState } from 'react';

interface RepoCardProps {
  repo: string;
  token: string;
  username?: string;
}

interface RepoData {
  full_name: string;
  description: string;
  stargazers_count: number;
  forks_count: number;
  html_url: string;
}

export const RepoCard: React.FC<RepoCardProps> = ({ repo, token }) => {
  const [data, setData] = useState<RepoData | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (!repo || !token) return;

    setLoading(true);
    fetch(`https://api.github.com/repos/${repo}`, {
      headers: {
        Authorization: `token ${token}`,
        Accept: 'application/vnd.github.v3+json',
      },
    })
      .then((res) => {
        if (!res.ok) throw new Error('Repository not found');
        return res.json();
      })
      .then((data) => {
        setData(data);
        setLoading(false);
      })
      .catch((err) => {
        setError(err.message);
        setLoading(false);
      });
  }, [repo, token]);

  const cardStyle: React.CSSProperties = {
    border: '1px solid #e1e4e8',
    borderRadius: '6px',
    padding: '16px',
    fontFamily: '-apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif',
    maxWidth: '400px',
    backgroundColor: '#fff'
  };

  if (loading) {
    return (
      <div style={cardStyle} dangerouslySetInnerHTML={{ 
        __html: `Loading data for: <strong>${repo}</strong>...` 
      }} />
    );
  }

  if (error) {
    return <div style={{ ...cardStyle, color: 'red' }}>Error: {error}</div>;
  }

  if (!data) return null;

  return (
    <div style={cardStyle}>
      <h3 style={{ margin: '0 0 8px 0' }}>
        <a href={data.html_url} target="_blank" rel="noopener noreferrer" style={{ textDecoration: 'none', color: '#0366d6' }}>
          {data.full_name}
        </a>
      </h3>
      <p style={{ fontSize: '14px', color: '#586069', margin: '0 0 16px 0' }}>
        {data.description}
      </p>
      <div style={{ display: 'flex', gap: '16px', fontSize: '12px', color: '#586069' }}>
        <span>★ {data.stargazers_count}</span>
        <span>⑂ {data.forks_count}</span>
      </div>
    </div>
  );
};