// sideMenu.tsx
"use client";
import Link from "next/link";
import React, { useState } from "react";

interface SideMenuWrapperProps {
  children: React.ReactNode;
}

export default function SideMenuWrapper({ children }: SideMenuWrapperProps) {
  const [isCollapsed, setIsCollapsed] = useState(false);
  const routs = [
    { name: "Home", path: "/" },
    { name: "Load Song", path: "/load-song" },
    { name: "Playlist Editor", path: "/playlist-editor" },
    { name: "Editor", path: "/editor" },
    { name: "About", path: "/about" },
  ];

  const toggleCollapse = () => {
    setIsCollapsed(!isCollapsed);
  };

  return (
    <div className="flex min-h-screen">
      <div className="toggle-button" onClick={toggleCollapse}>
        <span className="toggle-icon">{isCollapsed ? "▶" : "◀"}</span>
      </div>
      <div
        className={`${isCollapsed ? "hidden" : ""} border-r-2 pt-6 pr-4 pl-2`}
      >
        <ul className="flex flex-col gap-4">
          {routs.map((route) => (
            <li key={route.name}>
              <Link href={route.path}>
                <h1 className="hover:text-slate-300">{route.name}</h1>
              </Link>
            </li>
          ))}
        </ul>
      </div>
      <div className="content flex-1">{children}</div>
    </div>
  );
}
