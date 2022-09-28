import {
  type,
  string,
  number,
  boolean,
  strict,
  TypeOf,
  nullType,
  union,
} from "io-ts";

// https://github.com/gcanti/io-ts/blob/master/index.md

export const CourseSchema = strict({
  tucan_id: string,
  tucan_last_checked: string,
  title: string,
  course_id: string,
  sws: number,
  content: string,
  done: boolean,
});

export type CourseType = TypeOf<typeof CourseSchema>;

export const ModuleSchema = strict({
  tucan_id: string,
  tucan_last_checked: string,
  title: string,
  module_id: string,
  credits: union([number, nullType]),
  content: string,
  done: boolean,
});

export type ModuleType = TypeOf<typeof ModuleSchema>;
