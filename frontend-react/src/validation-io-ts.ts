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
  strict({
    Submenu: array(ModuleMenuSchema),
  }),
  strict({
    Modules: array(ModuleSchema),
  }),
]);

export type ModulesResponseType = TypeOf<typeof ModulesResponseSchema>;

export const ModuleMenuPathPart = strict({
  tucan_id: string,
  name: string,
});

export const ModuleMenuResponse = strict({
  entries: ModulesResponseSchema,
  path: array(array(ModuleMenuPathPart)),
});

export const ModuleResponse = strict({
  module: ModuleSchema,
  path: array(array(ModuleMenuPathPart)),
});
