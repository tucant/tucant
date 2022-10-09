import { createContext } from "react";

// This is in a separate file because of https://github.com/vitejs/vite/issues/3301#issuecomment-1192661323
// wait for https://github.com/vitejs/vite/pull/10239 release?
export const NavigationContext = createContext<React.TransitionStartFunction>(
  () => {
    console.error("no NavigationContext set");
  }
);
