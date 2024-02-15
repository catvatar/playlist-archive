import EditPlaylist from "@/src/components/editPlaylist";

export default function PlaylistEditor() {
  const exampleData = {
    name: "Playlist 1",
    songs: [{ title: "Song 1" }, { title: "Song 2" }],
  };
  return (
    <div className="p-4 bg-slate-100 rounded-md text-slate-950">
      <h1 className="p-2">Playlist Editor</h1>
      <EditPlaylist playlist={exampleData} />
    </div>
  );
}
