import "./globals.css";
import type { Metadata } from "next";
import SideMenu from "../components/sideMenu";

export const metadata: Metadata = {
  title: "Create Next App",
  description: "Generated by create next app",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className="bg-slate-900 text-slate-50 select-none">
        <SideMenu>
          <div className="text-center p-8 pt-10">{children}</div>
        </SideMenu>
      </body>
    </html>
  );
}