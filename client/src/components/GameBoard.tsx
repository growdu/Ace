import React, { useState } from 'react';
import { HandCards } from './HandCards';
import { PlayerCard } from './PlayerCard';
import { GameState } from '../types/game';
import './GameBoard.css';

interface GameBoardProps {
  gameState: GameState;
  onPlayCard: (cardIndex: number) => void;
  onBid?: (bid: number) => void;
}

export const GameBoard: React.FC<GameBoardProps> = ({ gameState, onPlayCard }) => {
  const [selectedCard, setSelectedCard] = useState<number | null>(null);

  const handleCardClick = (index: number) => {
    setSelectedCard(index);
    onPlayCard(index);
  };

  return (
    <div className="game-board">
      {/* 顶部对手 */}
      <div className="player-top">
        <PlayerCard
          player={gameState.players[0]}
          isCurrent={gameState.currentPlayer === 0}
        />
      </div>

      <div className="game-middle">
        {/* 左侧对手 */}
        <div className="player-left">
          <PlayerCard
            player={gameState.players[3]}
            isCurrent={gameState.currentPlayer === 3}
          />
        </div>

        {/* 中央区域 - 桌面牌 */}
        <div className="table-area">
          <div className="round-cards">
            {gameState.roundCards.map((card, index) => (
              <div key={index} className={`table-card pos-${index}`}>
                {card ? `${card.rank?.slice(0, 1)}${card.suit?.slice(0, 1)}` : ''}
              </div>
            ))}
          </div>
          {gameState.trumpSuit && (
            <div className="trump-indicator">
              主牌: {gameState.trumpSuit}
            </div>
          )}
        </div>

        {/* 右侧对手 */}
        <div className="player-right">
          <PlayerCard
            player={gameState.players[1]}
            isCurrent={gameState.currentPlayer === 1}
          />
        </div>
      </div>

      {/* 底部 - 玩家手牌 */}
      <div className="player-bottom">
        <PlayerCard
          player={gameState.players[2]}
          isCurrent={gameState.currentPlayer === 2}
        />
        <HandCards
          cards={gameState.players[2].cards}
          selectedIndex={selectedCard}
          onSelectCard={handleCardClick}
        />
      </div>

      {/* 分数面板 */}
      <div className="score-panel">
        <div className="score-host">庄家: {gameState.scores[0]}</div>
        <div className="score-info">叫分: {gameState.currentBid}</div>
        <div className="score-xian">闲家: {gameState.scores[1]}</div>
      </div>
    </div>
  );
};
