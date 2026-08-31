import { create } from "zustand";
import type { Project, ProductKnowledgeBase, Strategy } from "./types";

interface AppState {
  projects: Project[];
  selectedProject: Project | null;
  pkb: ProductKnowledgeBase | null;
  strategy: Strategy | null;
  loading: boolean;
  error: string | null;

  setProjects: (projects: Project[]) => void;
  setSelectedProject: (project: Project | null) => void;
  setPkb: (pkb: ProductKnowledgeBase | null) => void;
  setStrategy: (strategy: Strategy | null) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
}

export const useAppStore = create<AppState>((set) => ({
  projects: [],
  selectedProject: null,
  pkb: null,
  strategy: null,
  loading: false,
  error: null,

  setProjects: (projects) => set({ projects }),
  setSelectedProject: (project) =>
    set({ selectedProject: project, pkb: null, strategy: null }),
  setPkb: (pkb) => set({ pkb }),
  setStrategy: (strategy) => set({ strategy }),
  setLoading: (loading) => set({ loading }),
  setError: (error) => set({ error }),
}));
