import { useEffect, useRef } from "react";
import type { StreamUpdate } from "../lib/stream";

interface Props {
  history: StreamUpdate[];
  width?: number;
  height?: number;
}

export function Spectrogram({ history, width = 480, height = 160 }: Props) {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    ctx.fillStyle = "#13161c";
    ctx.fillRect(0, 0, width, height);

    if (history.length === 0) return;

    const cols = Math.min(history.length, width);
    const lastCols = history.slice(-cols);
    const colWidth = width / cols;

    lastCols.forEach((update, ci) => {
      const bins = update.amplitude;
      if (bins.length === 0) return;
      const rowHeight = height / bins.length;
      bins.forEach((amp, ri) => {
        const v = Math.max(0, Math.min(1, amp));
        const r = Math.round(20 + v * 80);
        const g = Math.round(120 + v * 130);
        const b = Math.round(200 + v * 55);
        ctx.fillStyle = `rgb(${r},${g},${b})`;
        ctx.fillRect(ci * colWidth, ri * rowHeight, colWidth + 1, rowHeight + 1);
      });
    });
  }, [history, width, height]);

  return (
    <canvas
      ref={canvasRef}
      width={width}
      height={height}
      className="rounded border border-line w-full"
    />
  );
}
