import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

interface PrimerInfo {
  primer: string[];
  melting_temp: number;
  gc_content: number;
  has_hairpin: boolean;
}

function App() {
  const lengths = [5, 15, 18, 20, 24];
  const meltingTempConstraints = ["Above", "Below"];

  const [length, setLength] = useState(lengths[0]);
  const [meltingTemp, setMeltingTemperaure] = useState(46);
  const [meltingTempConstraint, setMeltingTempConstraint] = useState(
    meltingTempConstraints[0]
  );
  const [primers, setPrimers] = useState([] as PrimerInfo[]);

  async function greet() {
    const meltingTemperature = {
      temperature: meltingTemp,
      constraint: meltingTempConstraint,
    };

    const primers: PrimerInfo[] = await invoke("generate_primers", {
      len: length,
      meltingTemperature,
      lenG: 3,
    });
    console.log(primers);
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
      <table>
        <tr>
          <th>Primer</th>
          <th>Melting Temperature</th>
          <th>GC Content</th>
          <th>Hairpin</th>
        </tr>
        {primers.map((p) => {
          return (
            <tr>
              <td>{p.primer.join("")}</td>
              <td>{p.melting_temp}</td>
              <td>{p.gc_content}</td>
              <td>{p.has_hairpin.toString()}</td>
            </tr>
          );
        })}
      </table>
    </div>
  );
}

export default App;
