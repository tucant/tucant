import { Course } from "../api";
import { Link } from "../Navigation";

export function CourseEntry({ c }: { c: Course }) {
  return (
    <Link
      key={c.tucan_id}
      className="list-group-item list-group-item-action"
      to={`/course/${c.tucan_id}`}
    >
      {c.title}
    </Link>
  );
}
