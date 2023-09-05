import { Fragment, useEffect, useRef, useState } from "react"
import { useRouter } from "next/router"
import { connect, E, getAllLocalStorageItems, getRefValue, isTrue, preventDefault, processEvent, refs, set_val, uploadFiles } from "/utils/state"
import "focus-visible/dist/focus-visible"
import { Box, Button, Container, HStack, Input, option, Select, Text, useColorMode } from "@chakra-ui/react"
import NextHead from "next/head"


export default function Component() {
  const [state, setState] = useState({"chat_history": [], "is_hydrated": false, "new_session_name": "", "question": "", "selected_session_name": "", "session_names": [], "events": [{"name": "state.hydrate"}], "files": []})
  const [result, setResult] = useState({"state": null, "events": [], "final": true, "processing": false})
  const [notConnected, setNotConnected] = useState(false)
  const router = useRouter()
  const socket = useRef(null)
  const { isReady } = router
  const { colorMode, toggleColorMode } = useColorMode()
  const focusRef = useRef();
  
  // Function to add new events to the event queue.
  const Event = (events, _e) => {
      preventDefault(_e);
      setState(state => ({
        ...state,
        events: [...state.events, ...events],
      }))
  }

  // Function to add new files to be uploaded.
  const File = files => setState(state => ({
    ...state,
    files,
  }))

  // Main event loop.
  useEffect(()=> {
    // Skip if the router is not ready.
    if (!isReady) {
      return;
    }

    // Initialize the websocket connection.
    if (!socket.current) {
      connect(socket, state, setState, result, setResult, router, ['websocket', 'polling'], setNotConnected)
    }

    // If we are not processing an event, process the next event.
    if (!result.processing) {
      processEvent(state, setState, result, setResult, router, socket.current)
    }

    // If there is a new result, update the state.
    if (result.state != null) {
      // Apply the new result to the state and the new events to the queue.
      setState(state => ({
        ...result.state,
        events: [...state.events, ...result.events],
      }))

      // Reset the result.
      setResult(result => ({
        state: null,
        events: [],
        final: true,
        processing: !result.final,
      }))

      // Process the next event.
      processEvent(state, setState, result, setResult, router, socket.current)
    }
  })

  // Set focus to the specified element.
  useEffect(() => {
    if (focusRef.current) {
      focusRef.current.focus();
    }
  })

  // Route after the initial page hydration.
  useEffect(() => {
    const change_complete = () => Event([E('state.hydrate', {})])
    router.events.on('routeChangeComplete', change_complete)
    return () => {
      router.events.off('routeChangeComplete', change_complete)
    }
  }, [router])


  return (
  <Fragment><Fragment>
  <Container>
  <Box>
  {state.chat_history.map((cpotkgyz, i) => (
  <Box key={i} sx={{"marginY": "1em"}}>
  <Box sx={{"textAlign": "right"}}>
  {cpotkgyz.at(0)}
</Box>
  <Box sx={{"textAlign": "left", "whitespace": "break-spaces"}}>
  {cpotkgyz.at(1)}
</Box>
</Box>
))}
</Box>
  <HStack>
  <Input onBlur={_e => Event([E("state.set_question", {value:_e.target.value})], _e)} placeholder="Type a message..." type="text"/>
  <Button onClick={_e => Event([E("state.answer", {})], _e)} sx={{"marginLeft": "1em"}}>
  {`Send`}
</Button>
</HStack>
  <Box>
  <Text>
  {`Session Manager`}
</Text>
  <Input onBlur={_e => Event([E("state.set_new_session_name", {value:_e.target.value})], _e)} placeholder="session name..." type="text"/>
  <Button onClick={_e => Event([E("state.add_session", {})], _e)}>
  {`New Session`}
</Button>
  <Select onChange={_e => Event([E("state.set_selected_session_name", {value:_e.target.value})], _e)} placeholder="select a session...">
  {state.session_names.map((viwzllxv, i) => (
  <option key={i} value={viwzllxv}>
  {viwzllxv}
</option>
))}
</Select>
  <Button onClick={_e => Event([E("state.load_selected_session", {})], _e)}>
  {`Load Session`}
</Button>
  <Button onClick={_e => Event([E("state.delete_selected_session", {})], _e)}>
  {`Delete Selected Session`}
</Button>
</Box>
</Container>
  <NextHead>
  <title>
  {`Reflex App`}
</title>
  <meta content="A Reflex app." name="description"/>
  <meta content="favicon.ico" property="og:image"/>
</NextHead>
</Fragment>
    </Fragment>
  )
}
