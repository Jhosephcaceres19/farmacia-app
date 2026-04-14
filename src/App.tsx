import "./App.css";
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

export const App = () => {
  const [saludo, setSaludo] = useState("");

  useEffect(() => {
    const obtenerSaludo = async () => {
      const resp = await invoke("saludar", { nombre: "jhoseph" });
      setSaludo(resp);
    };

    obtenerSaludo();
  }, []);

  return (
    <h1>{saludo}</h1>
  );
};