
// This file is automatically generated at startup. Do not modify.
import { genericFetch } from "./api_base"
export async function course(input: string): Promise<Course> {
        return await genericFetch("http://localhost:8080/course", input) as Course
}
export async function get_modules(input: string | null): Promise<ModuleMenuResponse> {
        return await genericFetch("http://localhost:8080/modules", input) as ModuleMenuResponse
}
export async function index(input: null): Promise<string> {
        return await genericFetch("http://localhost:8080/", input) as string
}
export async function login(input: Login): Promise<LoginResult> {
        return await genericFetch("http://localhost:8080/login", input) as LoginResult
}
export async function logout(input: null): Promise<null> {
        return await genericFetch("http://localhost:8080/logout", input) as null
}
export async function module(input: string): Promise<ModuleResponse> {
        return await genericFetch("http://localhost:8080/module", input) as ModuleResponse
}
export async function my_courses(input: null): Promise<Course[]> {
        return await genericFetch("http://localhost:8080/my_courses", input) as Course[]
}
export async function my_modules(input: null): Promise<Module[]> {
        return await genericFetch("http://localhost:8080/my_modules", input) as Module[]
}
export async function search_course(input: string): Promise<SearchResult[]> {
        return await genericFetch("http://localhost:8080/search-course", input) as SearchResult[]
}
export async function search_module(input: string): Promise<SearchResult[]> {
        return await genericFetch("http://localhost:8080/search-module", input) as SearchResult[]
}
export type Course =
{
  tucan_id: string,
  tucan_last_checked: string,
  title: string,
  course_id: string,
  sws: number,
  content: string,
  done: boolean,
}
export type Login =
{
  username: string,
  password: string,
}
export type LoginResult =
{
  success: boolean,
}
export type Module =
{
  tucan_id: string,
  tucan_last_checked: string,
  title: string,
  module_id: string,
  credits: number | null,
  content: string,
  done: boolean,
}
export type ModuleMenu =
{
  tucan_id: string,
  tucan_last_checked: string,
  name: string,
  child_type: number,
  parent: string,
}
export type ModuleMenuPathPart =
{
  parent: number[] | null,
  tucan_id: string,
  name: string,
  leaf: boolean,
}
export type ModuleMenuResponse =
{
  module_menu: ModuleMenu,
  entries: RegistrationEnum,
  path: ModuleMenuPathPart[][],
}
export type ModuleResponse =
{
  module: Module,
  path: ModuleMenuPathPart[][],
}
export type RegistrationEnum =
 | { type: "Submenu", value: ModuleMenu[] }
 | { type: "Modules", value: Module[] }

export type SearchResult =
{
  tucan_id: string,
  title: string,
  excerpt: string,
  rank: number,
}