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

export interface RelationDto {
  target_id: string; // Uuid
  relation_type: string; // "FOUNDER_OF", etc.
}

export interface CreateFigureRequest {
  name: string;
  role: string;
  location: string;
  start_year: string;
  end_year: string;
  quote?: string;
  relations?: RelationDto[];
}

export async function createFigure(request: CreateFigureRequest): Promise<string> {
  return await invoke("create_figure", { request });
}

export interface CreateInstitutionRequest {
  name: string;
  founded_start?: string;
  founded_end?: string;
  description?: string;
  relations?: RelationDto[];
}

export async function createInstitution(request: CreateInstitutionRequest): Promise<string> {
  return await invoke("create_institution", { request });
}

export interface CreateEventRequest {
  name: string;
  start_date: string;
  end_date: string;
  description?: string;
  relations?: RelationDto[];
}

export async function createEvent(request: CreateEventRequest): Promise<string> {
  return await invoke("create_event", { request });
}

export interface CreateGeoRequest {
  name: string;
  region?: string;
  description?: string;
  relations?: RelationDto[];
}

export async function createGeo(request: CreateGeoRequest): Promise<string> {
  return await invoke("create_geo", { request });
}

export interface CreateWorkRequest {
  title: string;
  summary?: string;
  relations?: RelationDto[];
}

export async function createWork(request: CreateWorkRequest): Promise<string> {
  return await invoke("create_work", { request });
}

export interface CreateSchoolOfThoughtRequest {
  name: string;
  description?: string;
  relations?: RelationDto[];
}

export async function createSchoolOfThought(request: CreateSchoolOfThoughtRequest): Promise<string> {
  return await invoke("create_school_of_thought", { request });
}

export interface SearchResult {
  id: string;
  entity_type: string;
  name: string;
  description?: string;
}

export async function searchEntities(query: string): Promise<SearchResult[]> {
  return await invoke("search_entities", { query });
}

export interface Institution {
  id: string;
  name: string;
  description?: RichContent;
  founded?: DateRange;
  // ...
}

export async function getAllInstitutions(): Promise<Institution[]> {
  return await invoke("get_all_institutions");
}

export interface Event {
  id: string;
  name: string;
  description?: RichContent;
  date_range: DateRange;
}

export async function getAllEvents(): Promise<Event[]> {
  return await invoke("get_all_events");
}

export interface Geo {
  id: string;
  name: string;
  region?: RichContent;
  description?: RichContent;
}

export async function getAllGeos(): Promise<Geo[]> {
  return await invoke("get_all_geos");
}

export interface Work {
  id: string;
  title: string;
  summary?: RichContent;
}

export async function getAllWorks(): Promise<Work[]> {
  return await invoke("get_all_works");
}

export interface SchoolOfThought {
  id: string;
  name: string;
  description?: RichContent;
}

export async function getAllSchoolsOfThought(): Promise<SchoolOfThought[]> {
  return await invoke("get_all_schools_of_thought");
}
