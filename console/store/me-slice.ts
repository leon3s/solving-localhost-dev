import { createSlice, type PayloadAction } from "@reduxjs/toolkit"

import type { User } from "@/types/user"

export interface MeState {
  data: User | null
}

const initialState: MeState = {
  data: null,
}

const meSlice = createSlice({
  name: "me",
  initialState,
  reducers: {
    setMe: (state, action: PayloadAction<User>) => {
      state.data = action.payload
    },
  },
})

export const { setMe } = meSlice.actions
export default meSlice.reducer
