import { useState } from "react";
import { Sidebar } from "./components/Sidebar";
import { Dashboard } from "./pages/Dashboard";
import { Projects } from "./pages/Projects";
import { StrategyView } from "./pages/StrategyView";
import { Assets } from "./pages/Assets";
import { Settings } from "./pages/Settings";

export type Page =
  | "dashboard"
  | "projects"
  | "strategy"
  | "assets"
  | "settings";

export default function App() {
  const [page, setPage] = useState<Page>("dashboard");
  const [selectedProject, setSelectedProject] = useState<string | null>(null);

  const renderPage = () => {
    switch (page) {
      case "dashboard":
        return (
          <Dashboard
            selectedProject={selectedProject}
            onSelectProject={setSelectedProject}
            onNavigate={setPage}
          />
        );
      case "projects":
        return (
          <Projects
            selectedProject={selectedProject}
            onSelectProject={setSelectedProject}
          />
        );
      case "strategy":
        return (
          <StrategyView
            selectedProject={selectedProject}
            onNavigate={setPage}
          />
        );
      case "assets":
        return <Assets selectedProject={selectedProject} />;
      case "settings":
        return <Settings />;
      default:
        return <Dashboard selectedProject={selectedProject} onSelectProject={setSelectedProject} onNavigate={setPage} />;
    }
  };

  return (
    <div className="flex h-screen overflow-hidden">
      <Sidebar
        currentPage={page}
        onNavigate={setPage}
        selectedProject={selectedProject}
      />
      <main className="flex-1 overflow-y-auto bg-surface-0">
        {renderPage()}
      </main>
    </div>
  );
}
