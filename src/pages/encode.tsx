import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { FormInput } from "../components/FormStyle";
import { appDir } from "@tauri-apps/api/path";
import { open } from "@tauri-apps/api/dialog";

function Encode() {
  const encoderTypes = ["rotation", "quaternary"];
  const [encodedSequence, setEncodedSequence] = useState([] as string[]);
  const [encoderType, setEncoderType] = useState(encoderTypes[0]);

  async function encode_file() {
    // Open a selection dialog for directories
    const filePath = await open({
      filters: [{ name: "Text", extensions: ["txt"] }],
      defaultPath: await appDir(),
    });
    if (filePath === null) {
      return;
    }
    const encoded_sequence: string[] = await invoke("encode_sequence", {
      encoderType: encoderType,
      filePath: filePath,
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
          encode_file();
        }}
      >
        <FormInput className="col">
          <label htmlFor="encoderType">Encoder Type</label>
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
        <button type="submit">Choose File To Encode</button>
      </form>
      <p>{encodedSequence.map((b) => b)}</p>
    </>
  );
}
export default Encode;
