import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import { Button } from '@ui/button'
import { Input } from '@ui/input';
import '@styles/global.css';
import s from "./App.module.scss";
import cx from 'clsx';

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }

  return (
    <div className="flex flex-col text-center justify-center pt-[10vh] m-0 p-16">
      <h1 className="text-center">Welcome to Tauri!</h1>

      <div className="flex flex-row items-center justify-center">
        <a href="https://rsbuild.dev/" target="_blank">
          <img src="/rsbuild.svg" className={cx(s.logo, s.rsbuild)} alt="Rsbuild logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className={cx(s.logo, s.tauri)} alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className={cx(s.logo, s.react)} alt="React logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="flex flex-row items-center gap-5"
        onSubmit={(e) => {
          e.preventDefault();
          greet();
        }}
      >
        <Input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="Enter a name..."
        />
        <Button type="submit">Greet</Button>
      </form>

      <p>{greetMsg}</p>
    </div>
  );
}

export default App;
