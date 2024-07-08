import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { FormInput } from "../components/FormStyle";

function Encode() {
  const encoderTypes = ["rotation", "quaternary"];
  const [encodedSequence, setEncodedSequence] = useState([] as string[]);
  const [encoderType, setEncoderType] = useState(encoderTypes[0]);
  const [filePath, setFilePath] = useState("");

  async function encode_sequence() {
    // TODO: Fix the hack
    const root = "/home/lucy/iGEM23_24/dnadrive/";
    const file = filePath.split("\\")[2];
    const encoded_sequence: string[] = await invoke("encode_sequence", {
      encoderType: encoderType,
      filePath: root + file,
    });
    setEncodedSequence(encoded_sequence);
  }
  return (
    <>
      <h2>Encode</h2>
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          encode_sequence();
        }}
      >
        <FormInput className="col">
          <label htmlFor="lengths">File Path</label>
          <input
            type="file"
            id="filePath"
            name="filePath"
            onChange={(e) => setFilePath(e.currentTarget.value)}
            value={filePath}
            accept=".doc, .txt"
          />
        </FormInput>

        <FormInput className="col">
          <label htmlFor="lengths">Encoder Type</label>
          <select
            id="encoder"
            name="encoder"
            onChange={(e) => setEncoderType(e.currentTarget.value)}
          >
            {encoderTypes.map((e) => (
              <option value={e}>{e}</option>
            ))}
          </select>
        </FormInput>

        <button type="submit">Encode File</button>
      </form>
      <p>{encodedSequence.map((b) => b)}</p>
    </>
  );
}
export default Encode;
