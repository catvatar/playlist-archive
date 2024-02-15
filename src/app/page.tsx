"use client";
import PlaylistTreeView from "@/src/components/playlistEditor";

export default function Home() {
  const exampleData: any = [
    { name: "Playlist 1", songs: [{ name: "Pl-1 S-1" }, { name: "Pl-1 S-2" }] },
    { name: "Playlist 2", songs: [{ name: "Pl-2 S-1" }] },
    {
      name: "Playlist 3",
      songs: [{ name: "Pl-3 S-1" }, { name: "Pl-3 S-2" }, { name: "Pl-3 S-3" }],
    },
    { name: "Playlist 4", songs: [{ name: "Pl-4 S-1" }, { name: "Pl-4 S-2" }] },
  ];
  return (
    <main className="flex min-h-screen flex-col items-center gap-8 p-8">
      <h1>Playlist Archiver</h1>
      <h2>Playlist Editor</h2>
      <PlaylistTreeView data={exampleData} />
    </main>
  );
}
