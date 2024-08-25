import init, { greet, WarframeData } from "./wasm_lib.js";

const getWarframeData = async () => {
  const response = await fetch("/droptables");
  return await response.text();
};

const createBox = (name) => {
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
  return box;
};

const addMission = (element, inputText) => {
  const fragment = document.createDocumentFragment();
  const name = element.name;
  let box = createBox(name);
  if (element.mission_type == 0) {
    box = addRotations(box, element, inputText);
  } else {
    box = addClassic(box, element, inputText);
  }

  fragment.appendChild(box);
  document.getElementById("wrapper").appendChild(fragment);
};

const addRotation = (rotationKey, box) => {
  const itemRotation = document.createElement("p");
  itemRotation.innerText = `Rotation ${rotationKey}`;
  itemRotation.style.color = "white";
  itemRotation.style.paddingBottom = "15px";
  itemRotation.style.fontSize = "1em";
  itemRotation.style.fontWeight = "bold";
  box.appendChild(itemRotation);
  return box;
};

const addRotationItems = (box, item, inputText) => {
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
  return box;
};

const addRotations = (box, el, inputText) => {
  box = addRotation("A", box);
  for (const item of el.rotations.a) {
    box = addRotationItems(box, item, inputText);
  }
  box = addRotation("B", box);
  for (const item of el.rotations.b) {
    box = addRotationItems(box, item, inputText);
  }
  box = addRotation("C", box);
  for (const item of el.rotations.c) {
    box = addRotationItems(box, item, inputText);
  }
  return box;
};

const addClassic = (box, el, inputText) => {
  for (const item of el.items) {
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

const addRelic = (element, inputText) => {
  const fragment = document.createDocumentFragment();
  const name = element.name;
  const box = createBox(name);
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
  fragment.appendChild(box);
  document.getElementById("wrapper").appendChild(fragment);
};

const showData = (data, inputText) => {
  const wrapper = document.getElementById("wrapper");
  const missions = data.missions;
  const relics = data.relics;
  wrapper.textContent = "";
  for (let i = 0; i < missions.length; i++) {
    addMission(missions[i], inputText.toLowerCase());
  }
  for (let i = 0; i < relics.length; i++) {
    addRelic(relics[i], inputText.toLowerCase());
  }
};

const searchInput = document.getElementById("input_search");

const debounce = (func, delay) => {
  let debounceTimer;
  return function (...args) {
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => func.apply(this, args), delay);
  };
};

init().then(() => {
  getWarframeData().then((data) => {
    const result = new WarframeData(data);
    showData(result, "NOTHING");
    searchInput.addEventListener(
      "input",
      debounce((e) => {
        const inputText = e.target.value;
        if (inputText == "" || inputText == null) {
          showData(result, "NOTHING");
          return;
        }
        const filteredResult = result.filter(inputText.toLowerCase());
        showData(filteredResult, inputText);
      }, 150),
    );
  });
});
