import { Link } from "../Navigation";
import { MaybeCompleteCourse } from "../api";

export function CourseEntry({ c }: { c: MaybeCompleteCourse }) {
  return (
    <Link
      key={c.value.tucan_id}
      className="list-group-item list-group-item-action"
      to={`/course/${c.value.tucan_id}`}
    >
      [V] {c.value.title}
    </Link>
  );
}
