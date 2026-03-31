import React from 'react';
import { Player } from '../types/game';
import './PlayerCard.css';

interface PlayerCardProps {
  player: Player;
  isCurrent: boolean;
}

export const PlayerCard: React.FC<PlayerCardProps> = ({ player, isCurrent }) => {
  return (
    <div className={`player-card ${isCurrent ? 'current' : ''}`}>
      <div className="player-avatar">
        {player.isRobot ? '🤖' : '👤'}
      </div>
      <div className="player-info">
        <div className="player-name">{player.name}</div>
        <div className="player-cards-count">🃏 {player.cards.length}</div>
        <div className="player-score">🏆 {player.score}</div>
      </div>
      {player.isHost && <div className="host-badge">庄</div>}
    </div>
  );
};
