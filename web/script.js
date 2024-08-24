import init, {
  greet,
  parse_warframe_data,
  filter_warframe_missions,
} from "./wasm_lib.js";

const getWarframeData = async () => {
  const response = await fetch("/droptables");
  return await response.text();
};

const addBox = (element, inputText) => {
  let name;
  if ("Rotation" in element) {
    name = element.Rotation.name;
  } else {
    name = element.Classic.name;
  }
  const wrapper = document.getElementById("wrapper");
  let box = document.createElement("div");
  box.id = name;
  box.className = "mission";
  const text = document.createElement("p");
  text.innerText = name;
  text.style.color = "white";
  text.style.paddingBottom = "15px";
  text.style.fontSize = "1.1em";
  text.style.fontWeight = "bold";
  box.appendChild(text);
  if ("Rotation" in element) {
    box = addRotation(box, element, inputText);
  } else {
    box = addClassic(box, element, inputText);
  }
  wrapper.appendChild(box);
};

const addRotation = (box, el, inputText) => {
  const element = el.Rotation;
  for (let key of Object.keys(element.rotations)) {
    const itemRotation = document.createElement("p");
    itemRotation.innerText = `Rotation ${key.toUpperCase()}`;
    itemRotation.style.color = "white";
    itemRotation.style.paddingBottom = "15px";
    itemRotation.style.fontSize = "1em";
    itemRotation.style.fontWeight = "bold";
    box.appendChild(itemRotation);
    for (const item of element.rotations[key]) {
      const itemWrapper = document.createElement("div");
      const itemName = document.createElement("p");
      const itemChance = document.createElement("p");
      itemWrapper.className = "item";
      const chance =
        Math.trunc(item.drop_chance * Math.pow(10, 2)) / Math.pow(10, 2);
      itemName.innerText = item.name;
      if (item.name.toLowerCase().includes(inputText)) {
        itemName.style.backgroundColor = "rgba(150, 150, 200, 1)";
      }
      itemChance.innerText = `${chance}%`;
      itemWrapper.appendChild(itemName);
      itemWrapper.appendChild(itemChance);
      box.appendChild(itemWrapper);
    }
  }
  return box;
};

const addClassic = (box, el, inputText) => {
  const element = el.Classic;
  for (const item of element.items) {
    const itemWrapper = document.createElement("div");
    const itemName = document.createElement("p");
    const itemChance = document.createElement("p");
    itemWrapper.className = "item";
    const chance =
      Math.trunc(item.drop_chance * Math.pow(10, 2)) / Math.pow(10, 2);
    itemName.innerText = item.name;
    if (item.name.toLowerCase().includes(inputText)) {
      itemName.style.backgroundColor = "rgba(150, 150, 200, 1)";
    }
    itemChance.innerText = `${chance}%`;
    itemWrapper.appendChild(itemName);
    itemWrapper.appendChild(itemChance);
    box.appendChild(itemWrapper);
  }
  return box;
};

let finalData;

const showData = (data, inputText) => {
  const wrapper = document.getElementById("wrapper");
  wrapper.textContent = "";
  for (let i = 0; i < data.length; i++) {
    const element = data[i];
    addBox(element, inputText.toLowerCase());
  }
};

const searchInput = document.getElementById("input_search");

init().then(() => {
  getWarframeData().then((data) => {
    const result = parse_warframe_data(data);
    finalData = result;
    showData(finalData, "NOTHING");
    searchInput.addEventListener("input", (e) => {
      const inputText = e.target.value;
      if (inputText == "" || inputText == null) {
        showData(finalData, "NOTHING");
        return;
      }
      const result = filter_warframe_missions(finalData, inputText);
      showData(result, inputText);
    });
  });
});
