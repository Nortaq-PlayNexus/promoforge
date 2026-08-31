import type { Page } from "../App";
import {
  LayoutDashboard,
  FolderOpen,
  Target,
  Image,
  Settings,
  Zap,
} from "lucide-react";

interface SidebarProps {
  currentPage: Page;
  onNavigate: (page: Page) => void;
  selectedProject: string | null;
}

const navItems: { page: Page; label: string; icon: typeof LayoutDashboard }[] =
  [
    { page: "dashboard", label: "Dashboard", icon: LayoutDashboard },
    { page: "projects", label: "Projects", icon: FolderOpen },
    { page: "strategy", label: "Strategy", icon: Target },
    { page: "assets", label: "Assets", icon: Image },
    { page: "settings", label: "Settings", icon: Settings },
  ];

export function Sidebar({ currentPage, onNavigate }: SidebarProps) {
  return (
    <aside className="w-64 bg-surface-1 border-r border-surface-3 flex flex-col h-screen">
      {/* Logo */}
      <div className="p-4 border-b border-surface-3">
        <div className="flex items-center gap-2">
          <div className="w-8 h-8 bg-brand-600 rounded-lg flex items-center justify-center">
            <Zap className="w-5 h-5 text-white" />
          </div>
          <div>
            <h1 className="text-base font-bold text-white">PromoForge</h1>
            <p className="text-[10px] text-surface-5 -mt-0.5">AI</p>
          </div>
        </div>
      </div>

      {/* Navigation */}
      <nav className="flex-1 p-3 space-y-1">
        {navItems.map(({ page, label, icon: Icon }) => (
          <button
            key={page}
            onClick={() => onNavigate(page)}
            className={`w-full flex items-center gap-3 px-3 py-2.5 rounded-lg text-sm font-medium transition-colors ${
              currentPage === page
                ? "bg-brand-600/20 text-brand-400 border border-brand-600/30"
                : "text-surface-5 hover:text-white hover:bg-surface-3 border border-transparent"
            }`}
          >
            <Icon className="w-4 h-4" />
            {label}
          </button>
        ))}
      </nav>

      {/* Footer */}
      <div className="p-4 border-t border-surface-3">
        <p className="text-[10px] text-surface-5 text-center">
          PromoForge AI v0.1.0
        </p>
      </div>
    </aside>
  );
}
