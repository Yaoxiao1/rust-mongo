import React, { useState } from "react";
import { json, useLocation } from "react-router-dom";

function NewPaper(props) {
  // Component logic for displaying the question paper
  // ...
  const { state } = useLocation();

  const question_list = state.question;
  console.log("question list:\n" + JSON.stringify(question_list));
  const papertype = state.type;
  console.log("papertype:\n" + JSON.stringify(papertype));
  const date = state.date;
  console.log("date\n" + JSON.stringify(date));
  const prefix = "/home/yaoxiao/mydoc/rust-mongo";
  console.log(question_list.length);
  question_list.length === 0 && <div>No data to show!</div>;

  return (
    <div>
      {question_list.length === 0 && <div>No data to show!</div>}
      {question_list.length > 0 && (
        <div>
          <h2>{date}</h2>
          <ol>
            {question_list.map((question, index) => {
              if (papertype === "brief_answer")
                return (
                  <li style={{ display: "flex", alignItems: "flex-start" }}>
                    <span style={{ marginRight: "10px" }}>{index + 1}.</span>
                    {question.short_answer.map((sa) => (
                      <p>{sa}</p>
                    ))}
                  </li>
                );
              if (papertype === "question")
                return (
                  <li style={{ display: "flex", alignItems: "flex-start" }}>
                    <span style={{ marginRight: "10px" }}>{index + 1}.</span>
                    {question.q_url.map((q) => (
                      <img src={q.replace(prefix, ".")} alt="Wrong Answer" />
                    ))}
                  </li>
                );
              if (papertype === "full_answer")
                return (
                  <li style={{ display: "flex", alignItems: "flex-start" }}>
                    <span style={{ marginRight: "10px" }}>{index + 1}.</span>
                    {question.qa_url.map((qa) => (
                      <img src={qa.replace(prefix, ".")} alt="Wrong Answer" />
                    ))}
                  </li>
                );
              return null;
            })}
          </ol>
        </div>
      )}
    </div>
  );
}

export default NewPaper;
