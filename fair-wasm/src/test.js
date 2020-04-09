const log = (msg) => {
  const log = document.getElementById("log");
  log.innerHTML += `\n${msg}`;
};

const run = async () => {
  const fair = await import("./index.js");
  const { allGames } = fair;
  for (let [_, game] of allGames) {
    log(game);
    let res;
    if (game === "crash") {
      res = fair.simulate_multiplayer(
        game,
        "0000000000000000001b34dc6a1e86083f95500b096231436e9b25cbdd0075c4"
      );
    } else {
      res = fair.simulate(game, "client seed", "server seed", 1);
    }
    log(JSON.stringify(res, null));
  }
};

run().catch(console.error);
