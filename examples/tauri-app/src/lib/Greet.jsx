import React, { useState } from 'react'
import { invoke } from "@tauri-apps/api/core"

function Greet() {
  const [name, setName] = useState("")
  const [greetMsg, setGreetMsg] = useState("")

  async function greet() {
    // Learn more about Tauri commands at https://v2.tauri.app/develop/calling-rust/#commands
    const message = await invoke("greet", { name })
    setGreetMsg(message)
  }

  return (
    <div>
      <div className="row">
        <input 
          id="greet-input" 
          placeholder="Enter a name..." 
          value={name}
          onChange={(e) => setName(e.target.value)}
        />
        <button onClick={greet}>
          Greet
        </button>
      </div>
      <p>{greetMsg}</p>
    </div>
  )
}

export default Greet
