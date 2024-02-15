"use client";
import { useState } from "react";
import EntryEditor from "./entryEditor";

export default function EditPlaylist({ playlist }: any) {
  const [editing, setEditing] = useState("");
  return (
    <>
      <h2>{playlist.name}</h2>
      <ul>
        {playlist.songs?.map((song: any) => (
          <li key={song.title}>
            <div className="flex items-baseline gap-2 p-2 border-1 border-slate-600 rounded-sm">
              <h2>{"- " + song.title}</h2>
              <button
                className="p-1 px-2 border-2 border-slate-600 rounded-md"
                onClick={() => {
                  setEditing(song.title);
                }}
              >
                Edit
              </button>
              <button className="p-1 px-2 border-2 border-slate-600 rounded-md">
                Delete
              </button>
            </div>
            {editing === song.title ? <EntryEditor song={song} /> : null}
          </li>
        ))}
      </ul>
    </>
  );
}
