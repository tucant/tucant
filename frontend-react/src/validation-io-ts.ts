// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import {
  string,
  number,
  boolean,
  strict,
  TypeOf,
  nullType,
  union,
  tuple,
  array,
  undefined,
  partial,
  intersection,
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

export const LoginResponseSchema = strict({
  success: boolean,
});

export type LoginResponseType = TypeOf<typeof LoginResponseSchema>;

export const SearchResultSchema = array(
  tuple([string, string, string, number])
);

export type SearchResultType = TypeOf<typeof SearchResultSchema>;

export const WelcomeSchema = string;

export type WelcomeType = TypeOf<typeof WelcomeSchema>;

export const ModuleMenuSchema = strict({
  tucan_id: string,
  tucan_last_checked: string,
  name: string,
  child_type: number,
});

export type ModuleMenuType = TypeOf<typeof ModuleMenuSchema>;

export const ModulesResponseSchema = union([
  intersection([
    strict({
      Submenu: union([array(ModuleMenuSchema), undefined]),
    }),
    partial({
      Modules: undefined,
    }),
  ]),
  intersection([
    strict({
      Modules: union([array(ModuleSchema), undefined]),
    }),
    partial({
      Submenu: undefined,
    }),
  ]),
]);

export type ModulesResponseType = TypeOf<typeof ModulesResponseSchema>;
