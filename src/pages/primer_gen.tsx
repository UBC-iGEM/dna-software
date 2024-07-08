import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./../App.css";
import { FormInput } from "../components/FormStyle";

function PrimerGeneration() {
  const lengths = [5, 15, 18, 20, 24];
  const meltingTempConstraints = ["Above", "Below"];

  const [length, setLength] = useState(lengths[0]);
  const [meltingTemp, setMeltingTemperaure] = useState(46);
  const [meltingTempConstraint, setMeltingTempConstraint] = useState(
    meltingTempConstraints[0],
  );
  const [primers, setPrimers] = useState([] as string[][]);

  async function generate_primers() {
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
    <div>
      <h1>Primer Generation</h1>
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          generate_primers();
        }}
      >
        <FormInput className="col">
          <label htmlFor="lengths">Temperature</label>
          <input
            id="meltingTemperature"
            onChange={(e) =>
              setMeltingTemperaure(parseFloat(e.currentTarget.value))
            }
            placeholder="Enter a melting temperature..."
            value={meltingTemp}
          />
        </FormInput>

        <FormInput className="col">
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
        </FormInput>

        <FormInput className="col">
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
        </FormInput>

        <button type="submit">Generate</button>
      </form>
      {primers.map((p) => (
        <p>{p.join("")}</p>
      ))}
    </div>
  );
}

export default PrimerGeneration;
