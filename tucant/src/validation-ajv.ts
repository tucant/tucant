import Ajv, { JTDDataType } from "ajv/dist/jtd";
const ajv = new Ajv({ code: { esm: true } });

// https://ajv.js.org/json-type-definition.html
// https://ajv.js.org/guide/getting-started.html
// https://ajv.js.org/guide/typescript.html

const courseSchema = {
  properties: {
    tucan_id: { type: "string" },
    tucan_last_checked: { type: "timestamp" },
    title: { type: "string" },
    course_id: { type: "string" },
    sws: { type: "int16" },
    content: { type: "string" },
    done: { type: "boolean" },
  },
} as const;

export type CourseType = JTDDataType<typeof courseSchema>;

export const courseValidator = ajv.compile(courseSchema);
