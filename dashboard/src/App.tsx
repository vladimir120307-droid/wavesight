import { useEffect, useMemo } from "react";
import { NodeCard } from "./components/NodeCard";
import { useDashboard } from "./lib/state";
import { StreamClient } from "./lib/stream";

export default function App() {
  const ingest = useDashboard((s) => s.ingest);
  const nodes = useDashboard((s) => s.nodes);

  useEffect(() => {
    const host = window.location.host || "localhost:8081";
    const proto = window.location.protocol === "https:" ? "wss:" : "ws:";
    const url = `${proto}//${host}/api/v1/stream`;
    const client = new StreamClient(url);
    const unsubscribe = client.subscribe(ingest);
    client.connect();
    return () => {
      unsubscribe();
      client.close();
    };
  }, [ingest]);

  const nodeList = useMemo(() => Object.values(nodes), [nodes]);
  const connectedCount = nodeList.length;

  return (
    <div className="max-w-6xl mx-auto px-6 py-8 space-y-6">
      <header className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold tracking-tight">WaveSight</h1>
          <p className="text-sm text-zinc-400">
            Live CSI · {connectedCount} node{connectedCount === 1 ? "" : "s"}{" "}
            connected
          </p>
        </div>
        <a
          className="text-xs text-zinc-500 hover:text-zinc-300"
          href="https://github.com/vladimir120307-droid/wavesight"
          target="_blank"
          rel="noreferrer"
        >
          github ↗
        </a>
      </header>

      {nodeList.length === 0 ? (
        <div className="panel text-zinc-400">
          <p className="font-medium">No nodes streaming yet.</p>
          <ol className="list-decimal list-inside text-sm mt-2 space-y-1">
            <li>
              Flash <code>firmware/esp32-csi-node</code> on an ESP32-S3.
            </li>
            <li>
              Set <code>WAVESIGHT_SERVER_URI</code> to{" "}
              <code>ws://&lt;this-machine&gt;:8080/ingest</code>.
            </li>
            <li>Power the board and refresh this page.</li>
          </ol>
        </div>
      ) : (
        <div className="grid md:grid-cols-2 gap-4">
          {nodeList.map((n) => (
            <NodeCard
              key={n.node}
              node={n.node}
              lastUpdate={n.lastUpdate}
              history={n.history}
            />
          ))}
        </div>
      )}

      <footer className="text-xs text-zinc-500 pt-6">
        WaveSight alpha · honest mode on · local-first
      </footer>
    </div>
  );
}
