interface SidebarProps {
  currentView: string;
  onChangeView: (view: string) => void;
}

export function Sidebar({ currentView, onChangeView }: SidebarProps) {
  const menuItems = [
    { id: "figures", label: "Inhabitants" },
    { id: "institutions", label: "Institutions" },
    { id: "events", label: "History" },
    { id: "geos", label: "Geography" },
    { id: "works", label: "Bibliography" },
    { id: "schools", label: "Schools of Thought" },
  ];

  return (
    <div className="w-64 h-full bg-[#111] border-r border-[#333] flex flex-col">
      <div className="p-6 border-b border-[#333]">
         <h2 className="text-xl font-[var(--font-header)] text-white tracking-widest uppercase">
           Index
         </h2>
      </div>
      
      <nav className="flex-1 overflow-y-auto py-4">
        <ul className="space-y-1">
          {menuItems.map((item) => (
            <li key={item.id}>
              <button
                onClick={() => onChangeView(item.id)}
                className={`w-full text-left px-6 py-3 text-sm font-[var(--font-mono)] uppercase tracking-wider transition-colors
                  ${currentView === item.id 
                    ? "text-white bg-[#222] border-r-2 border-[var(--disco-accent-orange)]" 
                    : "text-[#666] hover:text-[#ccc] hover:bg-[#1a1a1a]"
                  }`}
              >
                {item.label}
              </button>
            </li>
          ))}
        </ul>
      </nav>

      <div className="p-6 border-t border-[#333] text-[10px] text-[#444] font-[var(--font-mono)]">
        DISCO ELYSIUM<br/>
        ENCYCLOPEDIA PROJECT
      </div>
    </div>
  );
}
