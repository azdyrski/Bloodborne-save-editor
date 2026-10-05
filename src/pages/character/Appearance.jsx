import { exportAppearance, importAppearance } from "../../utils/backend";
import * as dialog from "../../utils/dialog";

const buttonStyle = {
  padding: "0 15px",
  fontSize: "inherit",
  backgroundSize: "100% 100%",
};

function Appearance() {
  return (
    <div
      style={{
        fontSize: "25px",
        marginTop: "5px",
        display: "flex",
        justifyContent: "space-between",
      }}
    >
      <button
        className="buttonBg"
        style={buttonStyle}
        onClick={async () => {
          try {
            dialog.downloadFile(await exportAppearance(), "face");
          } catch (error) {
            console.error(error);
            await dialog.message("There was an error exporting the face");
          }
        }}
      >
        Export face
      </button>
      <button
        className="buttonBg"
        style={buttonStyle}
        onClick={async () => {
          try {
            const file = await dialog.openFile();

            if (file) {
              await dialog.message(await importAppearance(file));
            }
          } catch (error) {
            console.error(error);
            await dialog.message(error);
          }
        }}
      >
        Import face
      </button>
    </div>
  );
}

export default Appearance;
