/* eslint-disable @next/next/no-head-element */
import "../styles/globals.css";
import { Inter } from "@next/font/google";
import GlobalNavbar from "../components/GlobalNavbar";
const inter = Inter({ subsets: ["latin"] });

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html>
      <head></head>
      <body>
        <div
          className={`bg-slate-900 transition-all duration-300 ${inter.className}`}
        >
          <GlobalNavbar />
          <div className="h-screen flex flex-col items-center justify-center">
            {children}
          </div>
        </div>
      </body>
    </html>
  );
}
