// pub struct User {
// pub id: u64,
// pub name: String,
// pub paper_name: String,
// pub date: Mydate,
// pub wrong_question_list: Vec<u64>,
// pub homework_question_list: Vec<u64>,
// 插入某个学生的错题列表和作业列表
import React, { useState } from "react";

const UpdateUser = () => {
  const [userName, setUserName] = useState("");
  const [paperName, setPaperName] = useState("");
  const [wrongAnswerList, setWrongAnswerList] = useState("");
  const [homeworkList, setHomeworkList] = useState("");
  const [submit, setSubmit] = useState(false);
  const [data, setData] = useState({});

  const convertToNumberArray = (inputString) => {
    return inputString.split(",").map((item) => Number(item.trim()));
  };

  const handleSubmit = async (e) => {
    e.preventDefault();

    const data = {
      user_name: userName,
      paper_name: paperName,
      wrong_answer_list: convertToNumberArray(wrongAnswerList),
      homework_list: convertToNumberArray(homeworkList),
    };
    setData(data);
    console.log(data);
    try {
      // Send a POST request to the backend
      const response = await fetch("http://localhost:8080/api/updateUser", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(data),
      });

      if (response.ok) {
        // Parse the response and extract the list of picture URLs
        setSubmit(true);
        console.log("update user successfully!");
      } else {
        console.error("Failed to submit the form");
      }
    } catch (error) {
      console.error("Error:", error);
    }
  };

  return (
    <div>
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
            onChange={(e) => setPaperName(e.target.value)}
          />
        </label>
        <br />
        <label>
          wrong_answer_list:
          <input
            type="text"
            value={wrongAnswerList}
            onChange={(e) => setWrongAnswerList(e.target.value)}
          />
        </label>
        <br />
        <label>
          Homework List:
          <input
            type="text"
            value={homeworkList}
            onChange={(e) => setHomeworkList(e.target.value)}
          />
        </label>
        <br />
        <input type="submit" value="Submit"></input>
      </form>
      {submit ? (
        <>
          <p>{JSON.stringify(data)}</p>
          <br />
          <p>Submit success</p>
        </>
      ) : null}
    </div>
  );
};

export default UpdateUser;
