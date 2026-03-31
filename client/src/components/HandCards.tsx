import React from 'react';
import { Card as CardType } from '../types/game';
import { Card } from './Card';
import './HandCards.css';

interface HandCardsProps {
  cards: CardType[];
  selectedIndex: number | null;
  onSelectCard: (index: number) => void;
}

export const HandCards: React.FC<HandCardsProps> = ({ cards, selectedIndex, onSelectCard }) => {
  return (
    <div className="hand-cards">
      {cards.map((card, index) => (
        <div key={index} className="hand-card-wrapper">
          <Card
            card={card}
            selected={index === selectedIndex}
            onClick={() => onSelectCard(index)}
          />
        </div>
      ))}
    </div>
  );
};
