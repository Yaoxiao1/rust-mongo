import React, { useState } from "react";


function GeneratePaper(props) {
    const [userName, setUserName] = useState('');
    const [paperList, setPaperList] = useState('');
    const [date, setDate] = useState('');
    const [questionUrlList, setQuestionUrlList] = useState('');
    const [qaUrlList, setQaUrlList] = useState('')
    const [shortAnswerList, setShortAnswerList] = useState('');
    const handleSubmit = async (e) => {
        e.preventDefault();
    }
    return (
        <>
        <div>
            <form onSubmit={handleSubmit}>
                <label>
                    User Name:
                    <input type="text" value={userName}
                    onChange={(e) => setUserName(e.target.value)}/>
                </label>
                <br/>
                <label>
                    Paper List:
                    <input type="text" value={userName} placeholder="seperate by comma(,)"
                    onChange={(e) => setPaperList(e.target.value)}/>
                </label>
                <br/>
                <label>
                    Date:
                    <input type="text" value={userName} placeholder="YYYYmmDD, eg.(20231216)"
                    onChange={(e) => setDate(e.target.value)}/>
                </label>
                <br/>
            </form>
        </div>
        </>
    )
}


export default GeneratePaper;