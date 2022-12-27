"use client";

import { Dispatch, SetStateAction, useState } from "react";

export class ValidationState {
  invalidNickname: boolean;
  setInvalidNickname: Dispatch<SetStateAction<boolean>>;
  invalidPassword: boolean;
  setInvalidPassword: Dispatch<SetStateAction<boolean>>;

  setDisabled: Dispatch<SetStateAction<boolean>>;
  setClasses: Dispatch<SetStateAction<string>>;
  setText: Dispatch<SetStateAction<string>>;

  constructor(
    setDisabled: Dispatch<SetStateAction<boolean>>,
    setClasses: Dispatch<SetStateAction<string>>,
    setText: Dispatch<SetStateAction<string>>
  ) {
    this.setDisabled = setDisabled;
    this.setClasses = setClasses;
    this.setText = setText;

    [this.invalidNickname, this.setInvalidNickname] = useState(true);
    [this.invalidPassword, this.setInvalidPassword] = useState(true);
  }
}

export const validateNickname = (s: string, item: ValidationState) => {
  if (s.length >= 2 && s.length <= 16) {
    item.setInvalidNickname(false);
  } else {
    item.setInvalidNickname(true);
  }
};

export const validatePassword = (s: string, item: ValidationState) => {
  if (s.length >= 8 && s.length <= 72) {
    item.setInvalidPassword(false);
  } else {
    item.setInvalidPassword(true);
  }
};
