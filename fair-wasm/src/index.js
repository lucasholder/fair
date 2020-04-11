import * as wasm from "../pkg/index.js";

export const gameTypes = {
  SINGLEPLAYER: "SINGLEPLAYER",
  MULTIPLAYER: "MULTIPLAYER",
};

const { SINGLEPLAYER, MULTIPLAYER } = gameTypes;

export const allGames = [
  { id: "baccarat", displayName: "Baccarat", type: SINGLEPLAYER },
  { id: "blackjack", displayName: "Blackjack", type: SINGLEPLAYER },
  { id: "crash", displayName: "Crash", type: MULTIPLAYER },
  { id: "diamond_poker", displayName: "Diamond Poker", type: SINGLEPLAYER },
  { id: "dice", displayName: "Dice", type: SINGLEPLAYER },
  { id: "hilo", displayName: "Hilo", type: SINGLEPLAYER },
  { id: "keno", displayName: "Keno", type: SINGLEPLAYER },
  { id: "limbo", displayName: "Limbo", type: SINGLEPLAYER },
  {
    id: "mines",
    displayName: "Mines",
    type: SINGLEPLAYER,
    defaultOpts: { mines: 3 },
  },
  {
    id: "plinko",
    displayName: "Plinko",
    type: SINGLEPLAYER,
    defaultOpts: {
      rows: 8,
      risk: "low",
    },
  },
  { id: "roulette", displayName: "Roulette", type: SINGLEPLAYER },
  {
    id: "slots",
    displayName: "Slots",
    type: SINGLEPLAYER,
    defaultOpts: { round: 0 },
  },
  { id: "video_poker", displayName: "Video Poker", type: SINGLEPLAYER },
  {
    id: "wheel",
    displayName: "Wheel",
    type: SINGLEPLAYER,
    defaultOpts: {
      risk: "low",
      segments: 10,
    },
  },
];

export function simulate(gameId, opts = {}) {
  const { type } = allGames.find(({ id }) => id === gameId);

  if (type === SINGLEPLAYER) {
    return simulateSingleplayer(gameId, opts);
  } else {
    return simulateMultiplayer(gameId, opts);
  }
}

export function simulateSingleplayer(gameId, opts = {}) {
  const { clientSeed, serverSeed, nonce, ...otherOpts } = opts;
  return wasm.simulate(gameId, clientSeed, serverSeed, nonce, otherOpts);
}

export function simulateMultiplayer(gameId, opts = {}) {
  const { gameHash, ...otherOpts } = opts;
  return wasm.simulate_multiplayer(gameId, gameHash, otherOpts);
}

export function hashServerSeed(serverSeed) {
  return wasm.hash_server_seed(serverSeed);
}

export function verifyGameHashStake(gameHash) {
  return wasm.verify_game_hash_stake(gameHash);
}
