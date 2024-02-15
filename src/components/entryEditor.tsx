"use client";
import React, { useState, useEffect } from "react";
import { writeTextFile } from "@tauri-apps/api/fs";
import { appDataDir } from "@tauri-apps/api/path";
import sanitize from "sanitize-filename";

export default function EntryEditor({ song }: any) {
  const fields = [
    "title",
    "artist",
    "album",
    "year",
    "genre",
    "tags",
    "sources",
  ];
  console.log(song);
  return (
    <div className="p-4 border-2 rounded-md bg-slate-200 w-full">
      <h1 className="text-2xl font-bold mb-4 text-slate-900">Add new entry</h1>
      <FieldList fields={fields} data={song} />
    </div>
  );
}

function FieldList({ fields, data }: { fields: string[]; data: any }) {
  const [fieldValues, setFieldValues] = useState(
    fields.map((field) => ({
      name: field,
      value: data[field] || "",
    }))
  );

  useEffect(() => {
    setFieldValues(
      fields.map((field) => ({
        name: field,
        value: data[field] || "",
      }))
    );
  }, [data]);

  const handleInputChange = (
    index: number,
    event: React.ChangeEvent<HTMLInputElement>
  ) => {
    const newFieldValues = [...fieldValues];
    newFieldValues[index].value = event.target.value;
    setFieldValues(newFieldValues);
  };

  const onSubmit = () => {
    const data = fieldValues.reduce((acc: any, field) => {
      acc[field.name] = field.value;
      return acc;
    }, {});
    if (data.title === "") return console.log("Title is required");
    saveData(data);
  };

  return (
    <>
      <ul className="space-y-4 p-4">
        {fieldValues.map((field, index) => (
          <li className="flex flex-col" key={index}>
            <label className="block text-gray-700">
              <h2>{capitalizeFirstLetter(field.name)}</h2>
            </label>
            <input
              type="text"
              value={field.value}
              onChange={(event) => handleInputChange(index, event)}
              className="border flex-1 text-slate-900 border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            />
          </li>
        ))}
      </ul>
      <button
        className="text-slate-900 bg-slate-50 p-2 border-2 border-slate-900 w-40 rounded-md"
        onClick={onSubmit}
      >
        Save
      </button>
    </>
  );
}

async function saveData(data: any) {
  const path = await appDataDir().then(
    (dir) => dir + sanitize(data.title) + ".txt"
  );
  const success = await writeTextFile(path, JSON.stringify(data)).then(
    (e) => e
  );
  return success;
}

function capitalizeFirstLetter(string: string) {
  return string.charAt(0).toUpperCase() + string.slice(1).toLowerCase();
}
