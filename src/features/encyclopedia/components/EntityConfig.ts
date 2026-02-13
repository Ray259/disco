import { 
    getAllFigures, 
    getAllInstitutions, 
    getAllEvents, 
    getAllGeos, 
    getAllWorks, 
    getAllSchoolsOfThought 
} from "../api";
import { Renderers } from "./EntityRenderers";
import { FigureForm } from "./forms/FigureForm";
import { InstitutionForm } from "./forms/InstitutionForm";
import { EventForm } from "./forms/EventForm";
import { GeoForm } from "./forms/GeoForm";
import { WorkForm } from "./forms/WorkForm";
import { SchoolForm } from "./forms/SchoolForm";

export const ENTITY_CONFIG: Record<string, any> = {
    figures: {
        title: "People",
        fetcher: getAllFigures,
        renderer: Renderers.Figure,
        createType: "Figure",
        formComponent: FigureForm,
        color: "var(--disco-accent-orange)"
    },
    institutions: {
        title: "Institutions",
        fetcher: getAllInstitutions,
        renderer: Renderers.Institution,
        createType: "Institution",
        formComponent: InstitutionForm,
        color: "var(--disco-accent-yellow)"
    },
    events: {
        title: "Historical Events",
        fetcher: getAllEvents,
        renderer: Renderers.Event,
        createType: "Event",
        formComponent: EventForm,
        color: "var(--disco-accent-purple)"
    },
    geos: {
        title: "Geography",
        fetcher: getAllGeos,
        renderer: Renderers.Geo,
        createType: "Geo",
        formComponent: GeoForm,
        color: "var(--disco-accent-teal)"
    },
    works: {
        title: "Bibliography",
        fetcher: getAllWorks,
        renderer: Renderers.Work,
        createType: "Work",
        formComponent: WorkForm,
        color: "#d4d4d8"
    },
    schools: {
        title: "Schools of Thought",
        fetcher: getAllSchoolsOfThought,
        renderer: Renderers.School,
        createType: "School",
        formComponent: SchoolForm,
        color: "#ef4444"
    }
};
