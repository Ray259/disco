import React from "react";
import { X, Volume2, VolumeX } from "lucide-react"; // Make sure lucide-react is installed

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
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4">
      {/* Container echoing the painterly, high-contrast look */}
      <div 
        className="relative w-full max-w-md bg-[#1d1b19] border-2 border-[#bfa275] shadow-2xl overflow-hidden"
        style={{
           // Subtle noise/texture effect often seen in the game UI
           backgroundImage: `url("data:image/svg+xml,%3Csvg viewBox='0 0 200 200' xmlns='http://www.w3.org/2000/svg'%3E%3Cfilter id='noiseFilter'%3E%3CfeTurbulence type='fractalNoise' baseFrequency='0.85' numOctaves='3' stitchTiles='stitch'/%3E%3C/filter%3E%3Crect width='100%25' height='100%25' filter='url(%23noiseFilter)' opacity='0.05'/%3E%3C/svg%3E")`
        }}
      >
        {/* Header Ribbon */}
        <div className="flex justify-between items-center bg-gradient-to-r from-[#591410] to-[#8c2a1c] p-3 border-b border-[#bfa275]">
           {/* Dobra-like font application (using a solid sans-serif fallback if font not loaded) */}
          <h2 className="text-[#f2e6d8] font-bold text-xl tracking-wider uppercase" style={{ fontFamily: "'Dobra Black', 'Arial Black', sans-serif" }}>
            Settings
          </h2>
          <button 
            onClick={onClose} 
            className="text-[#f2e6d8] hover:text-[#ffcca5] transition-colors p-1"
          >
            <X size={24} />
          </button>
        </div>

        {/* Content Body (Sina Nova / Serif aesthetic) */}
        <div className="p-6 text-[#d9cfc1]" style={{ fontFamily: "'Sina Nova', 'Libre Baskerville', 'Georgia', serif" }}>
          
          <div className="space-y-8">
            {/* Audio Settings Section */}
            <div>
              <h3 className="text-lg text-[#bfa275] border-b border-[#3b352e] pb-1 mb-4 uppercase tracking-widest font-sans font-semibold">
                Audio Options
              </h3>
              
              <div className="flex flex-col gap-6 pl-2">
                
                {/* Volume Slider */}
                <div className="flex flex-col gap-2">
                  <div className="flex justify-between">
                    <label htmlFor="volume-slider" className="opacity-90 tracking-wide">
                      Master Volume
                    </label>
                    <span className="text-[#bfa275] font-sans font-bold">
                      {Math.round(volume * 100)}%
                    </span>
                  </div>
                  
                  <input
                    id="volume-slider"
                    type="range"
                    min="0"
                    max="1"
                    step="0.01"
                    value={volume}
                    onChange={(e) => onVolumeChange(parseFloat(e.target.value))}
                    className="w-full h-2 rounded-none appearance-none cursor-pointer bg-[#3b352e] border border-[#594d3f] accent-[#bfa275] hover:accent-[#ffcca5] transition-all"
                  />
                  {/* Note: Native range inputs are hard to style fully cross-browser, but basic accent-color helps */}
                </div>

                {/* Mute Toggle */}
                <div className="flex items-center justify-between mt-2">
                  <span className="opacity-90 tracking-wide">Mute Audio</span>
                  <button
                    onClick={onMuteToggle}
                    className={`flex items-center justify-center p-2 border-2 transition-all ${
                      isMuted ? "bg-[#8c2a1c] border-[#bfa275] shadow-[0_0_10px_rgba(140,42,28,0.5)]" : "bg-[#2a2622] border-[#594d3f] hover:border-[#bfa275]"
                    }`}
                  >
                    {isMuted ? (
                      <VolumeX size={20} className="text-[#f2e6d8]" />
                    ) : (
                      <Volume2 size={20} className="text-[#bfa275]" />
                    )}
                  </button>
                </div>
              </div>
            </div>
            
            {/* Disclaimer / Lore flavor */}
            <div className="mt-8 pt-4 border-t border-[#3b352e] text-center opacity-60 italic text-sm">
               "The static of Revachol hums beneath the music."
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
