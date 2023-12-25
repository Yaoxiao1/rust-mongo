import './App.css';
import GeneratePaper from './components/generate_paper';
import PaperTemplate from './components/paper_template';
import UpdateDb from './components/update_db';

function App() {
  return (
    <div className="App">
      {/* <GeneratePaper></GeneratePaper> */}
      <PaperTemplate></PaperTemplate>  
      <UpdateDb></UpdateDb>
    </div>
    
  );
}

export default App;
