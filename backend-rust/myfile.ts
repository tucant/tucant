async function logout(input: void): Promise<void> {
  const response = await fetch("http://localhost:8080/logout", {
    credentials: "include",
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "x-csrf-protection": "tucant",
    },
    body: JSON.stringify(input),
  });
  return await response.json();
}
type LoginResult = {
  success: boolean;
};
async function module(input: string): Promise<ModuleResponse> {
  const response = await fetch("http://localhost:8080/module", {
    credentials: "include",
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "x-csrf-protection": "tucant",
    },
    body: JSON.stringify(input),
  });
  return await response.json();
}
type ModuleMenu = {
  tucan_id: boolean;
  tucan_last_checked: string;
  name: string;
  child_type: number;
  parent: boolean;
};
type SearchResult = {
  a: string;
  b: string;
  c: string;
  d: number;
};
type Module = {
  tucan_id: boolean;
  tucan_last_checked: string;
  title: string;
  module_id: string;
  credits: boolean;
  content: string;
  done: boolean;
};
type ModuleResponse = {
  module: Module;
  path: boolean;
};
type Course = {
  tucan_id: boolean;
  tucan_last_checked: string;
  title: string;
  course_id: string;
  sws: number;
  content: string;
  done: boolean;
};
async function get_modules(input: string): Promise<ModuleMenuResponse> {
  const response = await fetch("http://localhost:8080/modules", {
    credentials: "include",
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "x-csrf-protection": "tucant",
    },
    body: JSON.stringify(input),
  });
  return await response.json();
}
async function search_course(input: string): Promise<SearchResult> {
  const response = await fetch("http://localhost:8080/search-course", {
    credentials: "include",
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "x-csrf-protection": "tucant",
    },
    body: JSON.stringify(input),
  });
  return await response.json();
}
type RegistrationEnum = {
  Submenu: [boolean];
  Modules: [boolean];
};
async function login(input: Login): Promise<LoginResult> {
  const response = await fetch("http://localhost:8080/login", {
    credentials: "include",
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "x-csrf-protection": "tucant",
    },
    body: JSON.stringify(input),
  });
  return await response.json();
}
async function index(input: void): Promise<string> {
  const response = await fetch("http://localhost:8080/", {
    credentials: "include",
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "x-csrf-protection": "tucant",
    },
    body: JSON.stringify(input),
  });
  return await response.json();
}
type Login = {
  username: string;
  password: string;
};
async function course(input: string): Promise<Course> {
  const response = await fetch("http://localhost:8080/course", {
    credentials: "include",
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "x-csrf-protection": "tucant",
    },
    body: JSON.stringify(input),
  });
  return await response.json();
}
async function search_module(input: string): Promise<SearchResult> {
  const response = await fetch("http://localhost:8080/search-module", {
    credentials: "include",
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "x-csrf-protection": "tucant",
    },
    body: JSON.stringify(input),
  });
  return await response.json();
}
type ModuleMenuResponse = {
  module_menu: ModuleMenu;
  entries: RegistrationEnum;
  path: boolean;
};
