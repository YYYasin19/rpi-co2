import "./App.css";
import { Chart } from "./chart";
import { useState } from "react";

function App() {
  const [serverAdress, setServerAdress] = useState("http://localhost:3000");
  return (
    <>
      <h1>CO2 Data</h1>
      <input
        style={{ width: "100%", padding: "10px" }}
        type="text"
        defaultValue={serverAdress}
        onChange={(e) => setServerAdress(e.target.value)}
      />
      <Chart serverAdress={serverAdress} />
    </>
  );
}

export default App;
