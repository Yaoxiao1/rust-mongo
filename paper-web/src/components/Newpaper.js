import React from "react";
import { useLocation } from "react-router-dom";

function NewPaper(props) {
  // Component logic for displaying the question paper
  // ...
  let { state } = useLocation();
  return <div>show stata : {JSON.stringify(state)}</div>;
}

export default NewPaper;
