import * as wasm from "../pkg/index.js";

export const allGames = [
  ["Baccarat", "baccarat"],
  ["Dice", "dice"],
  ["Limbo", "limbo"],
  ["Hilo", "hilo"],
  ["Blackjack", "blackjack"],
  ["Diamond Poker", "diamond_poker"],
  ["Roulette", "roulette"],
  ["Plinko", "plinko"],
  ["Mines", "mines"],
  ["Video Poker", "video_poker"],
  ["Wheel", "wheel"],
];

export function simulate(game, client_seed, server_seed, nonce, opts = {}) {
  return wasm.simulate(game, client_seed, server_seed, nonce, opts);
}
