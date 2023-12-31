import { createContext } from "react"
import { E } from "/utils/state.js"

export const initialState = {"chat_history": [], "game_sessions": [], "get_chat_history": [], "get_game_sessions": [{"id": 1, "name": "test 1"}, {"id": 2, "name": "test 2"}, {"id": 3, "name": "test 3"}, {"id": 4, "name": "test 1"}], "get_session_names": ["test 1", "test 2", "test 3", "test 1"], "is_hydrated": false, "loaded_game_session": null, "loaded_session_name": "not loaded", "new_session_name": "", "not_loaded_show": false, "question": "", "selected_session_name": "", "session_names": []}
export const initialEvents = [E('state.hydrate', {})]
export const StateContext = createContext(null);
export const EventLoopContext = createContext(null);