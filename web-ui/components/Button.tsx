import { ButtonHTMLAttributes } from "react";

export default function CustomButton({
  children,
  className,
  ...props
}: ButtonHTMLAttributes<HTMLButtonElement>) {
  return (
    <button
      className={`bg-emerald-200 border-slate-700 border-solid border-2 rounded-lg p-2  ${className}`}
      {...props}
    >
      {children}
    </button>
  );
}
