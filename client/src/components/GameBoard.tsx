import React, { useState } from 'react';
import { HandCards } from './HandCards';
import { PlayerCard } from './PlayerCard';
import { BidPanel } from './BidPanel';
import './GameBoard.css';

interface Player {
  user_id: string;
  username: string;
  cards?: any[];
  cards_count?: number;
  is_robot?: boolean;
}

interface GameState {
  phase: string;
  current_player: number;
  current_bid: number;
  bidder?: number;
  trump_suit?: string;
  scores: number[];
  round_number: number;
  players: Player[];
  lead_suit?: string;
  round_cards?: any[];
}

interface GameBoardProps {
  gameState: GameState | null;
  onPlayCard: (cardIndex: number) => void;
  onBid?: (bid: number) => void;
  onPassBid?: () => void;
}

export const GameBoard: React.FC<GameBoardProps> = ({
  gameState,
  onPlayCard,
  onBid,
  onPassBid,
}) => {
  const [selectedCard, setSelectedCard] = useState<number | null>(null);
  const [confirming, setConfirming] = useState(false);

  if (!gameState) {
    return (
      <div className="game-board">
        <div className="waiting">等待游戏开始...</div>
      </div>
    );
  }

  // 转换后端玩家数据到前端格式
  const players = gameState.players.map((p, i) => ({
    id: i,
    name: p.username,
    cards: p.cards || [],
    cardsCount: p.cards_count || 0,
    isHost: i === gameState.bidder,
    isRobot: p.is_robot || false,
    score: 0,
    position: ['top', 'right', 'bottom', 'left'][i] as 'top' | 'right' | 'bottom' | 'left',
  }));

  const handleCardClick = (index: number) => {
    setSelectedCard(index);
    setConfirming(true);
  };

  const handleConfirm = () => {
    if (selectedCard !== null) {
      onPlayCard(selectedCard);
      setSelectedCard(null);
      setConfirming(false);
    }
  };

  const handleCancel = () => {
    setSelectedCard(null);
    setConfirming(false);
  };

  // 叫分阶段
  const showBidPanel = gameState.phase === 'bidding' && onBid && onPassBid;

  return (
    <div className="game-board">
      {/* 顶部对手 */}
      <div className="player-top">
        <PlayerCard
          player={players[0]}
          isCurrent={gameState.current_player === 0}
        />
      </div>

      <div className="game-middle">
        {/* 左侧对手 */}
        <div className="player-left">
          <PlayerCard
            player={players[3]}
            isCurrent={gameState.current_player === 3}
          />
        </div>

        {/* 中央区域 */}
        <div className="table-area">
          <div className="round-cards">
            {(gameState.round_cards || []).map((card, index) => (
              <div key={index} className={`table-card pos-${index}`}>
                {card ? `${card.rank?.toString()?.slice(0,1) || ''}${card.suit?.toString()?.slice(0,1) || ''}` : ''}
              </div>
            ))}
          </div>
          {gameState.trump_suit && (
            <div className="trump-indicator">
              主牌: {gameState.trump_suit}
            </div>
          )}
          <div className="phase-indicator">
            阶段: {gameState.phase === 'bidding' ? '叫分' : gameState.phase === 'playing' ? '打牌' : gameState.phase}
          </div>
        </div>

        {/* 右侧对手 */}
        <div className="player-right">
          <PlayerCard
            player={players[1]}
            isCurrent={gameState.current_player === 1}
          />
        </div>
      </div>

      {/* 底部 - 玩家手牌 */}
      <div className="player-bottom">
        <PlayerCard
          player={players[2]}
          isCurrent={gameState.current_player === 2}
        />
        {players[2].cards && players[2].cards.length > 0 ? (
          <>
            <HandCards
              cards={players[2].cards}
              selectedIndex={selectedCard}
              onSelectCard={handleCardClick}
            />
            {confirming && (
              <div className="confirm-dialog">
                <button className="confirm-btn" onClick={handleConfirm}>确认出牌</button>
                <button className="cancel-btn" onClick={handleCancel}>取消</button>
              </div>
            )}
          </>
        ) : (
          <div className="waiting-cards">等待发牌...</div>
        )}
      </div>

      {/* 叫分面板 */}
      {showBidPanel && (
        <BidPanel
          currentBid={gameState.current_bid}
          currentPlayer={gameState.current_player}
          myPlayerId="player_1"
          onBid={onBid!}
          onPass={onPassBid!}
        />
      )}

      {/* 分数面板 */}
      <div className="score-panel">
        <div className="score-host">庄家: {gameState.scores[0]}</div>
        <div className="score-info">叫分: {gameState.current_bid}</div>
        <div className="score-xian">闲家: {gameState.scores[1]}</div>
        <div className="round-info">回合: {gameState.round_number}</div>
      </div>
    </div>
  );
};