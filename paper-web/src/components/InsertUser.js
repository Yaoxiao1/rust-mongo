import React, { useState } from "react";

const InsertUserInfo = () => {
  const [userName, setUserName] = useState("");
  const [insertResult, setInsertResult] = useState("");
  const handleSubmit = async (e) => {
    e.preventDefault();
    console.log("username: " + userName);
    try {
      const response = await fetch("http://localhost:8080/api/insertUserInfo", {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(userName),
      });
      if (response.ok) {
        setInsertResult("Insert Success!");
        console.log("Insert Success!");
      } else {
        await response
          .text()
          .then((body) =>
            setInsertResult("response not ok, server error " + body)
          );
      }
    } catch (error) {
      setInsertResult("Insert Failed, " + error);
      console.error("Insert Failed");
    }
  };

  return (
    <>
      <input
        type="text"
        value={userName}
        onChange={(e) => setUserName(e.target.value)}
        placeholder="please input username"
      ></input>
      <button onClick={handleSubmit}>Submit</button>
      <p>{insertResult}</p>
    </>
  );
};

export default InsertUserInfo;
