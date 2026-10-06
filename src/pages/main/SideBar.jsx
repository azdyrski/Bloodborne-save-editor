import { useContext } from "react";
import { Link, useLocation } from "react-router-dom";
import { SaveContext } from "../../context/context";

function SideBar() {
  const { save } = useContext(SaveContext);
  const { pathname } = useLocation();

  return (
    <div id="sideBar">
      <ul>
        <li
          className={
            save && pathname.match(/^\/$/) ? "selected" : ""
          }
        >
          <Link to={"/"}>Inventory</Link>
        </li>
        <li
          className={
            save && pathname.match(/storage/)
              ? "selected"
              : ""
          }
        >
          <Link to={"/storage"}>Storage</Link>
        </li>
        <li
          className={
            save && pathname.match(/stats/) ? "selected" : ""
          }
        >
          <Link to={"/stats"}>Stats</Link>
        </li>
        <li
          className={
            save && pathname.match(/character/)
              ? "selected"
              : ""
          }
        >
          <Link to={"/character"}>Character</Link>
        </li>
        <li
          className={
            save && pathname.match(/bosses/) ? "selected" : ""
          }
        >
          <Link to={"/bosses"}>Bosses</Link>
        </li>
        <li
          className={
            save && pathname.match(/flags/) ? "selected" : ""
          }
        >
          <Link to={"/flags"}>Flags</Link>
        </li>
      </ul>
    </div>
  );
}

export default SideBar;
