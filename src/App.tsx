import { useState, useRef, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

export default function App() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [zoom, setZoom] = useState(0.004);
  const [offset, setOffSet] = useState({ x: -0.5, y: 0.0 });

  const renderFractal = async () => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const width = canvas.width;
    const height = canvas.height;

    const buffer: number[] = await invoke("get_fractal_image", {
      width,
      height,
      zoom,
      x: offset.x,
      y: offset.y,
    });

    const ctx = canvas.getContext("2d");
    if (ctx) {
      const uint8Array = new Uint8ClampedArray(buffer);
      const imageData = new ImageData(uint8Array, width, height);
      ctx.putImageData(imageData, 0, 0);
    }
  };

  useEffect(() => {
    renderFractal();
  }, [zoom, offset]);

  return (
    <div style={{ display: "flex", flexDirection: "column", alignItems: "center" }}>
      <h1>ChromaCor Engine SLAYYY</h1>
      <canvas ref={canvasRef} width={800} height={600} style={{ border: "1px solid white" }} />
      <div style={{ marginTop: 10, display: "flex", gap: "8px" }}>
        <button onClick={() => setZoom(z => z * 0.8)}>Nagyítás (+)</button>
        <button onClick={() => setZoom(z => z * 1.25)}>Kicsinyítés (-)</button>
        <button onClick={() => setOffSet(pos => ({ ...pos, x: pos.x - 50 * zoom }))}>Balra</button>
        <button onClick={() => setOffSet(pos => ({ ...pos, x: pos.x + 50 * zoom }))}>Jobbra</button>
        <button onClick={() => setOffSet(pos => ({ ...pos, y: pos.y - 50 * zoom }))}>Fel</button>
        <button onClick={() => setOffSet(pos => ({ ...pos, y: pos.y + 50 * zoom }))}>Le</button>
      </div>
    </div>
  );
}