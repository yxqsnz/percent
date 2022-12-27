"use client";

import { FormEvent, useEffect, useState } from "react";
import { ValidationState } from "./Validations";

interface BindInput {
  value: string;
  onChange: (event: FormEvent<HTMLInputElement>) => void;
}

export function bindInput(
  validationState: ValidationState,
  validate: (s: string, state: ValidationState) => void
): BindInput {
  let [val, set] = useState("");

  const callback = (event: FormEvent<HTMLInputElement>) => {
    validate(event.currentTarget.value, validationState);
    set(event.currentTarget.value);
  };

  return {
    value: val,
    onChange: callback,
  };
}
