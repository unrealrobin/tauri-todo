import "./styles/AddItem.css"
import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

function AddItem({refreshTodos}) {
    const [adding, setAdding] = useState(false);
    const [todotext, setTodoText] = useState()

    const handleAdd = () => {
        setAdding(!adding);
    }

    const handleChange = (e) => {
        setTodoText(e.target.value);
    }

    const handleSubmit = async (e) => {
        e.preventDefault();

        if(todotext == ""){
            handleAdd();
            setTodoText("");
            refreshTodos();

        } else {
            await invoke("create_item", {text: todotext});
            refreshTodos();
            handleAdd();
            setTodoText("");
        }
        
    }

    const resetTodo = () => {
        handleAdd();
        setTodoText("");
    }

    return adding ?  (
        <div className="input-container">
            <input  type="text" value={todotext} placeholder="What do you need to do?" onChange={handleChange}  />
            <button onClick={handleSubmit} > Add </button>
            <button onClick={resetTodo}  > X </button>
        </div>
        
    ) 
    : 
    (
        <button className="add-container" onClick={() => handleAdd()}>
            +
        </button>
    )
    
}

export default AddItem;