export type Suit = 'Spades' | 'Hearts' | 'Clubs' | 'Diamonds';
export type Rank = 'Three' | 'Four' | 'Five' | 'Six' | 'Seven' | 'Eight' | 'Nine' | 'Ten' | 'Jack' | 'Queen' | 'King' | 'Ace' | 'Two';
export type Joker = 'Small' | 'Large';
export type GamePhase = 'Waiting' | 'Dealing' | 'Bidding' | 'Playing' | 'Scoring';
export type BotLevel = 'Easy' | 'Normal' | 'Hard';

export interface Card {
  suit: Suit | null;
  rank: Rank | null;
  joker: Joker | null;
}

export interface Player {
  id: number;
  name: string;
  cards: Card[];
  isHost: boolean;
  isRobot: boolean;
  score: number;
  position: 'top' | 'left' | 'right' | 'bottom';
}

export interface GameState {
  players: Player[];
  currentPlayer: number;
  bottomCards: Card[];
  currentBid: number;
  bidder: number | null;
  trumpSuit: Suit | null;
  scores: [number, number];
  roundCards: (Card | null)[];
  roundWinner: number | null;
  phase: GamePhase;
  roundNumber: number;
}
