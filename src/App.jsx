import "./App.css";

import { useEffect, useState } from "react";
import Nav from "./components/Nav";
import { HashRouter as Router } from "react-router-dom";
import Main from "./pages/main/Main";
import { ImagesProvider } from "./context/imagesContext";

function App() {
  const [save, setSave] = useState(null);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    updateZoom();

    const scalingQuery = window.matchMedia(`(min-width: 2000px)`);

    function updateZoom() {
      if (window.innerWidth > 2000) {
        document.body.style.zoom = 2;
      } else {
        document.body.style.zoom = 1;
      }
    }

    scalingQuery.addEventListener("change", updateZoom);

    return () => {
      scalingQuery.removeEventListener("change", updateZoom);
    };
  }, []);

  return (
    <div className="App">
      <Router>
        <Nav setLoading={setLoading} save={save} setSave={setSave} />
        <ImagesProvider>
          <Main save={save} setSave={setSave} loading={loading} />
        </ImagesProvider>
      </Router>
    </div>
  );
}

export default App;
