import { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { Terminal as TerminalIcon, Send, StopCircle, Play } from "lucide-react";

interface ClaudeOutput {
  text: string;
  is_error: boolean;
}

// Comprehensive ANSI escape sequence remover (covers VT100/VT220, xterm, etc.)
function stripAnsi(str: string): string {
  return str
    // CSI sequences: optional private prefix (<=>?), optional params, letter
    .replace(/\x1B\[[\x3C-\x3F]*[0-9;]*[A-Za-z]/g, "")
    .replace(/\x1B\][^\x07]*\x07/g, "")           // OSC + BEL
    .replace(/\x1B\][^\x1B]*\x1B\\/g, "")          // OSC + ST
    .replace(/\x1B[PX^_][^\x1B]*\x1B\\/g, "")      // DCS/SOS/PM/APC
    .replace(/\x1B[^[\]PX^_\u0000-\u001F]/g, "")   // ESC + single char
    .replace(/[\x00-\x09\x0B\x0C\x0E-\x1F\x7F]/g, "") // control chars
    .replace(/\r/g, "");
}

export function CrewTerminal() {
  const [provider, setProvider] = useState<"claude" | "gemini">("claude");
  const [output, setOutput] = useState<string[]>([]);
  const [input, setInput] = useState("");
  const [isRunning, setIsRunning] = useState(false);
  const scrollRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    // Listen to the correct event stream depending on provider
    const eventName = provider === "claude" ? "claude-out" : "gemini-out";
    const unlisten = listen<ClaudeOutput>(eventName, (event) => {
      const cleanText = stripAnsi(event.payload.text);
      setOutput((prev) => [...prev, cleanText]);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, [provider]);

  // Optionally stop session when switching providers to avoid overlapping states
  useEffect(() => {
    if (isRunning) {
      handleStop();
    }
    // clear output on switch
    setOutput([]);
  }, [provider]);

  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, [output]);

  const handleStart = async () => {
    try {
      const cmd = provider === "claude" ? "start_claude_session" : "start_gemini_session";
      await invoke(cmd);
      setIsRunning(true);
      setOutput((prev) => [...prev, `>>> ${provider === "claude" ? "Claude" : "Gemini"} session started.\n`]);
    } catch (e) {
      setOutput((prev) => [...prev, `!!! Error: ${e}\n`]);
    }
  };

  const handleStop = async () => {
    try {
      const cmd = provider === "claude" ? "stop_claude_session" : "stop_gemini_session";
      await invoke(cmd);
      setIsRunning(false);
      setOutput((prev) => [...prev, `>>> ${provider === "claude" ? "Claude" : "Gemini"} session stopped.\n`]);
    } catch (e) {
      setOutput((prev) => [...prev, `!!! Error: ${e}\n`]);
    }
  };

  const handleSend = async () => {
    if (!input.trim() || !isRunning) return;
    try {
      setOutput((prev) => [...prev, `> ${input}\n`]);
      const cmd = provider === "claude" ? "send_claude_input" : "send_gemini_input";
      await invoke(cmd, { text: input });
      setInput("");
    } catch (e) {
      setOutput((prev) => [...prev, `!!! Error: ${e}\n`]);
    }
  };

  return (
    <div className="flex flex-col h-full bg-[#050505] font-mono text-sm text-[#00ff00]">
      <div className="flex items-center justify-between p-4 border-b border-[#333] bg-[#111]">
        <div className="flex items-center gap-2">
          <TerminalIcon size={18} />
          <span className="uppercase tracking-widest text-xs font-bold text-[#aaa]">Crew</span>
        </div>
        <div className="flex items-center gap-4">
          <div className="flex bg-black border border-[#333] rounded p-1">
            <button 
              disabled={isRunning}
              onClick={() => setProvider("claude")}
              className={`px-3 py-1 rounded text-xs font-bold uppercase transition-colors ${provider === "claude" ? "bg-[#333] text-white" : "text-[#777] hover:text-[#aaa]"} ${isRunning && "opacity-50 cursor-not-allowed"}`}
            >
              Claude
            </button>
            <button 
              disabled={isRunning}
              onClick={() => setProvider("gemini")}
              className={`px-3 py-1 rounded text-xs font-bold uppercase transition-colors ${provider === "gemini" ? "bg-[#333] text-white" : "text-[#777] hover:text-[#aaa]"} ${isRunning && "opacity-50 cursor-not-allowed"}`}
            >
              Gemini
            </button>
          </div>
          {!isRunning ? (
            <button onClick={handleStart} className="flex items-center gap-1 px-3 py-1 bg-[#1a3a1a] hover:bg-[#2a4a2a] text-[#00ff00] rounded transition-colors uppercase text-[10px] font-bold">
              <Play size={12} /> Start Session
            </button>
          ) : (
            <button onClick={handleStop} className="flex items-center gap-1 px-3 py-1 bg-[#3a1a1a] hover:bg-[#4a2a2a] text-[#ff4444] rounded transition-colors uppercase text-[10px] font-bold">
              <StopCircle size={12} /> Stop Session
            </button>
          )}
        </div>
      </div>

      <div ref={scrollRef} className="flex-1 overflow-y-auto p-4 space-y-1">
        {output.map((line, i) => (
          <div key={i} className="whitespace-pre-wrap leading-relaxed opacity-90 animate-in fade-in slide-in-from-bottom-1 duration-300">
            {line}
          </div>
        ))}
      </div>

      <div className="p-4 border-t border-[#333] bg-[#111] flex gap-2">
        <input
          type="text"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={(e) => e.key === "Enter" && handleSend()}
          placeholder={isRunning ? "Speak to the Crew..." : "Start session to speak..."}
          disabled={!isRunning}
          className="flex-1 bg-black border border-[#333] px-4 py-2 rounded text-[#00ff00] placeholder-[#333] focus:outline-none focus:border-[#444] transition-colors"
        />
        <button
          onClick={handleSend}
          disabled={!isRunning || !input.trim()}
          className={`p-2 rounded transition-colors ${isRunning && input.trim() ? "bg-[#222] text-[#00ff00] hover:bg-[#333]" : "text-[#222] cursor-not-allowed"}`}
        >
          <Send size={20} />
        </button>
      </div>
    </div>
  );
}
