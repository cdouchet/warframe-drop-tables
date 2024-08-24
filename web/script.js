import init, { greet, parse_warframe_data } from "./wasm_lib.js";

const getWarframeData = async () => {
  const response = await fetch("/droptables");
  return await response.text();
};

const addBox = (name) => {
  const wrapper = document.getElementById("wrapper");
  const box = document.createElement("div");
  box.id = name;
  // box.style.border = "3px solid white";
  // box.style.display = "flex";
  // box.style.justifyContent = "center";
  // box.style.alignItems = "center";
  box.className = "item";
  const text = document.createElement("p");
  text.innerText = name;
  text.style.color = "white";
  box.appendChild(text);
  wrapper.appendChild(box);
};

init().then(() => {
  getWarframeData().then((data) => {
    const result = parse_warframe_data(data);
    for (let i = 0; i < result.length; i++) {
      const element = result[i];
      let name;
      if ("Rotation" in element) {
        name = element.Rotation.name;
      } else {
        name = element.Classic.name;
      }
      addBox(name);
    }
    console.log(result);
  });
});
