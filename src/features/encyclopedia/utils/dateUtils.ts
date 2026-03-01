/**
 * Format a historical date string for display.
 * Handles negative / zero years as BCE.
 *
 * Input  : "YYYY-MM-DD" | "YYYY-MM" | "YYYY" | "-YYYY-MM-DD" | etc.
 * Output : e.g. "500 BCE", "44 BCE", "1453", "1453-03-29"
 */
export function formatHistoricalDate(dateStr: string | undefined | null): string {
  if (!dateStr) return "—";

  const trimmed = dateStr.trim();
  if (!trimmed) return "—";

  // Detect negative years: starts with "-"
  const isNegative = trimmed.startsWith("-");
  const withoutSign = isNegative ? trimmed.slice(1) : trimmed;

  // Split into parts: YYYY, MM, DD
  const parts = withoutSign.split("-");
  const yearNum = parseInt(parts[0], 10);

  if (isNaN(yearNum)) return trimmed; // fallback

  if (isNegative || yearNum === 0) {
    // BCE: show absolute year value + "BCE"
    const absYear = isNegative ? yearNum : 1; // year 0 → 1 BCE in astronomical year numbering
    return `${absYear} BCE`;
  }

  // CE positive year — return as-is (could add "CE" but not requested)
  return trimmed;
}
