"use client";
import loadMedia from "@/src/lib/loadMedia";
import { useState } from "react";
import { JsonView, collapseAllNested, darkStyles } from "react-json-view-lite";
import "react-json-view-lite/dist/index.css";

export default function About() {
  const [data, setData] = useState({});
  const [url, setUrl] = useState("");

  return (
    <>
      <h1 className="h-full">About</h1>
      <p>
        A utility app for storing and editing playlist as local plaintext files.
      </p>
      <form
        className="py-4"
        onSubmit={(e) => {
          e.preventDefault();
          loadMedia(url).then((data) => setData(data));
        }}
      >
        <input
          className="text-slate-900 w-full"
          type="text"
          onChange={(e) => setUrl(e.target.value)}
        />
        <button
          className="bg-slate-900 text-white px-4 py-2 rounded-lg"
          type="submit"
        >
          Submit
        </button>
      </form>
      <JsonView
        data={data}
        shouldExpandNode={collapseAllNested}
        style={darkStyles}
      />
    </>
  );
}
