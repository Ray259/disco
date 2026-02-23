import { useReducer, useCallback } from "react";

type FormAction<T> =
  | { type: "SET_FIELD"; field: keyof T; value: T[keyof T] }
  | { type: "RESET"; state: T };

function formReducer<T>(state: T, action: FormAction<T>): T {
  switch (action.type) {
    case "SET_FIELD":
      return { ...state, [action.field]: action.value };
    case "RESET":
      return action.state;
    default:
      return state;
  }
}

/**
 * Generic form state hook backed by useReducer.
 *
 * Usage:
 *   const [form, setField, resetForm] = useFormState({ name: "", age: 0 });
 *   setField("name", "Alex");
 *   resetForm({ name: "Jack", age: 42 });
 */
export function useFormState<T extends Record<string, any>>(initialState: T) {
  const [state, dispatch] = useReducer(formReducer<T>, initialState);

  const setField = useCallback(
    <K extends keyof T>(field: K, value: T[K]) => {
      dispatch({ type: "SET_FIELD", field, value: value as T[keyof T] });
    },
    []
  );

  const resetForm = useCallback(
    (newState: T) => {
      dispatch({ type: "RESET", state: newState });
    },
    []
  );

  return [state, setField, resetForm] as const;
}
