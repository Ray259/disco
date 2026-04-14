import { Settings as SettingsIcon } from "lucide-react";

interface SidebarProps {
  currentView: string;
  onChangeView: (view: string) => void;
  onOpenSettings: () => void;
}

export function Sidebar({ currentView, onChangeView, onOpenSettings }: SidebarProps) {
  const menuItems = [
    { id: "figures", label: "Inhabitants" },
    { id: "institutions", label: "Institutions" },
    { id: "events", label: "History" },
    { id: "geos", label: "Geography" },
    { id: "works", label: "Bibliography" },
    { id: "schools", label: "Schools of Thought" },
    { id: "crew", label: "Agentic Crew" },
  ];

  return (
    <div className="w-64 h-full bg-[#111]/80 backdrop-blur-sm border-r border-[var(--c-border)] flex flex-col">
      <div className="p-6 border-b border-[var(--c-border)]">
         <h2 className="text-xl font-header text-white tracking-widest uppercase">
           Index
         </h2>
      </div>
      
      <nav className="flex-1 overflow-y-auto py-4">
        <ul className="space-y-1">
          {menuItems.map((item) => (
            <li key={item.id}>
              <button
                onClick={() => onChangeView(item.id)}
                data-active={currentView === item.id}
                data-view-id={item.id}
                className={`sidebar-nav-button w-full text-left px-6 py-3 text-sm font-mono uppercase tracking-wider transition-colors
                  ${currentView === item.id 
                    ? "bg-[var(--c-deep)] border-r-2" 
                    : "text-[var(--c-muted)] hover:text-[#ccc] hover:bg-[var(--c-panel)]"
                  }`}
              >
                {item.label}
              </button>
            </li>
          ))}
        </ul>
      </nav>

      <div className="p-6 border-t border-[var(--c-border)] flex justify-between items-center">
        <div className="text-[10px] text-[var(--c-ghost)] font-mono">
          DISCO ELYSIUM<br/>
          ENCYCLOPEDIA PROJECT
        </div>
        <button 
          onClick={onOpenSettings}
          aria-label="Open settings"
          className="p-2 text-[var(--c-muted)] hover:text-[#bfa275] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-[#bfa275] rounded transition-colors"
          title="Settings"
        >
          <SettingsIcon size={20} />
        </button>
      </div>
    </div>
  );
}
