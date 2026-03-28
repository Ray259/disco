import { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Terminal as TerminalIcon, Send, StopCircle, ExternalLink } from "lucide-react";

interface AgentOutput {
  text: string;
  is_error: boolean;
}

// Strip ONLY ANSI color codes, preserving icons and box-drawing chars
function stripAnsi(str: string): string {
  return str
    .replace(/\x1B\[[\x3C-\x3F]*[0-9;]*[A-Za-z]/g, "") 
    .replace(/\x1B\][^\x07]*\x07/g, "")               
    .replace(/\r/g, "");                             
}

const PROVIDERS = ["gemini", "codex"];

export function CrewTerminal() {
  const [provider, setProvider] = useState("gemini");
  const [sessionId, setSessionId] = useState<string | null>(null);
  const [rawOutput, setRawOutput] = useState("");
  const [input, setInput] = useState("");
  const [isRunning, setIsRunning] = useState(false);
  const [isThinking, setIsThinking] = useState(false);
  const outputEndRef = useRef<HTMLDivElement>(null);
  const providerRef = useRef(provider);

  useEffect(() => {
    providerRef.current = provider;
  }, [provider]);

  // Robust async listener setup with proper cleanup
  useEffect(() => {
    let active = true;
    let unlisteners: (() => void)[] = [];

    const setup = async () => {
      const events = ["claude-out", "gemini-out", "codex-out"];
      const fns = await Promise.all(events.map(ev => 
        listen<AgentOutput>(ev, (event) => {
          if (active && ev.startsWith(providerRef.current)) {
            setRawOutput(prev => prev + stripAnsi(event.payload.text));
            setIsThinking(false);
          }
        })
      ));
      
      if (!active) {
        fns.forEach(f => f());
      } else {
        unlisteners = fns;
      }
    };

    setup();
    return () => {
      active = false;
      unlisteners.forEach(f => f());
    };
  }, []);

  useEffect(() => {
    if (isRunning) handleStop();
    setRawOutput("");
  }, [provider]);

  useEffect(() => {
    outputEndRef.current?.scrollIntoView({ behavior: "smooth" });
  }, [rawOutput]);

  const handleStart = async () => {
    if (isRunning) return;
    try {
      const cmd = provider === "claude" ? "start_claude_session" : 
                  provider === "gemini" ? "start_gemini_session" : 
                  "start_codex_session";
      await invoke(cmd);
      setIsRunning(true);
      
      const getters: any = {
        claude: "get_claude_session_id",
        gemini: "get_gemini_session_id",
        codex: "get_codex_session_id"
      };
      const id = await invoke<string>(getters[provider]);
      setSessionId(id);

      setRawOutput(prev => prev + `>>> [SESSION STARTED: ${provider.toUpperCase()}]\n`);
    } catch (e) {
      setRawOutput(prev => prev + `!!! [SYSTEM FAILURE]: ${e}\n`);
    }
  };

  const handleStop = async () => {
    try {
      const cmd = provider === "claude" ? "stop_claude_session" : 
                  provider === "gemini" ? "stop_gemini_session" : 
                  "stop_codex_session";
      await invoke(cmd);
      setIsRunning(false);
      setSessionId(null);
      setRawOutput(prev => prev + `>>> [SESSION TERMINATED]\n`);
    } catch (e) {
      console.error(e);
    }
  };

  const handleSend = async () => {
    if (!input.trim()) return;
    if (!isRunning) await handleStart();

    try {
      setRawOutput(prev => prev + `> [YOU]: ${input}\n`);
      setIsThinking(true);
      const cmd = provider === "claude" ? "send_claude_input" : 
                  provider === "gemini" ? "send_gemini_input" : 
                  "send_codex_input";
      await invoke(cmd, { text: input });
      setInput("");
    } catch (e) {
      setRawOutput(prev => prev + `!!! [TRANSMISSION ERROR]: ${e}\n`);
    }
  };

  const handleResume = async () => {
    if (isRunning) return;
    try {
      setRawOutput(""); // Clear stale output before re-loading history
      const cmd = provider === "claude" ? "resume_claude_session" : 
                  provider === "gemini" ? "resume_gemini_session" : 
                  "resume_codex_session";
      await invoke(cmd);
      setIsRunning(true);
      setSessionId("latest");
      setRawOutput(prev => prev + `>>> [SESSION RE-ESTABLISHED: ${provider.toUpperCase()}]\n`);
    } catch (e) {
      setRawOutput(prev => prev + `!!! [RESUME FAILURE]: ${e}\n`);
    }
  };

  const handleOpenTerminal = async () => {
    try {
      const getters: any = {
        claude: "get_claude_session_id",
        gemini: "get_gemini_session_id",
        codex: "get_codex_session_id"
      };
      const sessionId = await invoke<string>(getters[provider]);
      await invoke("launch_terminal_session", { provider, sessionId });
    } catch (e) {
      setRawOutput(prev => prev + `!!! [LOGIC ERROR]: ${e}\n`);
    }
  };

  const parseInlineMarkdown = (text: string) => {
    // Basic regex for inline markdown - more robust pattern
    // Catches **bold**, `code`, [[links]] even with trailing punctuation
    const parts = text.split(/(\*\*[^*]+\*\*|`[^`]+`|\[\[[^\]]+\]\])/g);
    return parts.map((part, i) => {
      if (part.startsWith("**") && part.endsWith("**")) {
        return <strong key={i} className="font-bold text-white/90">{part.slice(2, -2)}</strong>;
      }
      if (part.startsWith("`") && part.endsWith("`")) {
        return <code key={i} className="bg-white/10 px-1 rounded font-mono text-[11px] text-[var(--disco-accent-teal)]">{part.slice(1, -1)}</code>;
      }
      if (part.startsWith("[[") && part.endsWith("]]")) {
        return <span key={i} className="text-[var(--disco-accent-teal)] italic border-b border-[var(--disco-accent-teal)]/20 cursor-default">{part.slice(2, -2)}</span>;
      }
      return part;
    });
  };

  const renderLines = () => {
    const lines = rawOutput.split("\n");
    return lines.map((line, i) => {
      const trimmed = line.trim();
      if (!trimmed && i < lines.length - 1) return <div key={i} className="h-2" />;
      
      let className = "text-[#d4d4d8] opacity-90 text-[13px] leading-snug block";
      let style: React.CSSProperties = { whiteSpace: "pre-wrap", wordBreak: "break-word" };
      let content = line;

      if (trimmed.startsWith("###")) {
        className = "text-[#efac55] font-bold text-xs uppercase tracking-widest mt-4 mb-2 block";
        content = trimmed.replace(/^#+\s*/, ""); // Strip the hashes
      } else if (trimmed.startsWith("- ")) {
        className = "text-[#d4d4d8] pl-4 opacity-80 block";
      } else if (trimmed.startsWith("✦")) {
        className = "text-[var(--disco-accent-teal)] italic block py-0.5";
      } else if (trimmed.startsWith("╭") || trimmed.startsWith("│") || trimmed.startsWith("╰") || trimmed.startsWith("✓")) {
        className = "font-mono text-[11px] leading-tight text-[var(--disco-accent-teal)] opacity-70 block";
      } else if (trimmed.startsWith("> [YOU]")) {
        className = "text-[#efac55] font-bold border-l-2 border-[#efac55]/30 pl-3 py-1 my-2 bg-white/5 block";
      } else if (trimmed.includes("!!!")) {
        className = "text-[#b32b1e] italic bg-black/20 p-2 rounded border border-[#b32b1e]/20 text-[11px] block";
      } else if (trimmed.startsWith(">>>")) {
        className = "text-[var(--disco-accent-yellow)]/50 text-[10px] uppercase tracking-tighter italic border-y border-white/5 py-1 block";
      }

      return (
        <div key={i} className={className} style={style}>
          {parseInlineMarkdown(content)}
        </div>
      );
    });
  };

  const renderThinkingIndicator = () => {
    if (!isThinking) return null;
    
    return (
      <div className="flex items-center gap-2 text-[var(--disco-accent-orange)] opacity-50 italic text-[11px] animate-pulse mt-4">
        <span className="w-2 h-2 rounded-full bg-[var(--disco-accent-orange)] animate-bounce" />
        <span>Conceptualizing the problem...</span>
      </div>
    );
  };

  return (
    <div 
      className="flex flex-col h-full overflow-hidden select-none" 
      style={{ 
        backgroundColor: "var(--disco-bg)", 
        fontFamily: "var(--font-body)", 
        color: "var(--disco-text-primary)" 
      }}
    >
      <div className="flex items-center justify-between p-3 border-b-2 border-white/5 bg-black/40 backdrop-blur-md z-10">
        <div className="flex items-center gap-3">
          <div className="p-1.5 rounded bg-[var(--disco-accent-orange)]/10 border border-[var(--disco-accent-orange)]/20">
            <TerminalIcon size={16} className="text-[var(--disco-accent-orange)]" />
          </div>
          <span className="uppercase tracking-[0.2em] text-[10px] font-bold text-[var(--disco-accent-orange)]">
            Agentic Crew
          </span>
        </div>
        
        <div className="flex items-center gap-3">
          <div className="flex bg-black/60 border border-white/10 rounded overflow-hidden">
            {PROVIDERS.map((p) => (
              <button
                key={p}
                onClick={() => setProvider(p)}
                className={`px-3 py-1 text-[9px] font-bold uppercase tracking-wider transition-all duration-200 ${
                  provider === p 
                    ? "bg-[var(--disco-accent-orange)] text-black" 
                    : "text-[var(--disco-text-secondary)] hover:text-white hover:bg-white/5"
                }`}
              >
                {p}
              </button>
            ))}
          </div>

          <div className="flex items-center gap-2">
            {isRunning && (
              <button 
                onClick={handleOpenTerminal}
                className="group flex items-center gap-2 px-2 py-1.5 bg-white/5 hover:bg-white/10 text-white border border-white/10 rounded transition-all text-[9px] font-bold uppercase tracking-widest"
              >
                <ExternalLink size={10} className="group-hover:scale-110 transition-transform" /> 
                <span>Terminal</span>
              </button>
            )}

            {isRunning && (
              <button 
                onClick={handleStop} 
                className="flex items-center gap-2 px-2 py-1.5 bg-[#b32b1e]/10 hover:bg-[#b32b1e]/20 text-[#b32b1e] border border-[#b32b1e]/20 rounded transition-all text-[9px] font-bold uppercase tracking-widest"
              >
                <StopCircle size={10} /> Sever
              </button>
            )}
          </div>
        </div>
      </div>

      <div className="flex-1 overflow-auto p-6 custom-scrollbar">
        <div className="max-w-4xl mx-auto">
          {rawOutput.length === 0 && !isRunning && (
            <div className="py-20 text-center space-y-4 opacity-50">
              <div className="text-[var(--disco-accent-yellow)] text-xl italic opacity-40">"The somewhat organic lines remind you of old filament memory units, but not quite? No. You're nowhere near right.
This delicate machine, ready for picking."</div>
              <div className="flex flex-col items-center gap-4">
                <div className="text-[9px] uppercase tracking-[0.4em] font-black animate-pulse text-[var(--disco-accent-orange)]">
                  [ TALK TO THE LIMBED AND HEADED MACHINE ]
                </div>
                <div className="text-[9px] text-white/20 uppercase tracking-widest font-bold">or</div>
                <button 
                  onClick={handleResume}
                  className="px-6 py-2 bg-white/5 hover:bg-[var(--disco-accent-teal)]/10 text-[var(--disco-accent-teal)] border border-[var(--disco-accent-teal)]/20 rounded transition-all text-[10px] font-bold uppercase tracking-[0.2em]"
                >
                  Retrieve the Damaged Ledger
                </button>
              </div>
            </div>
          )}
          
          <div className="space-y-0 text-left">
            {renderLines()}
            {renderThinkingIndicator()}
          </div>
          <div ref={outputEndRef} className="h-8" />
        </div>
      </div>

      <form 
        onSubmit={(e) => {
          e.preventDefault();
          handleSend();
        }}
        className="p-4 bg-black/20 border-t border-white/5 z-10"
      >
        <div className="max-w-4xl mx-auto flex items-center gap-3 bg-black/60 border border-white/10 rounded-lg p-1.5 pl-4 focus-within:border-[var(--disco-accent-orange)]/40 transition-all shadow-2xl">
          <input
            type="text"
            value={input}
            onChange={(e) => setInput(e.target.value)}
            placeholder={isRunning ? "You need to get your shit together." : "Wait for the limbed machine to fire up..."}
            className="flex-1 bg-transparent outline-none py-1.5 text-sm text-[var(--disco-text-primary)] placeholder-white/20 italic selection:bg-[var(--disco-accent-orange)]/30 transition-all"
          />
          <button 
            type="submit"
            className={`p-2 rounded transition-all ${
              input.trim() 
                ? "bg-[var(--disco-accent-orange)] text-black hover:scale-105 active:scale-95" 
                : "opacity-0 scale-90"
            }`}
          >
            <Send size={16} />
          </button>
        </div>
        <div className="max-w-4xl mx-auto mt-2 flex justify-between px-2">
            <span className="text-[8px] uppercase tracking-[0.2em] font-bold text-white/20 italic">
               Conceptualizing via {provider} network {sessionId && `(${sessionId.substring(0, 8)})`}
            </span>
            <span className="text-[8px] uppercase tracking-[0.2em] font-bold text-white/20 italic">
               Inland Empire
            </span>
        </div>
      </form>

      <style>{`
        .custom-scrollbar::-webkit-scrollbar { width: 4px; }
        .custom-scrollbar::-webkit-scrollbar-track { background: transparent; }
        .custom-scrollbar::-webkit-scrollbar-thumb { background: rgba(255,255,255,0.05); border-radius: 2px; }
        .custom-scrollbar::-webkit-scrollbar-thumb:hover { background: rgba(255,255,255,0.1); }
      `}</style>
    </div>
  );
}
