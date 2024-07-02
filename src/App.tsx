import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import "./App.css";
import Encode from "./pages/encode";
import Home from "./pages";
import NavBar from "./components/NavBar";
import Decode from "./pages/decode";
import PrimerGeneration from "./pages/primer_gen";

function App() {
  return (
    <Router>
      <NavBar />
      <Routes>
        <Route path="/" element={<Home />}></Route>
        <Route path="/primer" element={<PrimerGeneration />}></Route>
        <Route path="/encode" element={<Encode />}></Route>
        <Route path="/decode" element={<Decode />}></Route>
      </Routes>
    </Router>
  );
}

export default App;
