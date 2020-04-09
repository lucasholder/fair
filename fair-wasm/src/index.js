import * as wasm from "../pkg/index.js";

export const allGames = [
  ["Baccarat", "baccarat"],
  ["Blackjack", "blackjack"],
  ["Crash", "crash"],
  ["Diamond Poker", "diamond_poker"],
  ["Dice", "dice"],
  ["Hilo", "hilo"],
  ["Keno", "keno"],
  ["Limbo", "limbo"],
  ["Mines", "mines"],
  ["Plinko", "plinko"],
  ["Roulette", "roulette"],
  ["Slots", "slots"],
  ["Video Poker", "video_poker"],
  ["Wheel", "wheel"],
];

export function simulate(game, client_seed, server_seed, nonce, opts = {}) {
  return wasm.simulate(game, client_seed, server_seed, nonce, opts);
}

export function simulate_multiplayer(game, game_hash, opts = {}) {
  return wasm.simulate_multiplayer(game, game_hash, opts);
}
