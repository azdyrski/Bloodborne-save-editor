import { getSaveBytes, loadSave } from "../utils/backend";
import { useState } from "react";
import * as dialog from "../utils/dialog";

function Nav({ setLoading, setSave, save }) {
  const [name, setName] = useState("");

  async function readFile() {
    try {
      const file = await dialog.openFile();
      if (!file) return;
      if (save) setSave(null);
      setLoading(true);

      const parsedSave = await loadSave(file);
      setLoading(false);
      setSave(parsedSave);
      setName(file.name);
    } catch (error) {
      console.error(error);
      await dialog.message(
        "Could not parse file, make sure it is a decrypted save file",
      );
      setLoading(false);
    }
  }

  async function saveChanges() {
    try {
      dialog.downloadFile(await getSaveBytes(), name || "save");
    } catch (error) {
      console.log(error);
    }
  }

  return (
    <nav className="nav">
      <button id="openSave" onClick={readFile}>
        Open
      </button>
      <span>{name}</span>
      <button disabled={save == null ? true : false} onClick={saveChanges}>
        Save
      </button>
    </nav>
  );
}

export default Nav;
