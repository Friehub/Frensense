// [frensense]
// observation: Redux reducer mutates the state object in-place instead of returning a new copy
// impact: React-Redux cannot detect the change; UI does not re-render; time-travel debugging broken
// improvement: always return a new state object via spread or Immer

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
      state.roles.push(action.payload)
    },
    setName(state, action: PayloadAction<string>) {
      state.name = action.payload.toUpperCase()
    },
  },
})

export const { addRole, setName } = userSlice.actions
export default userSlice.reducer
