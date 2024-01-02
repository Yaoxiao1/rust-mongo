import React, { useState } from "react";
import { Link } from "react-router-dom";

function GeneratePaper(props) {
  const [userName, setUserName] = useState("");
  const [paperList, setPaperList] = useState("");
  const [date, setDate] = useState("");
  const [questionList, setQuestionList] = useState("");
  const handleSubmit = async (e) => {
    e.preventDefault();
  };
  const openInNewTab = (url) => {
    window.open(url, "_blank", "noreferrer");
  };
  return (
    <>
      <div>
        {questionList.length >= 0 && (
          <Link
            to={{
              pathname: "newpaper",
            }}
            state={{ a: "1", b: "2" }}
          >
            go to newpaper
          </Link>
        )}
        {questionList.length === 0 && (
          <form onSubmit={handleSubmit}>
            <label>
              User Name:
              <input
                type="text"
                value={userName}
                onChange={(e) => setUserName(e.target.value)}
              />
            </label>
            <br />
            <label>
              Paper List:
              <input
                type="text"
                value={userName}
                placeholder="seperate by comma(,)"
                onChange={(e) => setPaperList(e.target.value)}
              />
            </label>
            <br />
            <label>
              Date:
              <input
                type="text"
                value={userName}
                placeholder="YYYYmmDD, eg.(20231216)"
                onChange={(e) => setDate(e.target.value)}
              />
            </label>
            <br />
          </form>
        )}
      </div>
    </>
  );
}

export default GeneratePaper;
