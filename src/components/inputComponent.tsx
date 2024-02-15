"use client";
import { useState } from "react";

export default function InputComponent({ setInput }: any) {
  const [importType, setImportType] = useState("song");
  const [link, setLink] = useState({ URL: "", source: "" });

  return (
    <div className="flex flex-col bg-slate-200 p-4 rounded-md text-slate-950 w-full">
      <h1 className="p-2">
        {"Import "}
        <select
          className="bg-slate-200"
          onChange={(e) => setImportType(e.currentTarget.value)}
        >
          <option value="song">Song</option>
          <option value="playlist">Playlist</option>
        </select>
      </h1>
      <div className="flex gap-2">
        <InputLink setLink={setLink} />
        <button
          onClick={() => {
            setInput({
              URL: link.URL,
              importType: importType,
              source: link.source,
            });
          }}
          className="border-2 border-slate-900 p-2"
        >
          Load
        </button>
      </div>
    </div>
  );
}

function InputLink({ setLink }: any) {
  const [URL, setURL] = useState("");
  const [source, setSource] = useState("youtube");

  return (
    <>
      <input
        className="border-2 border-slate-900 p-2 flex-1"
        onChange={(e) => {
          const newURL = e.currentTarget.value;
          setURL(newURL);
          setLink({ URL: newURL, source: source });
        }}
        placeholder="Media URL"
      />
      <select
        className="border-2 border-slate-900 p-2"
        id="source-select"
        onChange={(e) => {
          const newSource = e.currentTarget.value;
          setSource(newSource);
          setLink({ URL: URL, source: newSource });
        }}
        value={source}
      >
        {/* <option value="auto">Auto</option> */}
        <option value="youtube">YouTube</option>
      </select>
    </>
  );
}
