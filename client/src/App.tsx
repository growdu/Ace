import { useState } from 'react';
import { GameBoard } from './components/GameBoard';
import { auth, match } from './services/api';
import { useGame } from './hooks/useGame';

function App() {
  const [user, setUser] = useState<any>(null);
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [gameStarted, setGameStarted] = useState(false);
  const {
    connected,
    gameState,
    connect,
    sendBid,
    sendPassBid,
    sendPlayCard,
    sendStartGame,
  } = useGame();

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

  const handleRegister = async () => {
    try {
      const result = await auth.register(username, password);
      if (result.token) {
        setUser(result.user);
      }
    } catch (e) {
      console.error('注册失败', e);
    }
  };

  const handleStartMatch = async () => {
    if (!user) return;
    try {
      const result = await match.start(user.id, user.username, 'bot');
      if (result.room_id) {
        connect(result.room_id);
        setGameStarted(true);
        // 延迟发送开始游戏消息，让 WebSocket 连接稳定
        setTimeout(() => {
          sendStartGame();
        }, 500);
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
        <div className="auth-buttons">
          <button onClick={handleLogin}>登录</button>
          <button onClick={handleRegister} className="register-btn">注册</button>
        </div>
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

  return (
    <GameBoard
      gameState={gameState as any}
      onPlayCard={sendPlayCard}
      onBid={sendBid}
      onPassBid={sendPassBid}
    />
  );
}

export default App;