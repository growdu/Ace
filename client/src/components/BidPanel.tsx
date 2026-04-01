import React from 'react';
import './BidPanel.css';

interface BidPanelProps {
  currentBid: number;
  currentPlayer: number;
  myPlayerId?: string;
  onBid: (bid: number) => void;
  onPass: () => void;
  players?: { user_id: string }[];
}

const BID_OPTIONS = [75, 85, 100, 120, 150];

export const BidPanel: React.FC<BidPanelProps> = ({
  currentBid,
  currentPlayer,
  myPlayerId,
  onBid,
  onPass,
  players = [],
}) => {
  const myIndex = players.findIndex(p => p.user_id === myPlayerId);
  const isMyTurn = myIndex !== -1 && currentPlayer === myIndex;

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