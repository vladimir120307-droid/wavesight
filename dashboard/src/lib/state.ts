import { create } from "zustand";
import type { StreamUpdate } from "./stream";

interface NodeState {
  node: string;
  lastUpdate: StreamUpdate;
  history: StreamUpdate[];
}

interface DashboardState {
  nodes: Record<string, NodeState>;
  ingest(update: StreamUpdate): void;
}

const MAX_HISTORY = 128;

export const useDashboard = create<DashboardState>((set) => ({
  nodes: {},
  ingest(update) {
    set((s) => {
      const prev = s.nodes[update.node];
      const history = prev ? [...prev.history, update].slice(-MAX_HISTORY) : [update];
      return {
        nodes: {
          ...s.nodes,
          [update.node]: { node: update.node, lastUpdate: update, history },
        },
      };
    });
  },
}));
