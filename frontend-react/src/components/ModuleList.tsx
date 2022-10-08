import { ModuleMenuResponse } from "../api";
import { Link } from "../Navigation";

type ModuleListProps = { listData: ModuleMenuResponse };
export function ModuleList({ listData }: ModuleListProps) {
  return (
    <>
      {listData.entries.type === "Submenu" &&
        listData.entries.value.map((e) => (
          <Link key={e.tucan_id} to={`/modules/${e.tucan_id}`}>
            {e.name}
          </Link>
        ))}
      {listData.entries.type === "Modules" &&
        listData.entries.value.map((e) => (
          <Link key={e.tucan_id} to={`/module/${e.tucan_id}`}>
            {e.title}
          </Link>
        ))}
    </>
  );
}
