import { useContext } from "react";
import { useLocation, Routes, Route } from "react-router-dom";
import SideBar from "./SideBar";
import Inventory from "../inventory/Inventory";
import Stats from "../stats/Stats";
import Character from "../character/Character";
import { SaveContext } from "../../context/context";
import { ItemsProvider } from "../../context/itemsContext";
import { ImagesContext } from "../../context/imagesContext";
import EquippedGems from "./EquippedGems";
import Bosses from "../bosses/Bosses";
import Flags from "../flags/Flags";
import { loadSave } from "../../utils/backend";
import * as dialog from "../../utils/dialog";

const Main = ({ save, setSave, loading, setLoading, setName }) => {
  const location = useLocation();
  const { loading: loadingImages } = useContext(ImagesContext);

  async function createSave() {
    try {
      setLoading(true);
      const res = await fetch("default-save/userdata0001");
      if (!res.ok) throw new Error(`HTTP ${res.status}`);
      const parsedSave = await loadSave(await res.blob());
      setSave(parsedSave);
      setName("userdata0001");
    } catch (error) {
      console.error(error);
      await dialog.message("Could not load the default save");
    } finally {
      setLoading(false);
    }
  }

  return (
    <>
      <div
        style={{
          transition: "opacity",
          background: "#1b1b26",
          position: "absolute",
          display: "flex",
          flexDirection: "column",
          justifyContent: "center",
          alignItems: "center",
          height: "100%",
          width: "100%",
          top: 0,
          zIndex: 99999,
        }}
        className={loadingImages ? "" : "fade-out"}
      >
        <img src="./assets/icon.png" width="200px" alt="" />
        <div className="spinner"></div>
      </div>
      <main
        style={{
          gridTemplateColumns: `200px 805px ${
            location.pathname.match(/storage|\/$/) != null ? "1fr" : ""
          }`,
        }}
      >
        <SaveContext.Provider value={{ save, setSave }}>
          <SideBar />
          {loading ? <div>Loading</div> : null}

          {save != null ? (
            <Routes>
              <Route
                path="/"
                element={
                  <ItemsProvider>
                    <Inventory
                      key={"inventory"}
                      inv={save.inventory}
                      isStorage={false}
                    />
                  </ItemsProvider>
                }
              />
              <Route
                path="/storage"
                element={
                  <ItemsProvider>
                    <Inventory
                      key={"storage"}
                      inv={save.storage}
                      isStorage={true}
                    />
                  </ItemsProvider>
                }
              />
              <Route path="/stats" element={<Stats />} />
              <Route path="/character" element={<Character />} />
              <Route
                path="/equippedGems"
                element={
                  <ItemsProvider>
                    <EquippedGems />
                  </ItemsProvider>
                }
              ></Route>
              <Route path="/bosses" element={<Bosses />}></Route>
              <Route path="/flags" element={<Flags />}></Route>
            </Routes>
          ) : null}

          {save == null && !loading ? (
            <div
              style={{
                gridColumn: "2/4",
                justifySelf: "center",
                padding: "2.5rem",
              }}
            >
              This save editor works with decrypted save files, meaning that you
              can't use the savefile you get from exporting it directly from
              your playstation. If you don't know how to decrypt a save click{" "}
              <a
                style={{ textDecoration: "underline" }}
                href="https://github.com/Noxde/Bloodborne-save-editor/wiki/"
                target="_blank"
                rel="noreferrer"
              >
                here
              </a>{" "}
              to learn more.
              <p>
                Don't have a save? Click{" "}
                <a
                  style={{ textDecoration: "underline", cursor: "pointer" }}
                  onClick={createSave}
                >
                  Create
                </a>{" "}
                to start from a new default save. (Level 4 character, just woken up)
              </p>
            </div>
          ) : null}
        </SaveContext.Provider>
      </main>
    </>
  );
};

export default Main;
