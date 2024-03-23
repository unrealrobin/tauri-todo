import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import Item from "./components/item";
import AddItem from "./components/add-item";


function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div className="container">
      <h1>Add todo items below.</h1>
      <Item />
      <AddItem />
      
    </div>
  );
}

export default App;
