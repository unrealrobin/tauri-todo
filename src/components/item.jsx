import { useState } from "react";
import "./styles/Item.css"
import { invoke } from "@tauri-apps/api/tauri";

function Item({text, status, key, id, refreshTodos}) {

    const [todoStatus, setStatus] = useState(status); //Initialized to the DB Value
    const [todoText, setText] = useState(text);//Initialized to the DB Value

    const toggleStatus = () => {
        setStatus(!todoStatus);
        
        //TODO: Call Update Function in Rust Backend to Update DB.
    }

    const handleTextChange = (event) => {
        setText(event.target.value);

        //TODO: Call Update Function in Rust Backend to Update DB.
    }

    const handleDeleteItem = async () => {
        await invoke("delete_item", {key: parseInt(id)}).then(refreshTodos());        
    }
    

    return(
        <div className="item-container" key={key} >
            <input className="text-input" type="text"  placeholder="What do you need to do?" value={todoText} onChange={handleTextChange} />
            <div className="button-container">
                <button className="status-button" type="checkbox" onClick={toggleStatus}>{status ? 'Complete' : 'Incomplete'}</button>
                <button  className="delete-button" onClick={handleDeleteItem}>X</button>
            </div>
        </div>
    )
}

export default Item;