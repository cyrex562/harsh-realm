import { createContext } from "react"
import { E } from "/utils/state.js"

export const initialState = {"chat_history": [], "game_sessions": [], "is_hydrated": false, "new_session_name": "", "question": "", "selected_game_session": null, "selected_session_name": "", "session_names": []}
export const initialEvents = [E('state.hydrate', {})]
export const StateContext = createContext(null);
export const EventLoopContext = createContext(null);