import { invoke } from "@tauri-apps/api/core";

export interface DateRange {
  start: string;
  end: string;
}

export interface EntityRef {
  entity_type: string;
  display_text: string;
}

export interface RichContent {
  segments: ContentSegment[];
}

export type ContentSegment =
  | { Text: string }
  | { EntityRef: EntityRef }
  | { DateRef: DateRange };

export interface Figure {
  name: string;
  life: DateRange;
  primary_role: RichContent;
  primary_location: RichContent;
  defining_quote?: RichContent;
  created_at: string;
  updated_at: string;
}

export async function getAllFigures(): Promise<Figure[]> {
  return await invoke("get_all_figures");
}

export async function getFigure(name: string): Promise<Figure | null> {
  return await invoke("get_figure", { name });
}

export interface CreateFigureRequest {
  name: string;
  role: RichContent;
  location: RichContent;
  start_year: string;
  end_year: string;
  quote?: RichContent;
}

export async function createFigure(request: CreateFigureRequest): Promise<string> {
  return await invoke("create_figure", { request });
}

export async function updateFigure(name: string, request: CreateFigureRequest): Promise<string> {
  return await invoke("update_figure", { name, request });
}

// --- Institution ---

export interface CreateInstitutionRequest {
  name: string;
  founded_start?: string;
  founded_end?: string;
  description?: RichContent;
}

export interface Institution {
  name: string;
  description?: RichContent;
  founded?: DateRange;
}

export async function getAllInstitutions(): Promise<Institution[]> {
  return await invoke("get_all_institutions");
}

export async function getInstitution(name: string): Promise<Institution | null> {
  return await invoke("get_institution", { name });
}

export async function createInstitution(request: CreateInstitutionRequest): Promise<string> {
  return await invoke("create_institution", { request });
}

export async function updateInstitution(name: string, request: CreateInstitutionRequest): Promise<string> {
  return await invoke("update_institution", { name, request });
}

// --- Event ---

export interface CreateEventRequest {
  name: string;
  start_date: string;
  end_date: string;
  description?: RichContent;
}

export interface Event {
  name: string;
  description?: RichContent;
  date_range: DateRange;
}

export async function getAllEvents(): Promise<Event[]> {
  return await invoke("get_all_events");
}

export async function getEvent(name: string): Promise<Event | null> {
  return await invoke("get_event", { name });
}

export async function createEvent(request: CreateEventRequest): Promise<string> {
  return await invoke("create_event", { request });
}

export async function updateEvent(name: string, request: CreateEventRequest): Promise<string> {
  return await invoke("update_event", { name, request });
}

// --- Geo ---

export interface CreateGeoRequest {
  name: string;
  region?: RichContent;
  description?: RichContent;
}

export interface Geo {
  name: string;
  region?: RichContent;
  description?: RichContent;
}

export async function getAllGeos(): Promise<Geo[]> {
  return await invoke("get_all_geos");
}

export async function getGeo(name: string): Promise<Geo | null> {
  return await invoke("get_geo", { name });
}

export async function createGeo(request: CreateGeoRequest): Promise<string> {
  return await invoke("create_geo", { request });
}

export async function updateGeo(name: string, request: CreateGeoRequest): Promise<string> {
  return await invoke("update_geo", { name, request });
}

// --- Work ---

export interface CreateWorkRequest {
  title: string;
  summary?: RichContent;
}

export interface Work {
  title: string;
  summary?: RichContent;
}

export async function getAllWorks(): Promise<Work[]> {
  return await invoke("get_all_works");
}

export async function getWork(name: string): Promise<Work | null> {
  return await invoke("get_work", { name });
}

export async function createWork(request: CreateWorkRequest): Promise<string> {
  return await invoke("create_work", { request });
}

export async function updateWork(name: string, request: CreateWorkRequest): Promise<string> {
  return await invoke("update_work", { name, request });
}

// --- SchoolOfThought ---

export interface CreateSchoolOfThoughtRequest {
  name: string;
  description?: RichContent;
}

export interface SchoolOfThought {
  name: string;
  description?: RichContent;
}

export async function getAllSchoolsOfThought(): Promise<SchoolOfThought[]> {
  return await invoke("get_all_schools_of_thought");
}

export async function getSchoolOfThought(name: string): Promise<SchoolOfThought | null> {
  return await invoke("get_school_of_thought", { name });
}

export async function createSchoolOfThought(request: CreateSchoolOfThoughtRequest): Promise<string> {
  return await invoke("create_school_of_thought", { request });
}

export async function updateSchoolOfThought(name: string, request: CreateSchoolOfThoughtRequest): Promise<string> {
  return await invoke("update_school_of_thought", { name, request });
}

// --- Search ---

export interface SearchResult {
  entity_type: string;
  name: string;
}

export async function searchEntities(query: string): Promise<SearchResult[]> {
  return await invoke("search_entities", { query });
}

// --- Delete ---

export async function deleteEntity(entityType: string, name: string): Promise<string> {
  return await invoke("delete_entity", { entity_type: entityType, name });
}
