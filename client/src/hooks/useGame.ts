import { useState, useCallback, useRef } from 'react';

interface GameMessage {
  type: string;
  [key: string]: any;
}

interface Player {
  user_id: string;
  username: string;
  cards: any[];
  cards_count?: number;
  is_robot?: boolean;
}

interface GameState {
  phase?: string;
  current_player?: number;
  current_bid?: number;
  bidder?: number;
  trump_suit?: string;
  scores?: number[];
  round_number?: number;
  players?: Player[];
  lead_suit?: string;
  round_cards?: any[];
}

export function useGame() {
  const [connected, setConnected] = useState(false);
  const [gameState, setGameState] = useState<GameState | null>(null);
  const wsRef = useRef<WebSocket | null>(null);
  const userIdRef = useRef<string>('');

  const connect = useCallback((roomId: string, userId: string) => {
    userIdRef.current = userId;
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

  const handleMessage = useCallback((msg: GameMessage) => {
    console.log('Received:', msg);

    switch (msg.type) {
      case 'game_state':
        setGameState(prev => ({
          ...prev,
          phase: msg.phase,
          current_player: msg.current_player,
          current_bid: msg.current_bid,
          trump_suit: msg.trump_suit,
          scores: msg.scores,
          round_number: msg.round_number,
        }));
        break;

      case 'game_started':
        setGameState({
          phase: 'bidding',
          current_player: msg.current_player,
          current_bid: 0,
          bidder: undefined,
          trump_suit: undefined,
          scores: [0, 0],
          round_number: 0,
          players: msg.players,
        });
        break;

      case 'game_start':
        // 游戏开始（叫分完成）
        setGameState({
          phase: 'playing',
          current_player: msg.current_player,
          current_bid: 0,
          bidder: msg.bidder,
          trump_suit: msg.trump_suit,
          scores: [0, 0],
          round_number: 1,
          players: msg.players,
          lead_suit: undefined,
          round_cards: [null, null, null, null],
        });
        break;

      case 'bid_placed':
        setGameState(prev => prev ? ({
          ...prev,
          current_bid: msg.current_bid,
          current_player: msg.current_player,
        }) : null);
        break;

      case 'bid_passed':
        setGameState(prev => prev ? ({
          ...prev,
          current_player: msg.current_player,
        }) : null);
        break;

      case 'card_played':
        setGameState(prev => prev ? {
          ...prev,
          current_player: msg.current_player,
          lead_suit: msg.lead_suit,
          round_cards: prev.round_cards ? prev.round_cards.map((c, i) =>
            i === prev.players?.findIndex((p: Player) => p.user_id === msg.user_id) ? msg.card : c
          ) : undefined,
        } : null);
        break;

      case 'round_end':
        setGameState(prev => prev ? ({
          ...prev,
          scores: msg.scores,
          round_number: msg.round_number,
          current_player: msg.current_player,
          round_cards: [null, null, null, null],
        }) : null);
        break;

      case 'game_end':
        setGameState(prev => prev ? ({
          ...prev,
          phase: 'scoring',
          scores: msg.scores,
        }) : null);
        alert(`游戏结束！${msg.success ? '庄家获胜' : '闲家获胜'} - 得分: ${msg.bidder_score}/${msg.bid}`);
        break;

      default:
        console.log('Unknown message type:', msg.type);
    }
  }, []);

  const sendMessage = useCallback((msg: object) => {
    if (wsRef.current?.readyState === WebSocket.OPEN) {
      wsRef.current.send(JSON.stringify(msg));
      console.log('Sent:', msg);
    }
  }, []);

  const sendBid = useCallback((bid: number) => {
    sendMessage({ type: 'bid', user_id: userIdRef.current, bid });
  }, [sendMessage]);

  const sendPassBid = useCallback(() => {
    sendMessage({ type: 'pass_bid', user_id: userIdRef.current });
  }, [sendMessage]);

  const sendPlayCard = useCallback((cardIndex: number) => {
    sendMessage({ type: 'play_card', user_id: userIdRef.current, card_index: cardIndex });
  }, [sendMessage]);

  const sendStartGame = useCallback(() => {
    sendMessage({ type: 'start_game' });
  }, [sendMessage]);

  return {
    connected,
    gameState,
    connect,
    sendMessage,
    sendBid,
    sendPassBid,
    sendPlayCard,
    sendStartGame,
  };
}