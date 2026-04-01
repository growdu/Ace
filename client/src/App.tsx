import { useState } from 'react';
import { GameBoard } from './components/GameBoard';
import { auth, match } from './services/api';
import { useGame } from './hooks/useGame';

function App() {
  const [user, setUser] = useState<any>(null);
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [gameStarted, setGameStarted] = useState(false);
  const { connected, gameState, connect, sendMessage } = useGame();

  const handleLogin = async () => {
    try {
      const result = await auth.login(username, password);
      if (result.token) {
        setUser(result.user);
      }
    } catch (e) {
      console.error('登录失败', e);
    }
  };

  const handleStartMatch = async () => {
    if (!user) return;
    try {
      const result = await match.start(user.id, user.username, 'bot');
      if (result.room_id) {
        connect(result.room_id);
        setGameStarted(true);
      }
    } catch (e) {
      console.error('匹配失败', e);
    }
  };

  if (!user) {
    return (
      <div className="login-screen">
        <h1>腾冲百分</h1>
        <input
          placeholder="用户名"
          value={username}
          onChange={e => setUsername(e.target.value)}
        />
        <input
          type="password"
          placeholder="密码"
          value={password}
          onChange={e => setPassword(e.target.value)}
        />
        <button onClick={handleLogin}>登录</button>
      </div>
    );
  }

  if (!gameStarted) {
    return (
      <div className="lobby">
        <h1>欢迎, {user.username}</h1>
        <p>积分: {user.score}</p>
        <button onClick={handleStartMatch}>开始匹配</button>
        <p>状态: {connected ? '已连接' : '未连接'}</p>
      </div>
    );
  }

  return <GameBoard gameState={gameState} onPlayCard={(i) => sendMessage({ type: 'play_card', card_index: i })} onBid={(b) => sendMessage({ type: 'bid', bid: b })} />;
}

export default App;