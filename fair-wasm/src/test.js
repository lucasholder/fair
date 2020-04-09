const log = (msg) => {
  const log = document.getElementById("log");
  log.innerHTML += `\n${msg}`;
};

const run = async () => {
  const fair = await import("./index.js");
  const { allGames, gameTypes } = fair;
  for (let game of allGames) {
    log(game.displayName);
    let opts;
    if (game.type === gameTypes.SINGLEPLAYER) {
      opts = {
        clientSeed: "client seed",
        serverSeed: "server seed",
        nonce: 1,
      };
    } else {
      opts = {
        gameHash:
          "0000000000000000001b34dc6a1e86083f95500b096231436e9b25cbdd0075c4",
      };
    }
    const res = fair.simulate(game.id, opts);
    log(JSON.stringify(res, null));
  }
};

run().catch(console.error);
