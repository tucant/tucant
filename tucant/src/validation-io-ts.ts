import { type, string, number, boolean, strict, TypeOf } from "io-ts";

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

alert(
  CourseSchema.is({
    tucan_id: "",
    tucan_lasjt_checked: "",
    title: "",
    course_id: "",
    sws: 0,
    content: "",
    done: false,
    test: 1,
  })
);

alert(
  JSON.stringify(
    CourseSchema.decode({
      tucan_id: "",
      tucan_lasjt_checked: "",
      title: "",
      course_id: "",
      sws: 0,
      content: "",
      done: false,
      test: 1,
    })
  )
);
