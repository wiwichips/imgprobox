import { useState } from "react";

function ListGroup(props, headers) {
    const [selectedIndex, setSelectedIndex] = useState(-1);
    const [name, setName] = useState('');
    if (props.items.length === 0)
        return <></>

    console.log(props);

    return (
        <>
            <h1>{props.headers}</h1>
            <u1 className="list-group">
                {
                    props.items.map((item, index) => <li
                            className={ selectedIndex === index ? "list-group-item active" : "list-group-item"}
                            key={item}
                            onClick={() => { setSelectedIndex(index); props.onSelectionItem(item)}}
                        >
                        {item}
                    </li>)
                }
            </u1>
        </>);

}

export default ListGroup;
