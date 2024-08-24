const express = require("express");
const request = require("request");
const app = express();

// app.use((req, res, next) => {
//   res.set("Cross-Origin-Opener-Policy", "same-origin");
//   res.set("Cross-Origin-Resource-Policy", "same-site");
//   res.set("Cross-Origin-Embedder-Policy", "require-corp");
// });

app.get("/droptables", (req, res) => {
  const url = "https://warframe.com/droptables";
  request({ url }).pipe(res);
});

app.use(express.static("web"));
app.use(express.static("wasm_lib/pkg"));

app.listen(8769, () => {
  console.log("Starting Warframe Drop Tables");
});
