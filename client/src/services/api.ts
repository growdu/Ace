const API_BASE = 'http://localhost:8080';

export interface User {
  id: string;
  username: string;
  score: number;
  wins: number;
  losses: number;
}

export interface LoginResponse {
  token: string;
  user: User;
}

export const auth = {
  register: async (username: string, password: string) => {
    const res = await fetch(`${API_BASE}/api/auth/register`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username, password }),
    });
    return res.json();
  },

  login: async (username: string, password: string) => {
    const res = await fetch(`${API_BASE}/api/auth/login`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username, password }),
    });
    return res.json();
  },
};

export const match = {
  start: async (userId: string, username: string, mode: string) => {
    const res = await fetch(`${API_BASE}/api/match`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ user_id: userId, username, mode }),
    });
    return res.json();
  },
};