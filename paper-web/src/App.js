import { BrowserRouter, Routes, Route } from "react-router-dom";
import "./App.css";
import GeneratePaper from "./components/Generate";
import PaperTemplate from "./components/PaperTemplate";
import UpdateDb from "./components/UpdateDb";
import Layout from "./components/Layout";
import NewPaper from "./components/Newpaper";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<Layout />}>
          <Route index element={<GeneratePaper />} />
          <Route path="paper_template" element={<PaperTemplate />} />
          <Route path="update_db" element={<UpdateDb />} />
        </Route>
        <Route path="/newpaper" element={<NewPaper />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
