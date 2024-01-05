import React, { useEffect, useState } from "react";
import { json, useLocation } from "react-router-dom";

function NewPaper(props) {
  // Component logic for displaying the question paper
  // ...
  const { state } = useLocation();
  const [idx, setIdx] = useState(0);
  const question_list = state.question;
  console.log(question_list);
  const date = state.date;
  const prefix = "/home/yaoxiao/mydoc/rust-mongo";
  const type_list = ["question", "brief_answer", "full_answer"];
  const papertype = type_list[idx];
  // Event handler for the keydown event
  const handleKeyDown = (event) => {
    setIdx((idx + 1) % 3);
  };
  console.log(papertype, idx);

  return (
    <div>
      {question_list.length === 0 && <div>No data to show!</div>}
      {question_list.length > 0 && (
        <div>
          <h2 onClick={handleKeyDown}>{date}</h2>
          <ol>
            {question_list.map((question, index) => {
              if (papertype === "brief_answer")
                return question.short_answer ? (
                  <li style={{ display: "flex", alignItems: "flex-start" }}>
                    <span style={{ marginRight: "10px" }}>{index + 1}.</span>
                    {question.short_answer.map(
                      (sa) => sa.length > 0 && <p>{sa}</p>
                    )}
                  </li>
                ) : null;
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
