// SAFE: uses Immer's produce to create an immutable update manually

import { createSlice, PayloadAction } from '@reduxjs/toolkit'
import { produce } from 'immer'

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
      return produce(state, (draft) => {
        draft.roles.push(action.payload)
      })
    },
    setName(state, action: PayloadAction<string>) {
      return { ...state, name: action.payload.toUpperCase() }
    },
  },
})

export const { addRole, setName } = userSlice.actions
export default userSlice.reducer
