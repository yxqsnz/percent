"use client";

import {
  FormEvent,
  HTMLProps,
  MutableRefObject,
  useRef,
  useState,
} from "react";
import CustomButton from "../../components/Button";
import { bindInput } from "../../utils/Form";
import {
  validateNickname,
  validatePassword,
  ValidationState,
} from "../../utils/Validations";

function Field({ className: _, ...props }: HTMLProps<HTMLInputElement>) {
  return (
    <input
      className="rounded-lg p-2 bg-slate-800 text-slate-200 border border-slate-700 focus:border-blue-500 focus:ring-blue-500 block w-full outline-0"
      required
      {...props}
    />
  );
}

function Validations({ state }: { state: ValidationState }) {
  if (!state.invalidNickname && !state.invalidPassword) return <></>;

  return (
    <div className="text-red-500">
      <ul className="list-disc">
        {state.invalidNickname && (
          <li> Nickname must be at least 2 and litter than 16 characters</li>
        )}

        {state.invalidPassword && (
          <li> Password must be at least 8 and litter than 72 characters</li>
        )}
      </ul>
    </div>
  );
}

export default function createAccount() {
  let [classes, setClasses] = useState("");
  let [isDisabled, setDisabled] = useState(false);
  let [text, setText] = useState("Submit");
  const errors = new ValidationState(setDisabled, setClasses, setText);

  const nick = bindInput(errors, validateNickname);
  const pw = bindInput(errors, validatePassword);

  const handle = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    if (errors.invalidNickname || errors.invalidPassword) return;

    setClasses("cursor-wait bg-indigo-700 text-slate-200");
    setDisabled(true);
    setText("...");
  };

  return (
    <form onSubmit={handle}>
      <div className="grid place-items-stretch gap-6 m-4 rounded-lg bg-slate-900 p-8 border-solid border-slate-700 border-2">
        <h1 className="text-slate-200 font-medium">
          Account creation
          <Validations state={errors} />
        </h1>

        <Field
          type="text"
          placeholder="Nick"
          minLength={2}
          maxLength={16}
          onChange={nick.onChange}
        />

        <Field
          type="password"
          placeholder="Password"
          minLength={8}
          maxLength={72}
          onChange={pw.onChange}
        />

        <CustomButton type="submit" className={classes} disabled={isDisabled}>
          {text}
        </CustomButton>
      </div>
    </form>
  );
}
