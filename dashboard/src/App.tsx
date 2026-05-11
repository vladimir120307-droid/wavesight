import { useEffect, useMemo, useState } from "react";
import { NodeCard } from "./components/NodeCard";
import { SettingsPanel } from "./components/SettingsPanel";
import { t } from "./lib/i18n";
import { useSettings } from "./lib/settings";
import { useDashboard } from "./lib/state";
import { StreamClient } from "./lib/stream";

export default function App() {
  const ingest = useDashboard((s) => s.ingest);
  const nodes = useDashboard((s) => s.nodes);
  const locale = useSettings((s) => s.locale);
  const tr = t(locale);
  const [settingsOpen, setSettingsOpen] = useState(false);

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
          <h1 className="text-2xl font-bold tracking-tight">{tr.appTitle}</h1>
          <p className="text-sm text-zinc-400">{tr.liveCsi(connectedCount)}</p>
        </div>
        <div className="flex items-center gap-3">
          <button
            onClick={() => setSettingsOpen(true)}
            className="text-xs text-zinc-500 hover:text-zinc-300"
          >
            {tr.settings}
          </button>
          <a
            className="text-xs text-zinc-500 hover:text-zinc-300"
            href="https://github.com/vladimir120307-droid/wavesight"
            target="_blank"
            rel="noreferrer"
          >
            {tr.github}
          </a>
        </div>
      </header>

      {nodeList.length === 0 ? (
        <div className="panel text-zinc-400">
          <p className="font-medium">{tr.emptyTitle}</p>
          <ol className="list-decimal list-inside text-sm mt-2 space-y-1">
            {tr.emptySteps.map((step, i) => (
              <li key={i}>{step}</li>
            ))}
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
              locale={locale}
            />
          ))}
        </div>
      )}

      <footer className="text-xs text-zinc-500 pt-6">{tr.footer}</footer>

      {settingsOpen && <SettingsPanel onClose={() => setSettingsOpen(false)} />}
    </div>
  );
}
