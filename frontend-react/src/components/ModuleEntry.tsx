import { Link } from "../Navigation";
import { MaybeCompleteModule } from "../api";

export function ModuleEntry({ module }: { module: MaybeCompleteModule }) {
  return (
    <Link
      key={module.value.tucan_id}
      className="list-group-item list-group-item-action"
      to={`/module/${String(module.value.tucan_id)}`}
    >
      [M] {module.value.title}{" "}
      {module.type === "Complete" && <span className="badge rounded-pill text-bg-primary">{`${
        module.value.credits
      } Credits`}</span>}
    </Link>
  );
}
