type Module = {
  tucan_id: boolean,
  tucan_last_checked: string,
  title: string,
  module_id: string,
  credits: boolean,
  content: string,
  done: boolean,
}
function logout(input: void): void {

}
type ModuleResponse = {
  module: Module,
  path: boolean,
}
function login(input: Login): LoginResult {
  const response = await fetch("http://localhost:8080/login", {
    credentials: "include",
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "x-csrf-protection": "tucant",
    },
    body: JSON.stringify(input),
  });
  return await response.json()
}
function get_modules(input: string) -> ModuleMenuResponse {

}

type LoginResult = {
  success: boolean,
}
type Login = {
  username: string,
  password: string,
}
function module(input: string) -> ModuleResponse {

}
type ModuleMenu = {
  tucan_id: boolean,
  tucan_last_checked: string,
  name: string,
  child_type: number,
  parent: boolean,
}
function search_course(input: string) -> SearchResult {

}
type RegistrationEnum = {
  Submenu: [boolean,
],
  Modules: [boolean,
],
}
function index(input: void) -> string {

}
function search_module(input: string) -> SearchResult {

}
type SearchResult = {
  a: string,
  b: string,
  c: string,
  d: number,
}
function course(input: string) -> Course {

}
type Course = {
  tucan_id: boolean,
  tucan_last_checked: string,
  title: string,
  course_id: string,
  sws: number,
  content: string,
  done: boolean,
}
type ModuleMenuResponse = {
  module_menu: ModuleMenu,
  entries: RegistrationEnum,
  path: boolean,
}