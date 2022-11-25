import { ModuleMenuResponse } from "../api";
import { Link } from "../Navigation";

type ModuleListProps = { listData: ModuleMenuResponse };
export function ModuleList({ listData }: ModuleListProps) {
  return (
    <>
      <div className="list-group">
        {listData.entries.submenus.map((e) => (
          <Link
            key={e.tucan_id}
            className="list-group-item list-group-item-action"
            to={`/modules/${e.tucan_id}`}
          >
            {e.name}
          </Link>
        ))}
      </div>
      <div className="list-group">
        {listData.entries.modules_and_courses.map((e) => {
          if (e[0]) {
            return (
              <Link
                key={e[0].tucan_id}
                className="list-group-item list-group-item-action"
                to={`/module/${String(e[0].tucan_id)}`}
              >
                {e[0].title}
              </Link>
            );
          } else {
            return (
              <a className="list-group-item list-group-item-action">Guten Tag. TUCaN ist lost. Hier gibt es kein Modul.</a>
            );
          }
        })}
      </div>
    </>
  );
}
