import { CourseOrCourseGroup } from "../api";
import { Link } from "../Navigation";

export function CourseOrCourseGroupEntry({ c }: { c: CourseOrCourseGroup }) {
  return c.type === "Course" ? (
    <Link
      key={c.value[0].value.tucan_id}
      className="list-group-item list-group-item-action"
      to={`/course/${c.value[0].value.tucan_id}`}
    >
      {c.value[0].value.title}
    </Link>
  ) : (
    <Link
      key={c.value[0].tucan_id}
      className="list-group-item list-group-item-action"
      to={`/course-group/${c.value[0].tucan_id}`}
    >
      {c.value[0].title}
    </Link>
  );
}
