import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const lengths = [5, 15, 18, 20, 24];
  const meltingTempConstraints = ["Above", "Below"];

  const [length, setLength] = useState(lengths[0]);
  const [meltingTemp, setMeltingTemperaure] = useState(46);
  const [meltingTempConstraint, setMeltingTempConstraint] = useState(
    meltingTempConstraints[0],
  );
  const [primers, setPrimers] = useState([] as string[][]);

  async function greet() {
    const meltingTemperature = {
      temperature: meltingTemp,
      constraint: meltingTempConstraint,
    };

    const primers: string[][] = await invoke("generate_primers", {
      len: length,
      meltingTemperature,
      lenG: 4,
    });

    setPrimers(primers);
  }

  return (
    <div className="container">
      <h1>DNADrive</h1>

      <p>Generate some primers.</p>

      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <input
          id="meltingTemperature"
          onChange={(e) =>
            setMeltingTemperaure(parseFloat(e.currentTarget.value))
          }
          placeholder="Enter a melting temperature..."
          value={meltingTemp}
        />

        <label htmlFor="lengths">Lengths</label>
        <select
          id="lengths"
          name="lengths"
          onChange={(e) => setLength(parseInt(e.currentTarget.value))}
        >
          {lengths.map((l) => (
            <option value={l}>{l}</option>
          ))}
        </select>

        <label htmlFor="meltingTemperatureConstraint">
          Above or below melting temperature
        </label>
        <select
          id="meltingTemperatureConstraint"
          name="meltingTemperatureConstraint"
          onChange={(e) => setMeltingTempConstraint(e.currentTarget.value)}
        >
          {meltingTempConstraints.map((l) => (
            <option value={l}>{l}</option>
          ))}
        </select>

        <button type="submit">Greet</button>
      </form>
      {primers.map((p) => (
        <p>{p.join("")}</p>
      ))}
    </div>
  );
}

export default App;
