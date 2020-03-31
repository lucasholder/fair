const log = (msg) => {
  const log = document.getElementById("log");
  log.innerHTML += `\n${msg}`;
};

const run = async () => {
  const fair = await import("./index.js");
  console.log("fair", fair);
  log(
    fair.simulate("plinko", "client seed", "server seed", 1, {
      rows: 12,
      // risk: "high",
    })
  );
};

run().catch(console.error);
