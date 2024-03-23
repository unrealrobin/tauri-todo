import "./styles/Item.css"

function Item() {


    return(
        <div className="item-container" >
            <input className="text-input" type="text"  placeholder="What do you need to do? "></input>
            <div className="button-container">
                <button className="status-button" type="checkbox" ></button>
                <button  className="delete-button"></button>
            </div>
        </div>
    )
}

export default Item;