import { useState, useCallback, useRef } from 'react';

interface GameMessage {
  type: string;
  [key: string]: any;
}

export function useGame() {
  const [connected, setConnected] = useState(false);
  const [gameState, setGameState] = useState<any>(null);
  const wsRef = useRef<WebSocket | null>(null);

  const connect = useCallback((roomId: string) => {
    const ws = new WebSocket(`ws://localhost:8080/ws/game/${roomId}`);

    ws.onopen = () => {
      setConnected(true);
      console.log('WebSocket connected');
    };

    ws.onmessage = (event) => {
      const msg: GameMessage = JSON.parse(event.data);
      handleMessage(msg);
    };

    ws.onclose = () => {
      setConnected(false);
      console.log('WebSocket disconnected');
    };

    wsRef.current = ws;
  }, []);

  const handleMessage = (msg: GameMessage) => {
    switch (msg.type) {
      case 'game_start':
        setGameState({ phase: 'playing', ...msg });
        break;
      case 'deal_cards':
        break;
      case 'your_turn':
        break;
      case 'round_end':
        break;
      case 'game_end':
        break;
    }
  };

  const sendMessage = useCallback((msg: object) => {
    if (wsRef.current?.readyState === WebSocket.OPEN) {
      wsRef.current.send(JSON.stringify(msg));
    }
  }, []);

  return { connected, gameState, connect, sendMessage };
}