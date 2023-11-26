import SDK from "../index";

const sdk = new SDK();

sdk.on("connect", () => {
  console.log("Connected!");
  sdk.getServerStats();
});

sdk.on("ServerStatEvent", (data) => {
  console.log("ServerStatEventT:", data);
});

sdk.on("error", (error) => {
  console.log("Error:", error);
});

sdk.on("disconnect", (event) => {
  console.log("Disconnected:", event);
});

await sdk.connect();
