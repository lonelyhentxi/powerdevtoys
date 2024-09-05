import { invoke } from "@tauri-apps/api/core";
import './global.css';
import { FunctionsPage } from "./pages/_functions";

function App() {

  async function _greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    await invoke("greet", { name: "powerdevtoys" })
  }

  return (
    <FunctionsPage />
  );
}

export default App;
