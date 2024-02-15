"use client";
import EntryEditor from "@/src/components/entryEditor";
import dataInterface from "@/src/lib/dataInterface";
import { useSearchParams } from "next/navigation";

export default function Editor() {
  const data = JSON.parse(useSearchParams().get("data") || "{}");
  return (
    <>
      <EntryEditor song={data} />
    </>
  );
}
