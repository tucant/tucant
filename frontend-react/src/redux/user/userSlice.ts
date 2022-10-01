import { createSlice, PayloadAction } from "@reduxjs/toolkit";

interface UserState {
  loggedIn: boolean;
}

const initialState: UserState = {
  loggedIn: false,
};

export const userSlice = createSlice({
  name: "user",
  initialState,
  reducers: {
    setLoggedIn: (state, action: PayloadAction<boolean>) => {
      state.loggedIn = action.payload;
    },
  },
});

export const { setLoggedIn } = userSlice.actions;

export default userSlice.reducer;
