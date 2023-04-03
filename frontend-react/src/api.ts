// This file is automatically generated at startup. Do not modify.
import { genericFetch } from "./api_base"
export async function course(input: string): Promise<WithTucanUrlW0NvdXJzZSwgQ291cnNlR3JvdXBbXSwgQ291cnNlRXZlbnRbXSwgTW9kdWxlW11d> {
        return await genericFetch("http://localhost:8080/course", input) as WithTucanUrlW0NvdXJzZSwgQ291cnNlR3JvdXBbXSwgQ291cnNlRXZlbnRbXSwgTW9kdWxlW11d
}
export async function course_group(input: string): Promise<WithTucanUrlW0NvdXJzZSwgQ291cnNlR3JvdXAsIENvdXJzZUdyb3VwRXZlbnRbXV0> {
        return await genericFetch("http://localhost:8080/course-group", input) as WithTucanUrlW0NvdXJzZSwgQ291cnNlR3JvdXAsIENvdXJzZUdyb3VwRXZlbnRbXV0
}
export async function courses(input: string | null): Promise<WithTucanUrlW1ZWTWVudUl0ZW0sIFZWTWVudUl0ZW1bXSwgQ291cnNlW11d> {
        return await genericFetch("http://localhost:8080/courses", input) as WithTucanUrlW1ZWTWVudUl0ZW0sIFZWTWVudUl0ZW1bXSwgQ291cnNlW11d
}
export async function exam(input: string): Promise<WithTucanUrlW0V4YW0sIE1vZHVsZVtdLCBDb3Vyc2VbXV0> {
        return await genericFetch("http://localhost:8080/exam", input) as WithTucanUrlW0V4YW0sIE1vZHVsZVtdLCBDb3Vyc2VbXV0
}
export async function get_modules(input: string | null): Promise<WithTucanUrlTW9kdWxlTWVudVJlc3BvbnNl> {
        return await genericFetch("http://localhost:8080/modules", input) as WithTucanUrlTW9kdWxlTWVudVJlc3BvbnNl
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
export async function module(input: string): Promise<WithTucanUrlTW9kdWxlUmVzcG9uc2U> {
        return await genericFetch("http://localhost:8080/module", input) as WithTucanUrlTW9kdWxlUmVzcG9uc2U
}
export async function my_courses(input: null): Promise<WithTucanUrlQ291cnNlT3JDb3Vyc2VHcm91cFtd> {
        return await genericFetch("http://localhost:8080/my-courses", input) as WithTucanUrlQ291cnNlT3JDb3Vyc2VHcm91cFtd
}
export async function my_exams(input: null): Promise<WithTucanUrlW1tNb2R1bGUsIEV4YW1dW10sIFtDb3Vyc2UsIEV4YW1dW11d> {
        return await genericFetch("http://localhost:8080/my-exams", input) as WithTucanUrlW1tNb2R1bGUsIEV4YW1dW10sIFtDb3Vyc2UsIEV4YW1dW11d
}
export async function my_modules(input: null): Promise<WithTucanUrlTW9kdWxlW10> {
        return await genericFetch("http://localhost:8080/my-modules", input) as WithTucanUrlTW9kdWxlW10
}
export async function search_course(input: string): Promise<SearchResult[]> {
        return await genericFetch("http://localhost:8080/search-course", input) as SearchResult[]
}
export async function search_module(input: string): Promise<SearchResult[]> {
        return await genericFetch("http://localhost:8080/search-modules", input) as SearchResult[]
}
export async function search_module_opensearch(input: string): Promise<SearchResult[]> {
        return await genericFetch("http://localhost:8080/search-modules-opensearch", input) as SearchResult[]
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
export type CourseEvent =
{
  course: number[],
  timestamp_start: string,
  timestamp_end: string,
  room: string,
  teachers: string,
}
export type CourseGroup =
{
  tucan_id: string,
  course: string,
  title: string,
  done: boolean,
}
export type CourseGroupEvent =
{
  course: number[],
  timestamp_start: string,
  timestamp_end: string,
  room: string,
  teachers: string,
}
export type CourseOrCourseGroup =
 | { type: "Course", value: [Course, CourseGroup[], CourseEvent[], Module[]] }
 | { type: "CourseGroup", value: [CourseGroup, CourseGroupEvent[]] }

export type Exam =
{
  tucan_id: string,
  exam_type: string,
  semester: string,
  exam_time_start: string | null,
  exam_time_end: string | null,
  registration_start: string,
  registration_end: string,
  unregistration_start: string,
  unregistration_end: string,
  examinator: string | null,
  room: string | null,
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
  done: boolean,
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
  entries: Registration,
  path: ModuleMenuPathPart[][],
}
export type ModuleResponse =
{
  module: Module,
  courses: Course[],
  path: ModuleMenuPathPart[][],
}
export type Registration =
{
  submenus: ModuleMenu[],
  modules_and_courses: [Module, Course[]][],
}
export type SearchResult =
{
  tucan_id: string,
  title: string,
  excerpt: string,
  rank: number,
}
export type VVMenuItem =
{
  tucan_id: string,
  tucan_last_checked: string,
  name: string,
  done: boolean,
  parent: string | null,
}
export type WithTucanUrlQ291cnNlT3JDb3Vyc2VHcm91cFtd =
{
  tucan_url: string,
  inner: CourseOrCourseGroup[],
}
export type WithTucanUrlTW9kdWxlTWVudVJlc3BvbnNl =
{
  tucan_url: string,
  inner: ModuleMenuResponse,
}
export type WithTucanUrlTW9kdWxlUmVzcG9uc2U =
{
  tucan_url: string,
  inner: ModuleResponse,
}
export type WithTucanUrlTW9kdWxlW10 =
{
  tucan_url: string,
  inner: Module[],
}
export type WithTucanUrlW0NvdXJzZSwgQ291cnNlR3JvdXAsIENvdXJzZUdyb3VwRXZlbnRbXV0 =
{
  tucan_url: string,
  inner: [Course, CourseGroup, CourseGroupEvent[]],
}
export type WithTucanUrlW0NvdXJzZSwgQ291cnNlR3JvdXBbXSwgQ291cnNlRXZlbnRbXSwgTW9kdWxlW11d =
{
  tucan_url: string,
  inner: [Course, CourseGroup[], CourseEvent[], Module[]],
}
export type WithTucanUrlW0V4YW0sIE1vZHVsZVtdLCBDb3Vyc2VbXV0 =
{
  tucan_url: string,
  inner: [Exam, Module[], Course[]],
}
export type WithTucanUrlW1ZWTWVudUl0ZW0sIFZWTWVudUl0ZW1bXSwgQ291cnNlW11d =
{
  tucan_url: string,
  inner: [VVMenuItem, VVMenuItem[], Course[]],
}
export type WithTucanUrlW1tNb2R1bGUsIEV4YW1dW10sIFtDb3Vyc2UsIEV4YW1dW11d =
{
  tucan_url: string,
  inner: [[Module, Exam][], [Course, Exam][]],
}