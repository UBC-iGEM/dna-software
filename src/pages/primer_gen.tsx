import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./../App.css";
import { FormInput } from "../components/FormStyle";

interface PrimerInfo {
  primer: string[];
  melting_temp: number;
  gc_content: number;
  has_hairpin: boolean;
}


function PrimerGeneration() {
 const lengths = [5, 15, 18, 20, 24];
  const meltingTempConstraints = ["Above", "Below"];

  const [length, setLength] = useState(lengths[0]);
  const [meltingTemp, setMeltingTemperaure] = useState(46);
  const [meltingTempConstraint, setMeltingTempConstraint] = useState(
    meltingTempConstraints[0]
  );
  const [primers, setPrimers] = useState([] as PrimerInfo[]);

  async function get_primer_gen_info() {
    const meltingTemperature = {
      temperature: meltingTemp,
      constraint: meltingTempConstraint,
    };

    const primers: PrimerInfo[] = await invoke("generate_primers", {
      len: length,
      meltingTemperature,
      lenG: 3,
    });
    setPrimers(primers);
  }

  return (
    <div>

      <h2>Primer Generation</h2>
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          get_primer_gen_info();
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

        <button type="submit">Submit</button>
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

export default PrimerGeneration;
