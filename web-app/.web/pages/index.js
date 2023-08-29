import { Fragment, useContext, useEffect, useRef, useState } from "react"
import { useRouter } from "next/router"
import { E, getAllLocalStorageItems, getRefValue, getRefValues, isTrue, preventDefault, refs, set_val, uploadFiles, useEventLoop } from "/utils/state"
import { EventLoopContext, StateContext } from "/utils/context.js"
import "focus-visible/dist/focus-visible"
import { Box, Button, Container, HStack, Input, option, Select, Text, useColorMode } from "@chakra-ui/react"
import NextHead from "next/head"


export default function Component() {
  const state = useContext(StateContext)
  const router = useRouter()
  const { colorMode, toggleColorMode } = useColorMode()
  const focusRef = useRef();
  
  // Main event loop.
  const [Event, notConnected] = useContext(EventLoopContext)

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
  {state.chat_history.map((yrmqslun, i) => (
  <Box key={i} sx={{"marginY": "1em"}}>
  <Box sx={{"textAlign": "right"}}>
  {yrmqslun.at(0)}
</Box>
  <Box sx={{"textAlign": "left", "whitespace": "break-spaces"}}>
  {yrmqslun.at(1)}
</Box>
</Box>
))}
</Box>
  <HStack>
  <Input onBlur={_e => Event([E("state.set_question", {value:_e.target.value})], _e)} placeholder={`Type a message...`} type={`text`}/>
  <Button onClick={_e => Event([E("state.answer", {})], _e)} sx={{"marginLeft": "1em"}}>
  {`Send`}
</Button>
</HStack>
  <Box>
  <Text>
  {`Session Manager`}
</Text>
  <Input onBlur={_e => Event([E("state.set_new_session_name", {value:_e.target.value})], _e)} placeholder={`session name...`} type={`text`}/>
  <Button onClick={_e => Event([E("state.add_game_session", {})], _e)}>
  {`New Session`}
</Button>
  <Select onChange={_e => Event([E("state.set_selected_session_name", {value:_e.target.value})], _e)} placeholder={`select a session...`}>
  {state.session_names.map((jarenfjt, i) => (
  <option key={i} value={jarenfjt}>
  {jarenfjt}
</option>
))}
</Select>
  <Button onClick={_e => Event([E("state.load_selected_session", {})], _e)}>
  {`Load Session`}
</Button>
  <Button onClick={_e => Event([E("state.delete_selected_session", {})], _e)}>
  {`Delete Selected Session`}
</Button>
  <Box>
  <Text>
  {`Loaded Session: ${state.loaded_session_name}`}
</Text>
</Box>
</Box>
</Container>
  <NextHead>
  <title>
  {`Reflex App`}
</title>
  <meta content={`A Reflex app.`} name={`description`}/>
  <meta content={`favicon.ico`} property={`og:image`}/>
</NextHead>
</Fragment>
    </Fragment>
  )
}
