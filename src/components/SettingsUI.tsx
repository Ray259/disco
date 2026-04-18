import React, { useState, useEffect } from "react";
import { X } from "lucide-react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";

interface SettingsUIProps {
  volume: number;
  isMuted: boolean;
  onVolumeChange: (volume: number) => void;
  onMuteToggle: () => void;
  onClose: () => void;
}

export const SettingsUI: React.FC<SettingsUIProps> = ({
  volume,
  isMuted,
  onVolumeChange,
  onMuteToggle,
  onClose,
}) => {
  const [vaultPath, setVaultPath] = useState<string>("Loading...");

  useEffect(() => {
    const fetchVaultPath = async () => {
      try {
        const path = await invoke<string>("get_vault_path");
        setVaultPath(path || "No Vault Selected");
      } catch (err) {
        console.error("Failed to get vault path", err);
        setVaultPath("Error loading path");
      }
    };
    fetchVaultPath();
  }, []);

  const handleChangeVault = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Select Vault Folder",
      });
      
      if (selected && typeof selected === 'string') {
        setVaultPath("Updating...");
        await invoke("set_vault_path", { newPath: selected });
        // Fetch it again to confirm it was set
        const updatedPath = await invoke<string>("get_vault_path");
        setVaultPath(updatedPath || "No Vault Selected");
      }
    } catch (err) {
      console.error("Failed to change vault path", err);
      // fallback to refetching the old path
      const path = await invoke<string>("get_vault_path").catch(() => "Error");
      setVaultPath(path || "No Vault Selected");
    }
  };

  return (
    <div className="fixed inset-0 z-[100] flex items-center justify-center bg-black/60 backdrop-blur-sm font-header">
        <style dangerouslySetInnerHTML={{__html: `
          input[type=range]::-webkit-slider-thumb {
            -webkit-appearance: none;
            height: 16px; width: 16px; border-radius: 50%;
            background: #f5f5f5; cursor: pointer;
            border: 3px solid var(--c-dim);
            box-shadow: 0 0 4px rgba(0,0,0,0.8);
            margin-top: -6px;
          }
          input[type=range]::-webkit-slider-runnable-track {
            width: 100%; height: 2px; cursor: pointer;
            background: var(--c-subtle);
          }
        `}} />

      <div className="flex bg-[var(--c-panel)]/95 border-2 border-[#111] shadow-2xl relative max-w-2xl w-full">
        
        <button 
           onClick={onClose}
           aria-label="Close settings"
           className="absolute top-4 right-4 text-[var(--c-muted)] hover:text-white focus-visible:ring-2 focus-visible:ring-white focus-visible:outline-none transition-colors z-50 p-1 rounded"
        >
           <X size={24} />
        </button>

        <div className="w-[120px] bg-gradient-to-br from-[#c45511] via-[#94380b] to-[#752605] relative flex flex-col justify-center border-r-[3px] border-black/80 z-20">
          <div className="absolute inset-0 opacity-10 mix-blend-overlay pointer-events-none"
            style={{ backgroundImage: `url("data:image/svg+xml,%3Csvg viewBox='0 0 200 200' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='noiseFilter'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.85' numOctaves='3' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23noiseFilter)'/%3E%3C/svg%3E")` }}
          />
          <div className="absolute left-0 top-0 bottom-0 w-8 bg-black/40 border-r border-black/60 flex flex-col justify-between py-6 items-center text-[var(--c-subtle)] text-[10px]">
             <span className="transform -rotate-90 origin-center whitespace-nowrap tracking-widest opacity-50">01A13</span>
             <span className="transform -rotate-90 origin-center whitespace-nowrap tracking-widest opacity-50">2021-04-19</span>
          </div>
        </div>

        <div className="flex-1 flex flex-col items-center py-8">
            <div className="flex w-full px-8 mb-8 uppercase text-2xl tracking-wider text-[var(--c-muted)]">
                <div className="flex-1 bg-[#f5f5f5] text-black text-center py-1 shadow-md">
                    Settings
                </div>
            </div>

            <div className="w-full px-8">
                <table className="w-full uppercase text-base tracking-wider text-[var(--c-subtle)]">
                  <tbody>
                    <tr>
                      <td className="w-[45%] text-right py-3 pr-4 uppercase text-[#dedede]">Music Volume</td>
                      <td className="w-[55%] text-left py-3 pl-4 flex items-center text-[#dedede]">
                        <span className="mr-2 text-xl font-sans text-[var(--c-muted)] leading-none">(</span>
                        <input 
                            type="range" 
                            aria-label="Music Volume"
                            min="0" max="1" step="0.01" 
                            value={volume}
                            onChange={(e) => onVolumeChange(parseFloat(e.target.value))}
                            className="w-full max-w-[150px] appearance-none bg-transparent outline-none focus-visible:ring-2 focus-visible:ring-white focus-visible:outline-none rounded m-0 p-0"
                        />
                        <span className="ml-2 text-xl font-sans text-[var(--c-muted)] leading-none">)</span>
                      </td>
                    </tr>

                    <tr>
                      <td className="w-[45%] text-right py-3 pr-4 uppercase text-[#dedede]">Mute All Audio</td>
                      <td className="w-[55%] text-left py-3 pl-4 flex items-center text-[#dedede]">
                         <button 
                            onClick={onMuteToggle}
                            aria-label={isMuted ? "Unmute audio" : "Mute audio"}
                            className="w-5 h-5 flex items-center justify-center bg-white hover:bg-white/80 focus-visible:ring-2 focus-visible:ring-white focus-visible:outline-none transition-colors rounded"
                         >
                            {!isMuted && <div className="w-full h-full bg-[#111] border border-white rounded-sm" />}
                         </button>
                      </td>
                    </tr>
                    <tr>
                      <td className="w-[45%] text-right py-3 pr-4 uppercase text-[#dedede]">Vault Location</td>
                      <td className="w-[55%] text-left py-3 pl-4 flex flex-col justify-center text-[#dedede]">
                         <div className="flex items-center space-x-3">
                             <button 
                                onClick={handleChangeVault}
                                aria-label="Change Vault Location"
                                className="px-3 py-1 bg-white/10 hover:bg-white/20 border border-white/20 focus-visible:ring-2 focus-visible:ring-white focus-visible:outline-none text-xs transition-colors rounded"
                             >
                                CHANGE
                             </button>
                             <div className="text-xs font-mono lowercase truncate max-w-[200px] opacity-70" title={vaultPath}>
                                {vaultPath}
                             </div>
                         </div>
                      </td>
                    </tr>
                  </tbody>
                </table>
            </div>
            
            <div className="mt-8 text-center text-[var(--c-muted)] text-xs font-mono italic px-8">
               "The static of Revachol hums beneath the music."
            </div>
        </div>
      </div>
    </div>
  );
};
