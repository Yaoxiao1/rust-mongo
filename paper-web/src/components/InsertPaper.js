import React, { useState } from "react";

const InsertPaper = () => {
  const [paperPath, setPaperPath] = useState("");
  const [insertResult, setInsertResult] = useState("");
  const handleSubmit = async (e) => {
    e.preventDefault();
    console.log("paperPath: " + paperPath);
    try {
      const response = await fetch(
        "http://localhost:8080/api/insertQuestions",
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify(paperPath),
        }
      );
      if (response.ok) {
        setInsertResult("Insert Success!");
        console.log("Insert Success!");
      } else {
        setInsertResult("server error");
      }
    } catch (error) {
      setInsertResult("Insert Failed");
      console.error("Insert Failed");
    }
  };

  return (
    <>
      <input
        type="text"
        value={paperPath}
        onChange={(e) => setPaperPath(e.target.value)}
        placeholder="please input paper path"
      ></input>
      <button onClick={handleSubmit}>Submit</button>
      <p>{insertResult}</p>
    </>
  );
};

export default InsertPaper;
