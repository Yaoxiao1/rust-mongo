import { BrowserRouter, Routes, Route } from "react-router-dom";
import "./App.css";
import GeneratePaper from "./components/Generate";
import UpdateDb from "./components/UpdateDb";
import Layout from "./components/Layout";
import NewPaper from "./components/Newpaper";
import UpdateUser from "./components/UpdateUser";
import InsertPaper from "./components/InsertPaper";
import InsertUserInfo from "./components/InsertUser";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route index element={<UpdateUser />} />
          <Route path="update_user" element={<UpdateUser />} />
          <Route path="generate" element={<GeneratePaper />} />
          <Route path="insertPaper" element={<InsertPaper />} />
          <Route path="insertUserInfo" element={<InsertUserInfo />} />
        </Route>
        <Route path="/newpaper" element={<NewPaper />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
