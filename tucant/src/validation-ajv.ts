import Ajv, { JTDDataType } from "ajv/dist/jtd";
const ajv = new Ajv({ code: { esm: true } });

// dist/assets/index.459774db.js   515.54 KiB / gzip: 164.38 KiB

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
