import React, { useRef, useState, useEffect } from "react";

function UpdateDb(props) {
    const [data, setData] = useState(null);
    const [refresh, setRefresh] = useState(false);
    const [isLoaded, setIsLoaded] = useState(false);

    useEffect(() => {
        const fetch_data = async () => {
            let url = "http://localhost:8080/show-dbs";
            const response = await fetch(url);
            const data = await response.json();
            setData(data)
            setIsLoaded(true);
            console.log("get data " + data);
        }
    
        fetch_data();
    }, [refresh])
    

    function refreshPage() {
        // we force the component to re-render by
        // changing the state which triggers the
        // fetchData effect, which updates the items data
        // and that then triggers the putImage effect
        setIsLoaded(false);
        setRefresh(!refresh);
    }


    
    return (
        <div>
            <button onClick={refreshPage}>press to UpdateDb</button>
            <p>{data? JSON.stringify(data) : "{}"}</p>
        </div>
    )
}

export default UpdateDb;