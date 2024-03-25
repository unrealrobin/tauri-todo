import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import Item from "./components/item";
import AddItem from "./components/add-item";
import { v4 as uuidv4 } from 'uuid';


function App() {
  
  const [listData, setListData] = useState({});



  useEffect(() => {
    readDb();
  },[]);


  //Invoking my read_db function from rust backend to retrieve data.
  async function readDb () {
    let data = await invoke("read_db");
    setListData(data);
  }

  return (
    <div className="container">
      <h1>Add todo items below.</h1>
      {
      Object.keys(listData).map((listkey) => (
        <Item 
          refreshTodos={readDb}
          key={uuidv4()} 
          id={listkey}
          text={listData[listkey].text} 
          status={listData[listkey].is_complete} 
        />
      ))

      
      }   
      <AddItem refreshTodos={readDb} />
        
    </div>
  );
}

export default App;
