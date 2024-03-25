import { useEffect, useState, useCallback } from "react";
import "./styles/Item.css"
import { invoke } from "@tauri-apps/api/tauri";

function Item({text, status, key, id, refreshTodos}) {

    const [todoStatus, setStatus] = useState(status); //Initialized to the DB Value
    const [todoText, setText] = useState(text);//Initialized to the DB Value

    const toggleStatus = async() => {
        setStatus(!todoStatus);
        
        
    }

    const buttonBackgroundColor = useCallback(() => {
        return todoStatus ? "green" : "red";
    }, [todoStatus]);

    const handleTextChange = (event) => {
        setText(event.target.value);
    
    }

    const handleUpdateBackEnd = () => {
        //TODO: Call Update Function in Rust Backend to Update DB.
        invoke("update_item", {
            key: parseInt(id),
            text: todoText,
            newStatus: todoStatus,
        })
        
    }

    useEffect(() => {
        handleUpdateBackEnd();
    }, [todoStatus, todoText])

    const handleDeleteItem = async () => {
        await invoke("delete_item", {key: parseInt(id)}).then(refreshTodos());        
    }
    

    return(
        <div className="item-container" >
            <input className="text-input" type="text"  placeholder="What do you need to do?" value={todoText} onChange={handleTextChange}/>
            <div className="button-container">
                <button className="status-button" type="checkbox" onClick={toggleStatus} style={{backgroundColor: todoStatus ? "#2bff32" : "#ff3d2b", color: "#2d2e2d"}}  >{todoStatus ? 'Complete' : 'Incomplete'}</button>
                <button  className="delete-button" onClick={handleDeleteItem}>X</button>
            </div>
        </div>
    )
}

export default Item