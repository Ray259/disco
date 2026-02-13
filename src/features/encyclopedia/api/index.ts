import { invoke } from "@tauri-apps/api/core";

// Matching Rust DateRange (NaiveDate strings "YYYY-MM-DD")
export interface DateRange {
  start: string;
  end: string;
}

// Matching Rust EntityRef
export interface EntityRef {
  entity_type: string;
  entity_id: string; // Uuid
  display_text: string;
}

// Matching Rust RichContent
export interface RichContent {
  segments: ContentSegment[];
}

export type ContentSegment = 
  | { Text: string }
  | { EntityRef: EntityRef }
  | { DateRef: DateRange };

// Matching Rust Figure
export interface Figure {
  id: string; // Uuid
  name: string;
  life: DateRange;
  primary_role: RichContent;
  primary_location: RichContent;
  defining_quote?: RichContent;
  // Simplified for now, can add more fields as we build UI for them
  // zeitgeist?: Zeitgeist;
  // ... other fields
  created_at: string;
  updated_at: string;
}

export async function getAllFigures(): Promise<Figure[]> {
  return await invoke("get_all_figures");
}

export async function getFigure(id: string): Promise<Figure | null> {
  return await invoke("get_figure", { id });
}

export interface CreateFigureRequest {
  name: string;
  role: string;
  location: string;
  start_year: string;
  end_year: string;
  quote?: string;
}

export async function createFigure(request: CreateFigureRequest): Promise<string> {
  return await invoke("create_figure", { request });
}

export interface CreateInstitutionRequest {
  name: string;
  founded_start?: string;
  founded_end?: string;
  description?: string;
}

export async function createInstitution(request: CreateInstitutionRequest): Promise<string> {
  return await invoke("create_institution", { request });
}

export interface CreateEventRequest {
  name: string;
  start_date: string;
  end_date: string;
  description?: string;
}

export async function createEvent(request: CreateEventRequest): Promise<string> {
  return await invoke("create_event", { request });
}

export interface CreateGeoRequest {
  name: string;
  region?: string;
  description?: string;
}

export async function createGeo(request: CreateGeoRequest): Promise<string> {
  return await invoke("create_geo", { request });
}

export interface CreateWorkRequest {
  title: string;
  summary?: string;
}

export async function createWork(request: CreateWorkRequest): Promise<string> {
  return await invoke("create_work", { request });
}

export interface CreateSchoolOfThoughtRequest {
  name: string;
  description?: string;
}

export async function createSchoolOfThought(request: CreateSchoolOfThoughtRequest): Promise<string> {
  return await invoke("create_school_of_thought", { request });
}
