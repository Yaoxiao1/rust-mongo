import React, { useState } from "react";
import { Link } from "react-router-dom";

// 根据输入的用户名和试卷名，查询数据库调出对应的错题列表和作业列表，导出到试卷界面
function GeneratePaper(props) {
  const [userName, setUserName] = useState("");
  const [paperName, setPaperName] = useState("");
  const [title, setTitle] = useState("");
  const [questionList, setQuestionList] = useState("");
  const handleSubmit = async (e) => {
    e.preventDefault();

    const data = {
      user_name: userName,
      paper_name: paperName,
    };
    try {
      // Send a POST request to the backend
      const response = await fetch("http://localhost:8080/api/searchUser", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(data),
      });

      if (response.ok) {
        // Parse the response and extract the list of picture URLs
        const question = await response.json();
        setQuestionList(question);
      } else {
        const errorMessage = await response.text();
        console.error("Failed to submit the form:", errorMessage);
      }
    } catch (error) {
      console.error("Error:", error);
    }
  };

  return (
    <>
      <div>
        {questionList.length > 0 && (
          <>
            <Link
              to={{
                pathname: "/newpaper",
              }}
              state={{
                question: questionList,
                title: title,
              }}
            >
              Question Paper
            </Link>
            <br />

            {/* todo, need a buttion to insert a document to Mongo to keep track of the new paper*/}
          </>
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
              Paper Name:
              <input
                type="text"
                value={paperName}
                placeholder="seperate by comma(,)"
                onChange={(e) => setPaperName(e.target.value)}
              />
            </label>
            <br />
            <label>
              Date:
              <input
                type="text"
                value={title}
                placeholder="输入试卷标题"
                onChange={(e) => setTitle(e.target.value)}
              />
            </label>
            <br />
            <button type="submit">Submit</button>
          </form>
        )}
        {questionList.length > 0 && (
          <div>
            {<p>{questionList.map((question) => question.id).join(", ")}</p>}
          </div>
        )}
      </div>
    </>
  );
}

export default GeneratePaper;
