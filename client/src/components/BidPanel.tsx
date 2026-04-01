import React from 'react';
import './BidPanel.css';

interface BidPanelProps {
  currentBid: number;
  currentPlayer: number;
  myPlayerId?: string;
  onBid: (bid: number) => void;
  onPass: () => void;
}

const BID_OPTIONS = [75, 85, 100, 120, 150];

export const BidPanel: React.FC<BidPanelProps> = ({
  currentBid,
  currentPlayer,
  onBid,
  onPass,
}) => {
  const isMyTurn = currentPlayer === 0; // 简化：假设玩家是0号

  return (
    <div className="bid-panel">
      <div className="bid-info">
        <span>当前叫分: <strong>{currentBid}</strong></span>
      </div>

      {isMyTurn && (
        <div className="bid-actions">
          <div className="bid-buttons">
            {BID_OPTIONS.filter(b => b > currentBid).map(bid => (
              <button
                key={bid}
                className="bid-btn"
                onClick={() => onBid(bid)}
              >
                {bid}
              </button>
            ))}
          </div>
          <button className="pass-btn" onClick={onPass}>
            不叫
          </button>
        </div>
      )}

      {!isMyTurn && (
        <div className="waiting-message">
          等待其他玩家叫分...
        </div>
      )}
    </div>
  );
};