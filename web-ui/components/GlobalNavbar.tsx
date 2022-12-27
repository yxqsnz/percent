import Link from "next/link";
import CustomButton from "./Button";

export default function GlobalNavbar() {
  return (
    <div className="absolute top-0 z-40 backdrop-blur-sm bg-slate-900 w-full">
      <div className="p-3">
        <div className="flex justify-between">
          <p className="bg-emerald-200 border-slate-700 border-2 border-solid rounded-lg p-2 cursor-default">
            Percent
          </p>

          <div className="space-x-4">
            <Link href="/createAccount">
              <CustomButton> Create account </CustomButton>
            </Link>

            <CustomButton> Login </CustomButton>
          </div>
        </div>
      </div>
    </div>
  );
}
