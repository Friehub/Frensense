// SAFE: returns a new state object using array spread and object spread

import { createSlice, PayloadAction } from '@reduxjs/toolkit'

interface UserState {
  name: string
  roles: string[]
}

const initialState: UserState = { name: '', roles: [] }

const userSlice = createSlice({
  name: 'user',
  initialState,
  reducers: {
    addRole(state, action: PayloadAction<string>) {
      state.roles = [...state.roles, action.payload]
    },
    setName(state, action: PayloadAction<string>) {
      state.name = action.payload.toUpperCase()
    },
  },
})

export const { addRole, setName } = userSlice.actions
export default userSlice.reducer
