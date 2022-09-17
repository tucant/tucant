import { useLocation } from "react-router-dom";
import Module from "./Module";
import Modules from "./Modules";



export default function ModuleOrModules() {
  const location = useLocation();

  if (location.pathname.endsWith("/")) {
    return <Modules></Modules>
  } else {
    return <Module></Module>
  }
}