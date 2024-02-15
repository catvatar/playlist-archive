"use client";
import { useEffect, useState } from "react";
import InputComponent from "../../components/inputComponent";
import loadMedia from "@/src/lib/loadMedia";
import { useRouter } from "next/navigation";

export default function LoadSong() {
  const router = useRouter();

  const [err, setErr] = useState("");
  const [input, setInput] = useState({
    URL: "",
    importType: "",
    source: "",
  });

  useEffect(() => {
    if (input.URL === "") return;
    const fetchMedia = async () => {
      const response = await loadMedia(input);
      if (response === "failed") {
        setErr("failed");
        setTimeout(() => {
          setErr("");
        }, 3000);
        return;
      }
      router.push(
        `/editor?data=${JSON.stringify(response)
          .replaceAll("#", "%23")
          .replaceAll("&", "%26")}`
      );
    };
    fetchMedia();
  }, [input]);

  return (
    <>
      <div className="p-2">
        <InputComponent setInput={setInput} />
      </div>
      {err === "failed" ? (
        <div className="p-2">
          <div className="bg-red-500 p-2 rounded-md text-white">
            Failed to load song
          </div>
        </div>
      ) : (
        <></>
      )}
    </>
  );
}
