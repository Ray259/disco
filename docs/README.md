# Disco Elysium Encyclopedia - Engineering Documentation

> **System Version**: 1.0.0 (Alpha)
> **Last Updated**: Feb 2026

This documentation covers the internal architecture, design decisions, and exhaustive implementation details of the Encyclopedia application.

## Core Architecture
*How the system is designed at a high level.*

- **[Architecture & Design](./architecture.md)**: IPC Bridge, CQRS Pattern, Module Organization.
- **[Database Strategy](./database.md)**: Hybrid Relational/Document Model details.
- **[Domain Model](./domain_model.md)**: Exact definitions of all 6 Entity Types.
- **[Rich Content Protocol](./protocol_rich_content.md)**: Specification for the text/link system.

## Code Reference (The "Why" and "How")
*File-by-file explanations of the implementation.*

- **[Backend API Reference](./backend_api_reference.md)**:
    - Lists every Command (`create_figure`, `get_institution`, etc.).
    - Explains internal logic, validation rules, and error handling.
    - Covers Value Objects (`Zeitgeist`, `DateRange`).

- **[Frontend Component Reference](./frontend_component_reference.md)**:
    - Lists every Component (`EntityCreate`, `FigureForm`, etc.).
    - Details State management, Props, and Render logic.
    - Explains the Form Controller pattern.

## Workflows
- **[Frontend Architecture](./frontend.md)**: General patterns for React/Vite/Tailwind.
- **[Roadmap](./roadmap.md)**: Upcoming features (Relation Manager).
