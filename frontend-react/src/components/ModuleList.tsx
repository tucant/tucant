import { Fragment } from "react";
import { ModuleMenuResponse } from "../api";
import { Link } from "../Navigation";
import { ModuleEntry } from "./ModuleEntry";
import { CourseEntry } from "./CourseEntry";

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
        {listData.entries.modules_and_courses.map((e) => (
          <Fragment key={e[0].tucan_id}>
            <ModuleEntry module={e[0]} />
            {e[1].map((c) => (
              <CourseEntry key={c.tucan_id} c={c} />
            ))}
          </Fragment>
        ))}
      </div>
    </>
  );
}
