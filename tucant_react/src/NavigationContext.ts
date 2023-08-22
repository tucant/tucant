import React, { createContext } from "react";

export const NavigationContext = createContext<React.TransitionStartFunction>(
  () => {
    console.error("no NavigationContext set");
  },
);
