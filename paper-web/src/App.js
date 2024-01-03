import { BrowserRouter, Routes, Route } from "react-router-dom";
import "./App.css";
import GeneratePaper from "./components/Generate";
import UpdateDb from "./components/UpdateDb";
import Layout from "./components/Layout";
import NewPaper from "./components/Newpaper";
import UpdateUser from "./components/UpdateUser";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route index element={<UpdateUser />} />
          <Route path="update_user" element={<UpdateUser />} />
          <Route path="generate" element={<GeneratePaper />} />
        </Route>
        <Route path="/newpaper" element={<NewPaper />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
