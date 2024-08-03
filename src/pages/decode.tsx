import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { FormInput } from "../components/FormStyle";
import { appDir } from "@tauri-apps/api/path";
import { open } from "@tauri-apps/api/dialog";
import { WrapText } from "../components/Text";

function Decode() {
  const [decodedFileContents, setDecodedFileContents] = useState("" as string);
  async function decode_file() {
    // Open a selection dialog for directories

    console.log("here1");
    const filePath = await open({
      filters: [{ name: "Text", extensions: ["fasta", "fastq"] }],
      defaultPath: await appDir(),
    });
    console.log("here2");
   const decoded_file_contents: string= await invoke("decode_sequence", {
      filePath: filePath,
    });
    setDecodedFileContents(decoded_file_contents);
  }

  return (
    <>
      <h2>Decode</h2>
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          decode_file();
        }}
      >
        <FormInput className="col">
        </FormInput>
        <button type="submit">Choose File To Decode</button>
      </form>
    <WrapText>{decodedFileContents}</WrapText>
    </>
  );
}
export default Decode;
