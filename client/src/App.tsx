import { useState } from 'react';
import { GameBoard } from './components/GameBoard';
import { GameState } from './types/game';

function App() {
  const [gameState] = useState<GameState>({
    players: [
      { id: 0, name: '对手1', cards: [], isHost: false, isRobot: true, score: 0, position: 'top' },
      { id: 1, name: '对手2', cards: [], isHost: false, isRobot: true, score: 0, position: 'right' },
      { id: 2, name: '玩家', cards: [], isHost: false, isRobot: false, score: 0, position: 'bottom' },
      { id: 3, name: '对手3', cards: [], isHost: false, isRobot: true, score: 0, position: 'left' },
    ],
    currentPlayer: 0,
    bottomCards: [],
    currentBid: 75,
    bidder: null,
    trumpSuit: null,
    scores: [0, 0],
    roundCards: [null, null, null, null],
    roundWinner: null,
    phase: 'Waiting',
    roundNumber: 0,
  });

  const handlePlayCard = (cardIndex: number) => {
    console.log('Play card:', cardIndex);
    // TODO: 调用 Rust 后端
  };

  const handleBid = (bid: number) => {
    console.log('Bid:', bid);
    // TODO: 调用 Rust 后端
  };

  return (
    <div className="app">
      <GameBoard
        gameState={gameState}
        onPlayCard={handlePlayCard}
        onBid={handleBid}
      />
    </div>
  );
}

export default App;
