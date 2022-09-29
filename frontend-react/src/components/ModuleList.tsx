import { RouterLink } from "../MiniDrawer";
import List from "@mui/material/List";
import {
  ModuleMenuType,
  ModulesResponseType,
  ModuleType,
} from "../validation-io-ts";

type ModuleListProps = { listData: ModulesResponseType };
export function ModuleList({ listData }: ModuleListProps) {
  return (
    <List>
      {listData.Submenu &&
        listData.Submenu.map((submenuEntry: ModuleMenuType) => (
          <RouterLink
            key={submenuEntry.tucan_id}
            to={`/modules/${submenuEntry.tucan_id}`}
            text={submenuEntry.name}
          ></RouterLink>
        ))}
      {listData.Modules &&
        listData.Modules.map((module: ModuleType) => (
          <RouterLink
            key={module.tucan_id}
            to={`/module/${module.tucan_id}`}
            text={module.title}
          ></RouterLink>
        ))}
    </List>
  );
}
