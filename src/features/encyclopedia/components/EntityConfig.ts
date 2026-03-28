import { 
    getAllFigures, 
    getAllInstitutions, 
    getAllEvents, 
    getAllGeos, 
    getAllWorks, 
    getAllSchoolsOfThought,
    getFigure,
    getInstitution,
    getEvent,
    getGeo,
    getWork,
    getSchoolOfThought
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
        title: "Figures of Note",
        fetcher: getAllFigures,
        getById: getFigure,
        renderer: Renderers.Figure,
        createType: "Figure",
        formComponent: FigureForm,
        color: "var(--disco-accent-orange)"
    },
    institutions: {
        title: "Established Institutions",
        fetcher: getAllInstitutions,
        getById: getInstitution,
        renderer: Renderers.Institution,
        createType: "Institution",
        formComponent: InstitutionForm,
        color: "var(--disco-accent-yellow)"
    },
    events: {
        title: "The Historical Record",
        fetcher: getAllEvents,
        getById: getEvent,
        renderer: Renderers.Event,
        createType: "Event",
        formComponent: EventForm,
        color: "var(--disco-accent-purple)"
    },
    geos: {
        title: "Geopolitical Coordinates",
        fetcher: getAllGeos,
        getById: getGeo,
        renderer: Renderers.Geo,
        createType: "Geo",
        formComponent: GeoForm,
        color: "var(--disco-accent-teal)"
    },
    works: {
        title: "Bibliographic Archives",
        fetcher: getAllWorks,
        getById: getWork,
        renderer: Renderers.Work,
        createType: "Work",
        formComponent: WorkForm,
        color: "#d4d4d8"
    },
    schools: {
        title: "Ideological Frameworks",
        fetcher: getAllSchoolsOfThought,
        getById: getSchoolOfThought,
        renderer: Renderers.School,
        createType: "School",
        formComponent: SchoolForm,
        color: "#ef4444"
    }
};
