import { RichContentDisplay } from "./RichContentDisplay";
import { Institution, Event, Geo, Work, SchoolOfThought, Figure } from "../api";

const itemBase = "border border-transparent hover:border-[var(--c-border-light)] hover:bg-[var(--c-deep)] p-3 transition-all cursor-pointer group relative";

export const Renderers = {
  Figure: (item: Figure) => (
    <div className={itemBase} data-entity-type="Figure">
        <div className="absolute left-0 top-0 bottom-0 w-1 bg-[var(--theme-color)] opacity-0 group-hover:opacity-100 transition-opacity" />
        <div className="flex justify-between items-baseline mb-1">
            <h3 className="text-lg font-header text-[var(--disco-text-primary)] group-hover:text-white leading-none">
                {item.name}
            </h3>
        </div>
        <div className="text-sm font-body text-[var(--disco-text-secondary)] italic leading-tight line-clamp-2">
            <RichContentDisplay content={item.primary_role} /> — <RichContentDisplay content={item.primary_location} />
        </div>
    </div>
  ),

  Institution: (item: Institution) => (
    <div className={itemBase} data-entity-type="Institution">
      <h3 className="text-xl font-header text-[var(--theme-color)]">{item.name}</h3>
      {item.description && <div className="text-sm text-gray-400"><RichContentDisplay content={item.description} /></div>}
    </div>
  ),

  Event: (item: Event) => (
    <div className={`${itemBase} flex justify-between items-center`} data-entity-type="Event">
      <div>
        <h3 className="text-lg font-header text-white group-hover:text-[var(--theme-color)] transition-colors">{item.name}</h3>
        {item.description && <div className="text-xs text-gray-500 line-clamp-1"><RichContentDisplay content={item.description} /></div>}
      </div>
      <div className="text-xs font-mono text-[var(--c-muted)]">
        {item.date_range ? `${item.date_range.start} — ${item.date_range.end}` : "No Date"}
      </div>
    </div>
  ),

  Geo: (item: Geo) => (
    <div className={itemBase} data-entity-type="Geo">
      <div className="flex justify-between items-baseline">
        <h3 className="text-lg font-header text-[var(--theme-color)]">{item.name}</h3>
        {item.region && <span className="text-xs font-mono uppercase text-[var(--c-faint)]"><RichContentDisplay content={item.region} /></span>}
      </div>
      {item.description && <div className="text-sm text-gray-400 mt-1"><RichContentDisplay content={item.description} /></div>}
    </div>
  ),

  Work: (item: Work) => (
    <div className={itemBase} data-entity-type="Work">
      <h3 className="text-lg font-serif italic text-white group-hover:text-[var(--theme-color)] transition-colors">"{item.title}"</h3>
      {item.summary && <div className="text-sm text-gray-500 mt-1 line-clamp-2"><RichContentDisplay content={item.summary} /></div>}
    </div>
  ),

  School: (item: SchoolOfThought) => (
    <div className={itemBase} data-entity-type="SchoolOfThought">
      <h3 className="text-xl font-header uppercase text-[var(--theme-color)]">{item.name}</h3>
      {item.description && <div className="text-sm text-gray-400 mt-1 border-l-2 border-[var(--c-border)] pl-2"><RichContentDisplay content={item.description} /></div>}
    </div>
  )
};
