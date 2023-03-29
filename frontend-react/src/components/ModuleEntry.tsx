import { Module } from "../api";
import { Link } from "../Navigation";

export function ModuleEntry({ module }: { module: Module }) {
    return (
        <Link
            key={module.tucan_id}
            className="list-group-item list-group-item-action"
            to={`/module/${String(module.tucan_id)}`}
        >
            {module.title} <span className="badge rounded-pill text-bg-primary">{`${module.credits ?? 0
                } Credits`}</span>
        </Link>
    );
}
