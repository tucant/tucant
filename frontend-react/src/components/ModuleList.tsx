import { ModuleMenuResponse } from "../api";
import { Link } from "../Navigation";

type ModuleListProps = { listData: ModuleMenuResponse };
export function ModuleList({ listData }: ModuleListProps) {
  return (
    <>
      {listData.entries.type === "Submenu" && (
        <div className="list-group">
          {listData.entries.value.map((e) => (
            <Link
              key={e.tucan_id}
              className="list-group-item list-group-item-action"
              to={`/modules/${e.tucan_id}`}
            >
              {e.name}
            </Link>
          ))}
        </div>
      )}
      {listData.entries.type === "Modules" && (
        <div className="list-group">
          {listData.entries.value.map((e) => (
            <Link
              key={e.tucan_id}
              className="list-group-item list-group-item-action"
              to={`/module/${e.tucan_id}`}
            >
              {e.title}
            </Link>
          ))}
        </div>
      )}
    </>
  );
}
