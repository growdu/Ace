import React from 'react';
import { Card as CardType } from '../types/game';
import './Card.css';

interface CardProps {
  card: CardType;
  selected?: boolean;
  onClick?: () => void;
  hidden?: boolean;
}

export const Card: React.FC<CardProps> = ({ card, selected, onClick, hidden }) => {
  if (hidden) {
    return <div className="card card-hidden" />;
  }

  const getSuitSymbol = (suit: string): string => {
    switch (suit) {
      case 'Spades': return '♠';
      case 'Hearts': return '♥';
      case 'Clubs': return '♣';
      case 'Diamonds': return '♦';
      default: return '';
    }
  };

  const getRankDisplay = (rank: string): string => {
    const map: Record<string, string> = {
      Three: '3', Four: '4', Five: '5', Six: '6', Seven: '7',
      Eight: '8', Nine: '9', Ten: '10', Jack: 'J', Queen: 'Q',
      King: 'K', Ace: 'A', Two: '2'
    };
    return map[rank] || '';
  };

  const isRed = card.suit === 'Hearts' || card.suit === 'Diamonds';
  const isJoker = card.joker;

  return (
    <div
      className={`card ${selected ? 'selected' : ''} ${isRed ? 'red' : 'black'}`}
      onClick={onClick}
    >
      {isJoker ? (
        <span className="joker">{card.joker === 'Large' ? '大王' : '小王'}</span>
      ) : (
        <>
          <div className="card-top">
            <span>{getRankDisplay(card.rank || '')}</span>
            <span>{getSuitSymbol(card.suit || '')}</span>
          </div>
          <div className="card-center">{getSuitSymbol(card.suit || '')}</div>
          <div className="card-bottom">
            <span>{getRankDisplay(card.rank || '')}</span>
            <span>{getSuitSymbol(card.suit || '')}</span>
          </div>
        </>
      )}
    </div>
  );
};
